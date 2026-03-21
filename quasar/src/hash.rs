#[cfg(any(target_os = "solana", target_arch = "bpf"))]
use solana_define_syscall::definitions::sol_sha256;

use sha2_const_stable::Sha256 as ConstSha256;

const fn make_zero_hashes() -> [[u8; 32]; 21] {
    let mut h = [[0u8; 32]; 21];
    h[0] = ConstSha256::new().update(&[0u8; 32]).finalize();
    let mut i = 1usize;
    while i <= 20 {
        let mut buf = [0u8; 64];
        let mut j = 0usize;
        while j < 32 {
            buf[j] = h[i - 1][j];
            buf[32 + j] = h[i - 1][j];
            j += 1;
        }
        h[i] = ConstSha256::new().update(&buf).finalize();
        i += 1;
    }
    h
}

pub const ZERO_HASHES: [[u8; 32]; 21] = make_zero_hashes();

#[inline(always)]
pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    #[cfg(any(target_os = "solana", target_arch = "bpf"))]
    {
        let slices: [&[u8]; 2] = [left.as_ref(), right.as_ref()];
        let mut hash = core::mem::MaybeUninit::<[u8; 32]>::uninit();
        unsafe {
            sol_sha256(
                slices.as_ptr() as *const u8,
                2u64,
                hash.as_mut_ptr() as *mut u8,
            );
            hash.assume_init()
        }
    }

    #[cfg(not(any(target_os = "solana", target_arch = "bpf")))]
    {
        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().into()
    }
}
