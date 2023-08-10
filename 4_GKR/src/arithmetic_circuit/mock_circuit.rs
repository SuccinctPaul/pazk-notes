use crate::arithmetic_circuit::mock_circuit::Ops::{ADD, MUL};
use crate::structed_circuit::StructCircuits;
use bls12_381::Scalar;
use ff::Field;
use std::collections::HashMap;

// Operators. for now, they are add and mul.
// Left and right  input index from layer i+1.
#[derive(Clone)]
pub enum Ops {
    ADD(usize, usize),
    MUL(usize, usize),
}

#[derive(Clone)]
pub struct Layer {
    pub gates: Vec<Ops>,
    pub var_num: usize, // 2^var_num = gates.len()
}

#[derive(Clone, Default)]
pub struct Circuit {
    pub layers: Vec<Layer>,
    // from layer 0 to d-1.
    pub input_var_num: usize,
    // input_layer_len = 2^input_var_num. input is layer-d
    pub depth: usize, // layer depth, from 0 to depth
}

impl Circuit {
    pub fn evaluate(&self, inputs: &Vec<Scalar>) -> Vec<Scalar> {
        let max_n = 1 << self.input_var_num;
        assert_eq!(inputs.len(), max_n);

        // start with layer d(input layer)
        let mut layer_i_plus_1_values = inputs;

        // from layer d-1 to layer 0(output layer).
        for i in (0..self.depth).rev() {
            let layer_i = self.layers.get(i).expect("Can't capture gate_i");
            let gates_num = 1 << layer_i.var_num;
            let gates = layer_i.gates;
            assert_eq!(gates.len(), gates_num);

            let mut layer_i_outputs = vec![];

            // iter each gate in layer_i
            for gate in gates {
                let gate_output = match gate {
                    ADD(left, right) => layer_i_plus_1_values[left] + layer_i_plus_1_values[right],
                    MUL(left, right) => layer_i_plus_1_values[left] * layer_i_plus_1_values[right],
                };
                layer_i_outputs.push(gate_output);
            }
            assert_eq!(layer_i_outputs.len(), gates_num);

            // prepare for next iter.
            layer_i_plus_1_values = layer_i_plus_1_values;
        }

        // after iter, will calculate output.
        layer_i_plus_1_values.clone()
    }
}

// impl Into<StructCircuits> for Circuit {
//     fn into(self) -> StructCircuits {}
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::arithmetic_circuit::mock_circuit::Ops::MUL;
    use ff::PrimeField;

    // sample from Figure 4.12.
    fn simple_circuit() -> Circuit {
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
        Circuit {
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
        let output = circuit.evaluate(&inputs);

        let expected = vec![Scalar::from_u128(4), Scalar::from_u128(2)];
        assert_eq!(expected, output);
    }
}
