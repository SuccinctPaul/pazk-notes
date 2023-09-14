
// Represents the minimal parameters that determine a `ConstraintSystem`.
pub struct ConstraintSystem<'l> {
    // witness.
    a: Vec<&'l str>,
    b: Vec<&'l str>,
    c: Vec<&'l str>,
    q_l: Vec<Fr>,
    q_r: Vec<Fr>,
    q_m: Vec<Fr>,
    q_o: Vec<Fr>,
    q_c: Vec<Fr>,
    q_lookup: Vec<Fr>,
    pub pub_gate_position: Vec<usize>,
    pub pub_gate_value: Vec<Fr>,
    pub table: Table,
}
