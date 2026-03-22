use merkle_pinocchio::hash::{ZERO_HASHES, hash_pair};
use merkle_pinocchio::state::MerkleTree;
use mollusk_svm::{Mollusk, program, result::Check};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

const ERR_ZERO_LEAF: u32 = 0;
const ERR_INVALID_PROOF: u32 = 2;

fn setup() -> (Mollusk, Pubkey) {
    let program_id = Pubkey::new_unique();
    let mollusk = Mollusk::new(&program_id, "target/deploy/merkle_pinocchio");
    (mollusk, program_id)
}

fn merkle_pda(authority: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"merkle", authority.as_ref()], program_id)
}

fn ix_initialize(program_id: &Pubkey, authority: &Pubkey, pda: &Pubkey, bump: u8) -> Instruction {
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*authority, true),
            AccountMeta::new(*pda, false),
            AccountMeta::new_readonly(Pubkey::default(), false),
        ],
        data: vec![0, bump],
    }
}

fn ix_insert(program_id: &Pubkey, authority: &Pubkey, pda: &Pubkey, leaf: [u8; 32]) -> Instruction {
    let mut data = vec![1];
    data.extend_from_slice(&leaf);
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*authority, true),
            AccountMeta::new(*pda, false),
        ],
        data,
    }
}

fn ix_verify(
    program_id: &Pubkey,
    pda: &Pubkey,
    leaf: [u8; 32],
    index: u32,
    proof: &[[u8; 32]; 20],
) -> Instruction {
    let mut data = vec![2];
    data.extend_from_slice(&leaf);
    data.extend_from_slice(&index.to_le_bytes());
    for p in proof {
        data.extend_from_slice(p);
    }
    Instruction {
        program_id: *program_id,
        accounts: vec![AccountMeta::new_readonly(*pda, false)],
        data,
    }
}

fn do_initialize(
    mollusk: &Mollusk,
    program_id: &Pubkey,
    authority: &Pubkey,
    pda: &Pubkey,
    bump: u8,
) -> Vec<(Pubkey, Account)> {
    let (sp, sa) = program::keyed_account_for_system_program();
    let result = mollusk.process_and_validate_instruction(
        &ix_initialize(program_id, authority, pda, bump),
        &[
            (*authority, Account::new(10_000_000_000, 0, &sp)),
            (*pda, Account::new(0, 0, &sp)),
            (sp, sa),
        ],
        &[Check::success()],
    );
    result.resulting_accounts
}

fn parse_tree(data: &[u8]) -> (u32, [u8; 32]) {
    let next_index = u32::from_le_bytes(data[36..40].try_into().unwrap());
    let root: [u8; 32] = data[40..72].try_into().unwrap();
    (next_index, root)
}

fn expected_root_after_insert(leaf: [u8; 32]) -> [u8; 32] {
    let mut h = leaf;
    for i in 0..20usize {
        h = hash_pair(&h, &ZERO_HASHES[i]);
    }
    h
}

#[test]
fn test_initialize() {
    let (mollusk, program_id) = setup();
    let (sp, sa) = program::keyed_account_for_system_program();
    let authority = Pubkey::new_unique();
    let (pda, bump) = merkle_pda(&authority, &program_id);

    let result = mollusk.process_and_validate_instruction(
        &ix_initialize(&program_id, &authority, &pda, bump),
        &[
            (authority, Account::new(10_000_000_000, 0, &sp)),
            (pda, Account::new(0, 0, &sp)),
            (sp, sa),
        ],
        &[Check::success()],
    );
    println!("initialize CU: {}", result.compute_units_consumed);

    let data = &result.resulting_accounts[1].1.data;
    assert_eq!(data.len(), MerkleTree::LEN, "wrong account size");
    let (next_index, root) = parse_tree(data);
    assert_eq!(next_index, 0, "next_index must start at 0");
    assert_eq!(root, ZERO_HASHES[20], "empty tree root mismatch");
}

#[test]
fn test_insert_updates_state() {
    let (mollusk, program_id) = setup();
    let authority = Pubkey::new_unique();
    let (pda, bump) = merkle_pda(&authority, &program_id);

    let after_init = do_initialize(&mollusk, &program_id, &authority, &pda, bump);
    let authority_acc = after_init[0].1.clone();
    let pda_acc = after_init[1].1.clone();

    let leaf = [1u8; 32];
    let result = mollusk.process_and_validate_instruction(
        &ix_insert(&program_id, &authority, &pda, leaf),
        &[(authority, authority_acc), (pda, pda_acc)],
        &[Check::success()],
    );
    println!("insert CU: {}", result.compute_units_consumed);

    let data = &result.resulting_accounts[1].1.data;
    let (next_index, root) = parse_tree(data);
    assert_eq!(next_index, 1, "next_index must be 1 after insert");
    assert_eq!(root, expected_root_after_insert(leaf), "root mismatch");
}

#[test]
fn test_insert_and_verify() {
    let (mollusk, program_id) = setup();
    let authority = Pubkey::new_unique();
    let (pda, bump) = merkle_pda(&authority, &program_id);

    let after_init = do_initialize(&mollusk, &program_id, &authority, &pda, bump);
    let authority_acc = after_init[0].1.clone();
    let pda_acc = after_init[1].1.clone();

    let leaf = [0xABu8; 32];
    let after_insert = mollusk.process_and_validate_instruction(
        &ix_insert(&program_id, &authority, &pda, leaf),
        &[(authority, authority_acc), (pda, pda_acc)],
        &[Check::success()],
    );
    let pda_acc = after_insert.resulting_accounts[1].1.clone();

    let proof: [[u8; 32]; 20] = core::array::from_fn(|i| ZERO_HASHES[i]);
    let result = mollusk.process_and_validate_instruction(
        &ix_verify(&program_id, &pda, leaf, 0, &proof),
        &[(pda, pda_acc.clone())],
        &[Check::success()],
    );
    println!("verify CU: {}", result.compute_units_consumed);

    let bad_proof: [[u8; 32]; 20] = [[0xFFu8; 32]; 20];
    mollusk.process_and_validate_instruction(
        &ix_verify(&program_id, &pda, leaf, 0, &bad_proof),
        &[(pda, pda_acc.clone())],
        &[Check::err(solana_sdk::program_error::ProgramError::Custom(
            ERR_INVALID_PROOF,
        ))],
    );

    mollusk.process_and_validate_instruction(
        &ix_verify(&program_id, &pda, [0x01u8; 32], 0, &proof),
        &[(pda, pda_acc.clone())],
        &[Check::err(solana_sdk::program_error::ProgramError::Custom(
            ERR_INVALID_PROOF,
        ))],
    );
}

#[test]
fn test_zero_leaf_rejected() {
    let (mollusk, program_id) = setup();
    let authority = Pubkey::new_unique();
    let (pda, bump) = merkle_pda(&authority, &program_id);

    let after_init = do_initialize(&mollusk, &program_id, &authority, &pda, bump);
    let authority_acc = after_init[0].1.clone();
    let pda_acc = after_init[1].1.clone();

    mollusk.process_and_validate_instruction(
        &ix_insert(&program_id, &authority, &pda, [0u8; 32]),
        &[(authority, authority_acc), (pda, pda_acc)],
        &[Check::err(
            solana_sdk::program_error::ProgramError::InvalidArgument,
        )],
    );
    let _ = ERR_ZERO_LEAF;
}
