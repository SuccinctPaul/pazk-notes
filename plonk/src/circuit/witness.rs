use ff::PrimeField;

pub struct Assigments<F: PrimeField> {
    pub a: Vec<F>,
    pub b: Vec<F>,
    pub c: Vec<F>,
}

pub struct Witness<F: PrimeField> {
    pub a: F,
    pub b: F,
    pub c: F,
}
