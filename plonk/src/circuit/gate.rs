use ff::PrimeField;

pub enum Gate<F: PrimeField> {
    Add,
    Mul,
    Const(F), // The constant op. to constraint c = q_c.

              // Empty, // empty row.
}

// (q_l * a) + (q_r * b) + (q_o * c) + (q_m * a * b) + q_c = 0
// where a,b,c are the left, right and output wires of the gate
#[derive(Debug)]
pub struct PlonkGate<F: PrimeField> {
    pub q_l: F,
    pub q_r: F,
    pub q_o: F,
    pub q_m: F,
    pub q_c: F,
}
impl<F: PrimeField> PlonkGate<F> {
    pub fn new(q_l: F, q_r: F, q_o: F, q_m: F, q_c: F) -> Self {
        Self {
            q_l,
            q_r,
            q_o,
            q_m,
            q_c,
        }
    }
}

impl<F: PrimeField> From<GateType<F>> for PlonkGate<F> {
    fn from(op: &GateType<F>) -> Self {
        match op {
            GateType::Add => Self::new(F::ONE, F::ONE, -F::ONE, F::ZERO, F::ZERO),
            GateType::Mul => Self::new(F::ZERO, F::ZERO, -F::ONE, F::ONE, F::ZERO),
            GateType::Const(c) => Self::new(F::ZERO, F::ZERO, -F::ONE, F::ZERO, c.clone()),
            _ => panic!(),
        };
    }
}

// copy relation is defined with
// Tag specifies the permtated cell. which identified with column_name + index (start from 0)
pub enum CopyTag {
    a(usize),
    b(usize),
    c(usize),
}
