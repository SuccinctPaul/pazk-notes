# PCPs and Succinct Arguments
* MIP vs PCP
In an MIP, if a prover is asked multiple questions by the verifier, then the prover can behave adaptively,
which means that the prover’s responses to any question can depend on the earlier questions asked by the
verifier. This adaptivity was potentially bad for soundness.

In contrast, Probabilistically Checkable Proofs (PCPs) have non-adaptivity baked directly into the defi-
nition, by considering a verifier V who is given oracle access to a static proof string π. Since π is static, V
can ask several queries to π, and π’s response to any query qi can depend only on qi , and not on q j for j ̸= i.


## Succinct Argument
In order to turn a PCP into a succinct argument, we used a Merkle tree, and did not need to
use a low-degree test.

The idea is the following. The argument system consists of two phases: commit and reveal. 
* In the commit phase, the prover writes down the PCP π, but doesn’t send it to the verifier. Instead, the prover
builds a Merkle tree, with the symbols of the PCP as the leaves, and sends the root hash of the tree to the
verifier. This binds the prover to the string π. 
* In the reveal phase, the argument system verifier simulates the
PCP verifier to determine which symbols of π need to be examined (call the locations that the PCP verifier
queries q1 , . . . , qk ). The verifier sends q1 , . . . , qk to the prover to P, and the prover sends back the answers
π(q1 ), . . . , π(qk ), along with their authentication paths.


## MIP -> PCP
9.3

## PCP for Circuit-SAT
Step 1: Reduce to checking that a polynomial vanishes on a designated subspace

Step 2: Reduce to Checking that a Related Polynomial is Low-Degree

Step 3: A PCP for Reed-Solomon Testing