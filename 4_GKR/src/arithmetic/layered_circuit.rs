use crate::arithmetic::layered_circuit::Ops::{ADD, MUL};
use crate::poly::MPolynomial;
use bls12_381::Scalar;
use ff::Field;
use std::collections::HashMap;
use std::env::var;
use std::net::Shutdown::Read;

// Operators. for now, they are add and mul.
// Left and right  input index from layer i+1.
#[derive(Clone, Debug)]
pub enum Ops {
    ADD(usize, usize),
    MUL(usize, usize),
}

#[derive(Clone)]
pub struct Layer {
    pub gates: Vec<Ops>,
    pub var_num: usize, // 2^var_num = gates.len()
}

// Configure Circuit Constraints. We assume circuit is layered one, whose gates have fan-in-2 and fan-out-1.
#[derive(Clone, Default)]
pub struct CircuitConfig {
    pub layers: Vec<Layer>,   // from layer 0 to d-1.
    pub input_var_num: usize, // input_layer_len = 2^input_var_num. input is layer-d
    pub depth: usize,         // layer depth, from 0 to depth. depth -1 = layers.len.
}

impl CircuitConfig {
    pub fn evaluate(&self, inputs: &Vec<Scalar>) -> Vec<Scalar> {
        assert_eq!(self.layers.len(), self.depth - 1);
        let max_n = 1 << self.input_var_num;
        assert_eq!(inputs.len(), max_n);

        // start with layer d(input layer)
        let mut layer_i_plus_1 = inputs.clone();

        // from layer d-1 to layer 0(output layer).
        for i in (0..(self.depth - 1)).rev() {
            let layer_i = self.layers.get(i).expect("Can't capture layer_i");
            let gates_num = 1 << layer_i.var_num;
            let gates = &layer_i.gates;
            assert_eq!(gates.len(), gates_num);

            let mut layer_i_outputs = vec![];

            // iter each gate in layer_i
            for gate in gates {
                let gate_output = match gate {
                    ADD(left, right) => layer_i_plus_1[*left] + layer_i_plus_1[*right],
                    MUL(left, right) => layer_i_plus_1[*left] * layer_i_plus_1[*right],
                };
                layer_i_outputs.push(gate_output);
            }
            assert_eq!(layer_i_outputs.len(), gates_num);

            // prepare for next iter.
            layer_i_plus_1 = layer_i_outputs.clone();
        }

        // after iter, will calculate output.
        layer_i_plus_1.clone()
    }

    pub fn witness_to_poly(&self, inputs: &Vec<Scalar>) -> Vec<MPolynomial> {
        assert_eq!(self.layers.len(), self.depth - 1);
        let max_n = 1 << self.input_var_num;
        assert_eq!(inputs.len(), max_n);

        // At start, the mpoly in result start from layer d to 0.
        // However, we'll inverse it to adopt from layer 0 to d.
        let mut result = vec![];

        // start with layer d(input layer)
        let input_mpoly = MPolynomial::lagrange(self.input_var_num, inputs);
        result.push(input_mpoly);

        let mut layer_i_plus_1 = inputs.clone();

        // from layer d-1 to layer 0(output layer).
        for i in (0..(self.depth - 1)).rev() {
            let layer_i = self.layers.get(i).expect("Can't capture layer_i");
            let gates_num = 1 << layer_i.var_num;
            let gates = &layer_i.gates;
            assert_eq!(gates.len(), gates_num);

            let mut layer_i_outputs = vec![];

            // iter each gate in layer_i
            for gate in gates {
                let gate_output = match gate {
                    ADD(left, right) => layer_i_plus_1[*left] + layer_i_plus_1[*right],
                    MUL(left, right) => layer_i_plus_1[*left] * layer_i_plus_1[*right],
                };
                layer_i_outputs.push(gate_output);
            }
            assert_eq!(layer_i_outputs.len(), gates_num);

            let layer_i_mpoly = MPolynomial::lagrange(layer_i.var_num, &layer_i_outputs);
            result.push(layer_i_mpoly);

            // prepare for next iter.
            layer_i_plus_1 = layer_i_outputs.clone();
        }

        assert_eq!(result.len(), self.depth);
        // after iter, will calculate output.
        result.reverse();
        result
    }

