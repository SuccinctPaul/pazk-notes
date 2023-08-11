use rand_core::{OsRng, RngCore};
use std::env::var;

pub struct Verifier {}

impl Verifier {
    // generate r1, r2, ..., rv,  $r_i ∈ F^{k_i}$
    fn gen_challenge(var_num: usize) -> Vec<u32> {
        (0..var_num)
            .map(|_| OsRng.next_u32() % 1000)
            .collect::<Vec<_>>()
    }

    // picks a random r0∈Fk0 and lets m0←D ̃(r0).
    fn round_i() {}

    // V checks  m_d = W_d (r_d )
    fn check() {}
}
