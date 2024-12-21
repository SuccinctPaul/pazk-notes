//! The plonkish prove system is consisted of:
//! 1. constrains system: arithmetic and permutation constraints
//! 2. custome gate
//! 3. lookup gate(todo)

use crate::pcs::kzg::KZGProof;
use pairing::Engine;

mod prover;
mod verifier;

#[derive(Debug, PartialEq, Default)]
pub struct PlonkProof<E: Engine> {
    // wire poly commitment
    cm_a: E::G1,
    cm_b: E::G1,
    cm_c: E::G1,
    // permutation poly commitment
    cm_z: E::G1,

    // $a(s)$
    pub a_s: E::G1,
    // $b(s)$
    pub b_s: E::G1,
    // $c(s)$
    pub c_s: E::G1,
    // $z(s)$
    pub z_s: E::G1,
    // $t_lo(s)$
    pub t_lo_s: E::G1,
    // $t_mid(s)$
    pub t_mid_s: E::G1,
    // $t_hi(s)$
    pub t_hi_s: E::G1,
    // $w_{\mathfrak{Z}}(s)$
    pub w_z_s: E::G1,
    // $w_{\mathfrak{Z}\omega}(s)$
    pub w_z_omega_s: E::G1,

    // evaluations
    // $\bar a$
    pub a_z: E::G1,
    // $\bar b$
    pub b_z: E::G1,
    // $\bar c$
    pub c_z: E::G1,
    // $\overline {s_{\sigma_1}}$
    pub s_sigma_1_z: E::G1,
    // $\overline {s_{\sigma_1}}$
    pub s_sigma_2_z: E::G1,
    // $\overline r$
    // pub r_z: E::G1,
    // see $\overline {z_\omega}$
    pub z_omega_z: E::G1,

    kzg_proof: KZGProof<E>,
}
