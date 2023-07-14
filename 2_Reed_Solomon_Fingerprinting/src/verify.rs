use crate::utils::read_from_file;
use crate::Person;
use ff::Field;

trait Verifier<F: Field> {
    fn verify(&self, r: F, target_v: F) -> bool;
}

// impl<F: Field> Verifier<F> for Person<F> {
//     fn verify(&self, r: F, target_v: F) -> bool {
//         // hash
//         let real_v = self
//             .data
//             .iter()
//             .enumerate()
//             .map(|i, a| {
//                 let r_i = r.pow_vartime(i);
//                 r_i.mul(a)
//             })
//             .sum();
//
//         // compare
//         target_v == real_v
//     }
// }
