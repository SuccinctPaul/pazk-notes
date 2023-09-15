use crate::kzg::KZGProof;
use pairing::Engine;

mod prover;
mod verifier;

#[derive(Debug, PartialEq)]
pub struct PlonkProof<E: Engine> {
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
    // $\bar a$
    pub a_z: E::HF,
    // $\bar b$
    pub b_z: E::HF,
    // $\bar c$
    pub c_z: E::HF,
    // $\overline {s_{\sigma_1}}$
    pub s_sigma_1_z: E::HF,
    // $\overline {s_{\sigma_1}}$
    pub s_sigma_2_z: E::HF,
    // $\overline r$
    pub r_z: E::HF,
    // see $\overline {z_\omega}$
    pub z_omega_z: E::HF,

    kzg_proof: KZGProof<E>,
}
