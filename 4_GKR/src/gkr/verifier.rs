use rand_core::{OsRng, RngCore};

pub struct Verifier {}

impl Verifier {
    // generate r1, r2, ..., rv
    fn gen_challenge() -> usize {
        let k = OsRng.next_u32() % 1000;
        k as usize
    }

    // picks a random r0∈Fk0 and lets m0←D ̃(r0).
    fn round_i() {}

    // V checks  m_d = W_d (r_d )
    fn check() {}
}