    // Obtain the addi and multi mpoly from the circuit.
    // eg:  mult0 is the function defined over domain {0,1}×{0,1}2 ×{0,1}2 as follows. mult0 evaluates
    //      to 1 on the following two inputs: (0,(0,0),(0,1)) and (1,(1,0),(1,1)). On all other inputs,
    //      mult0 evaluates to zero.
    pub(crate) fn ops_to_mpoly(&self) -> Vec<(MPolynomial, MPolynomial)> {
        // result ares vector of (addi_mpoly, multi_mpoly).
        let mut result = Vec::with_capacity(self.depth - 1);

        // from layer 0(output layer) to layer d-1.
        for i in 0..self.depth - 1 {
            let layer_i = self.layers.get(i).expect("Can't capture layer_i");
            let var_num_i = layer_i.var_num;
            let var_num_i_plus_1 = if i == self.depth - 2 {
                // layer d-1's input is input layer.
                self.input_var_num
            } else {
                let layer_i_plus_1 = self
                    .layers
                    .get(i + 1)
                    .expect("Can't capture layer_i_plus_1");
                layer_i_plus_1.var_num
            };
            let n_i_plus_1 = 1 << var_num_i_plus_1;

            let mpoly_var_num = var_num_i + 2 * var_num_i_plus_1;
            let mpoly_size = 1 << mpoly_var_num;
            let mut addi_mpoly_eval = vec![Scalar::zero(); mpoly_size];
            let mut multi_mpoly_eval = vec![Scalar::zero(); mpoly_size];

            // iter each gate in layer_i
            for (j, gate) in layer_i.gates.iter().enumerate() {
                match gate {
                    ADD(left, right) => {
                        assert!(n_i_plus_1 > *left && n_i_plus_1 > *right);
                        // turn binary index (0,(0,1),(1,1)) into vec array.
                        let mpoly_index =
                            (j << 2 * var_num_i_plus_1) + (left << var_num_i_plus_1) + right;
                        addi_mpoly_eval[mpoly_index] = Scalar::one();
                    }
                    MUL(left, right) => {
                        assert!(n_i_plus_1 > *left && n_i_plus_1 > *right);
                        let mpoly_index =
                            (j << 2 * var_num_i_plus_1) + (left << var_num_i_plus_1) + right;
                        multi_mpoly_eval[mpoly_index] = Scalar::one();
                    }
                };
            }

            let addi_mpoly = MPolynomial::lagrange(mpoly_var_num, &addi_mpoly_eval);
            let multi_mpoly = MPolynomial::lagrange(mpoly_var_num, &multi_mpoly_eval);
            result.push((addi_mpoly, multi_mpoly));
        }
        result
    }
}

