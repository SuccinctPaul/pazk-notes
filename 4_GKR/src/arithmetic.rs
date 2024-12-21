// For each layer:
// Let Si denote the number of gates at layer i of the constraint_system C. Number the gates at layer i from 0 to Si − 1.
// Assume Si is a power of 2 and let $Si = 2^{k_i} $.
//
// Witness
// Wi : {0, 1}ki → F denote the function t􏰌hat takes as input a binary gate label,
//      and outputs the corresponding gate’s value at layer i
// \widetilde{W_i}:  multilinear extension(MLE) of Wi
// NOTE: Wi depend on input x to C.
//
//
// Constraints
// wiring predicate: that encodes which pairs of wires from layeri+1 are connected to a given gate at layeri in C.
//  Let in_{1,i},in_{2,i}:{0,1}ki →{0,1}ki+1 denote the functions that take as input the label a of a gate at layer i of C,
//      and respectively output the label of the first and second in-neighbor of gate a.
//  eg: if gate a at layer i computes the sum of gates b and c at layer i + 1, then in_{1,i}(a) = b and in_{2,i}(a) = c.
//
//  Define two functions, addi and multi , mapping {0, 1}^{ki +2ki+1} to {0, 1}, which together constitute the wiring predicate of layer i of C.
//      These functions take as input three gate labels (a,b,c), and return 1 if and only if (b,c) = (in1,i(a),in2,i(a))
//      and gate a is an addition (respectively, multiplication) gate.
//
//      Let \widetilde{add_i} and \widetilde{mult_i} denote the multilinear extensions of addi and multi.
//
//  NOTE: wiring predicate(addi, multi) depend only on the constraint_system C and not on the input x to C

pub mod layered_circuit;

// pub mod r1cs
// pub mod plonk
