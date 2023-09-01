use ff::PrimeField;
use group::Group;
use pairing::Engine;
use rand_core::OsRng;
use std::env::consts::OS;

// KeyGen is a randomized algorithm that generates a commitment key `ck` and verification key `vk`
fn setup<E: Engine>() {
    let mut rng = OsRng;

    // specify the generator of subgroup. g,h.
    let g = E::G1::random(rng);
    let h = E::G1::random(rng);

    // generate ck/vk
}