// impl Into<StructCircuits> for Circuit {
//     fn into(self) -> StructCircuits {}
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::arithmetic::layered_circuit::Ops::MUL;
    use crate::utils::convert_from_binary;
    use ff::PrimeField;

    // sample from Figure 4.12.
    fn simple_circuit() -> CircuitConfig {
        let input_var_num: usize = 2;

        let layer_1 = Layer {
            gates: vec![MUL(0, 0), MUL(1, 1), MUL(1, 2), MUL(3, 3)],
            var_num: 2,
        };

        let output_layer = Layer {
            gates: vec![MUL(0, 1), MUL(2, 3)],
            var_num: 1,
        };

        // Return layer
        CircuitConfig {
            layers: vec![output_layer, layer_1],
            input_var_num,
            depth: 3,
        }
    }

    #[test]
    fn test_evaluation() {
        let inputs = vec![
            Scalar::one(),
            Scalar::from_u128(2),
            Scalar::one(),
            Scalar::from_u128(4),
        ];

        let circuit = simple_circuit();
        // evaluate the circuit with input
        let actual = circuit.evaluate(&inputs);

        let expected = vec![Scalar::from_u128(4), Scalar::from_u128(32)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_witness_to_poly() {
        let inputs = vec![
            Scalar::one(),
            Scalar::from_u128(2),
            Scalar::one(),
            Scalar::from_u128(4),
        ];

        let circuit = simple_circuit();
        let actual = circuit.witness_to_poly(&inputs);

        // Expect
        let input_mpoly = MPolynomial::lagrange(2, &inputs);
        // layer_1 values: [1, 4, 2, 16]
        let layer_1_mpoly = MPolynomial::lagrange(
            2,
            &vec![
                Scalar::one(),
                Scalar::from_u128(4),
                Scalar::from_u128(2),
                Scalar::from_u128(16),
            ],
        );
        // layer_0 values: [4, 32]
        let layer_0_mpoly =
            MPolynomial::lagrange(1, &vec![Scalar::from_u128(4), Scalar::from_u128(32)]);

        let expected = vec![layer_0_mpoly, layer_1_mpoly, input_mpoly];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_op_to_mpoly() {
        let circuit = simple_circuit();
        let actual = circuit.ops_to_mpoly();

        // layer 1, add and mult mpoly
        let var_num_1 = 2 + 2 * 2;
        let add_1_evals = vec![Scalar::zero(); 1 << var_num_1];
        let mut mult_1_evals = vec![Scalar::zero(); 1 << var_num_1];
        // gates: vec![MUL(0, 0), MUL(1, 1), MUL(1, 2), MUL(3, 3)],
        mult_1_evals[0] = Scalar::one(); // (0, (0,0))
        mult_1_evals[(1 << 4) + (1 << 2) + 1] = Scalar::one(); // (1, (1,1))
        mult_1_evals[(2 << 4) + (1 << 2) + 2] = Scalar::one(); // (2, (1,2))
        mult_1_evals[(3 << 4) + (3 << 2) + 3] = Scalar::one(); // (3, (3,3))

        let mpoly_1 = (
            MPolynomial::lagrange(var_num_1, &add_1_evals),
            MPolynomial::lagrange(var_num_1, &mult_1_evals),
        );
        // test MPolynomial::lagrange.
        assert_eq!(mpoly_1.1.evaluate(&vec![0, 1, 0, 1, 0, 1]), Scalar::one());
        assert_eq!(mpoly_1.1.evaluate(&vec![1, 0, 0, 1, 1, 0]), Scalar::one());
        assert_eq!(mpoly_1.1.evaluate(&vec![1, 1, 1, 1, 1, 1]), Scalar::one());
        assert_eq!(mpoly_1.1.evaluate(&vec![1, 1, 1, 0, 1, 1]), Scalar::zero());

        // layer 1, add and mult mpoly
        let var_num_0 = 1 + 2 * 2;
        let add_0_evals = vec![Scalar::zero(); 1 << var_num_0];
        let mut mult_0_evals = vec![Scalar::zero(); 1 << var_num_0];
        // gates: vec![MUL(0, 1), MUL(2, 3)],
        mult_0_evals[1] = Scalar::one(); // (0, (0,1))
        mult_0_evals[(1 << 4) + (2 << 2) + 3] = Scalar::one(); // (1, (2,3))
        let mpoly_0 = (
            MPolynomial::lagrange(var_num_0, &add_0_evals),
            MPolynomial::lagrange(var_num_0, &mult_0_evals),
        );

        let expected = vec![mpoly_0, mpoly_1];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_var_num_len() {
        // Assume all var_num =2, we have three number, the combination is (10, 01, 11)
        let a = 2;
        let b = 1;
        let c = 3;

        println!("{:?}", a << 4);
        println!("{:?}", b << 2);
        // assert_eq!(convert_from_binary(&vec![1,0,0,1,1,1]), a<<4+ b<<2 + c);// meet error
        assert_eq!(
            convert_from_binary(&vec![1, 0, 0, 1, 1, 1]),
            (a << 4) + (b << 2) + c
        );
    }
}
