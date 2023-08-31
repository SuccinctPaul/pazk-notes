# Zero-Knowledge via Commit-And-Prove and Masking Polynomials

## commit-and-prove
The commit-and-prove argument described above is conceptually related to fully homomorphic encryption (FHE).
An FHE scheme allows for computation over encrypted data. 


The commit-and-prove zero-knowledge argument is the argument.
* commit
To preserve zero-knowledge, the prover wishes to keep the elements of the witness hidden from the verifier. 
So the prover commits to the witness elements using an additively homomorphic commitment scheme (Pedersen commitments)–
these commitments are analogs of the ciphertexts in the FHE scenario above. 

* verifier
The verifier seeks to obtain a commitment to the output of the circuit. 

* The key difference in the commit-and-prove argument is that the commitment scheme is only additively homomorphic rather than fully homomorphic. 
This means that the verifier on its own can “add two committed values” without ever opening the commitments, but cannot multiply them. 
So for every multiplication gate in the circuit, the prover in the commit-and-prove argument helps the verifier compute the multiplication, 
by sending a commitment to the product and proving in zero-knowledge that indeed it can open that commitment to the appropriate product of committed values. 
This is why the proof length grows linearly with the number of multiplication gates in the circuit, but has no dependence on the number of addition gates.

## zk vs masking polynomial
