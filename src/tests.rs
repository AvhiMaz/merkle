use quasar_svm::{Account, AccountMeta, Instruction, Pubkey, QuasarSvm};

const PROGRAM_ID: Pubkey = Pubkey::new_from_array(crate::ID.to_bytes());

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/merkle.so");
    QuasarSvm::new().with_program(&PROGRAM_ID, elf)
}

fn merkle_tree_pda(authority: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"merkle", authority.as_ref()], &PROGRAM_ID).0
}

fn ix_initialize(authority: Pubkey) -> Instruction {
    let merkle_tree = merkle_tree_pda(&authority);
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(merkle_tree, false),
            AccountMeta::new_readonly(quasar_svm::system_program::ID, false),
        ],
        data: vec![0],
    }
}

fn ix_insert(authority: Pubkey, leaf: [u8; 32]) -> Instruction {
    let merkle_tree = merkle_tree_pda(&authority);
    let mut data = vec![1];
    data.extend_from_slice(&leaf);
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(merkle_tree, false),
        ],
        data,
    }
}

fn ix_verify(merkle_tree: Pubkey, leaf: [u8; 32], index: u32, proof: [u8; 640]) -> Instruction {
    let mut data = vec![2];
    data.extend_from_slice(&leaf);
    data.extend_from_slice(&index.to_le_bytes());
    data.extend_from_slice(&proof);
    Instruction {
        program_id: PROGRAM_ID,
        accounts: vec![AccountMeta::new_readonly(merkle_tree, false)],
        data,
    }
}

fn authority_account(address: Pubkey) -> Account {
    Account {
        address,
        lamports: 10_000_000_000,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    }
}

#[test]
fn test_initialize() {
    let mut svm = setup();
    let authority = Pubkey::new_unique();
    let merkle_tree = merkle_tree_pda(&authority);

    let result = svm.process_instruction(
        &ix_initialize(authority),
        &[
            authority_account(authority),
            Account {
                address: merkle_tree,
                lamports: 0,
                data: vec![],
                owner: quasar_svm::system_program::ID,
                executable: false,
            },
        ],
    );

    result.assert_success();
    println!("initialize CU: {}", result.compute_units_consumed);
}

#[test]
fn test_insert() {
    let mut svm = setup();
    let authority = Pubkey::new_unique();
    let merkle_tree = merkle_tree_pda(&authority);

    let empty_tree = Account {
        address: merkle_tree,
        lamports: 0,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    };
    let init_result = svm.process_instruction(
        &ix_initialize(authority),
        &[authority_account(authority), empty_tree],
    );
    init_result.assert_success();
    let tree_account = init_result
        .account(&merkle_tree)
        .expect("merkle_tree not in result")
        .clone();

    let leaf = [1u8; 32];
    let result = svm.process_instruction(
        &ix_insert(authority, leaf),
        &[authority_account(authority), tree_account],
    );

    result.assert_success();
    println!("insert CU: {}", result.compute_units_consumed);
}

#[test]
fn test_insert_and_verify() {
    let mut svm = setup();
    let authority = Pubkey::new_unique();
    let merkle_tree = merkle_tree_pda(&authority);

    svm.process_instruction(&ix_initialize(authority), &[authority_account(authority)])
        .assert_success();

    let empty_tree = Account {
        address: merkle_tree,
        lamports: 0,
        data: vec![],
        owner: quasar_svm::system_program::ID,
        executable: false,
    };
    let init_result = svm.process_instruction(
        &ix_initialize(authority),
        &[authority_account(authority), empty_tree],
    );
    init_result.assert_success();
    let tree_account = init_result
        .account(&merkle_tree)
        .expect("merkle_tree not in result")
        .clone();

    let leaf = [1u8; 32];
    let insert_result = svm.process_instruction(
        &ix_insert(authority, leaf),
        &[authority_account(authority), tree_account],
    );
    insert_result.assert_success();
    let tree_account = insert_result
        .account(&merkle_tree)
        .expect("merkle_tree not in result")
        .clone();

    use crate::hash::ZERO_HASHES;
    let mut proof = [0u8; 640];
    for i in 0..20 {
        proof[i * 32..(i + 1) * 32].copy_from_slice(&ZERO_HASHES[i]);
    }

    let result = svm.process_instruction(&ix_verify(merkle_tree, leaf, 0, proof), &[tree_account]);

    result.assert_success();
    println!("verify CU: {}", result.compute_units_consumed);
}
