// Verify takes as input the commitment, the verification
// key, and a claimed message m′ provided by the committer, and any opening information d and decides
// whether to accept m′ as a valid opening of the commitment.

use pairing::Engine;

pub struct Verifier<E: Engine> {
    vk: E::Fr, // commitment key
}

impl<E: Engine> Verifier<E> {
    // cm = Commit(m, ck)
    fn commit(m: Vec<E::Fr>) {
        // generate a blind value, d.
    }
}
