use ark_ff::Field;
use ark_std::ops::{Add, Mul};
use std::collections::HashMap;

// (ty, g, (u, v))
// g: gate index
// ty: 3, the input layer has to be 3
// u: the left(or the only) input of gate. Note all gates are fan-in 2
// v: the right input of gate.
pub type InputGate<F> = (usize, usize, (F, F));
// (ty, g, (u, v))
// g: gate index
// ty: arithmetic type. 0-add, 1-mul, 2-zero (no inputs), 3-u (only one input. more specific, the input layer has to be 3)
// u: the left(or the only) input index of gate i-1. Note all gates are fan-in 2
// v: the right input index of gate i-1.
pub type Gate = (usize, usize, (usize, usize));

#[derive(Clone)]
pub struct InputLayer<F: Field> {
    pub gates: Vec<InputGate<F>>,
    pub num_gates: usize,
}

#[derive(Clone)]
pub struct Layer {
    pub gates: Vec<Gate>,
    pub num_gates: usize,
}

#[derive(Clone)]
pub struct Circuit<F: Field> {
    pub input_layer: InputLayer<F>,
    pub topology: Vec<Layer>,

    pub num_layers: usize,
}
