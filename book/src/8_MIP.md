# MIPs and Succinct Arguments
Multi-prover interactive proofs (MIPs) grant the verifier access to more than one untrusted prover, and assume the provers 
cannot tell each other about what challenges they receive from the verifier.

## Definitions and Basic Results
Definition
A k-prover interactive proof protocol for a language L⊆{0, 1} involves k+1 parties: a probabilistic polynomial time verifier, 
and k provers. 
The verifier exchanges a sequence of messages with each prover; 
each prover’s message is a function of the input and the messages from V that it has seen so far. 
The interaction produces a transcript t = (V(r),P1,...,Pk)(x), where r denotes V’s internal randomness. 
After the transcript t is produced, V decides whether to output accept or reject based on r, t, and x. 

Denote by out(V,x,r,P1,...,Pk) the output of verifier V on input x given prover strategies (P1,...,Pk) and that V’s internal randomness is equal to r.