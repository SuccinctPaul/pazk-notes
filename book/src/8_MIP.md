# MIPs and Succinct Arguments
Multi-prover interactive proofs (MIPs) grant the verifier access to more than one untrusted prover, and assume the provers 
cannot tell each other about what challenges they receive from the verifier.

## Definition
A k-prover interactive proof protocol for a language L⊆{0, 1} involves k+1 parties: 
a probabilistic polynomial time verifier, and k provers. 

The verifier exchanges a sequence of messages with each prover; 
each prover’s message is a function of the input and the messages from V that it has seen so far. 
The interaction produces a transcript t = (V(r),P1,...,Pk)(x), where r denotes V’s internal randomness. 
After the transcript t is produced, V decides whether to output accept or reject based on r, t, and x. 

Denote by out(V,x,r,P1,...,Pk) the output of verifier V on input x given prover strategies (P1,...,Pk) and that V’s internal randomness is equal to r.


## MIP for Circuit-SAT
8.2

## R1CS-SAT
R1CS(rank-1 constraint system). 
R1CS instance is specified by three m × n matrices A, B,C with entries from a field F and is satisfiable if and only if there is a vector z ∈ Fn 
with z1 = 1 such that: `(A · z) ◦ (B · z) = C · z` .
Here, `·` denotes matrix-vector product, and `◦` denotes entrywise (a.k.a. Hadamard) product.

The i’th rows, ai , bi , and ci , of the matrices A, B, and C collectively specify a so-called rank-one con-
straint, and satisfy that `⟨ai , z⟩ · ⟨bi , z⟩ = ⟨ci , z⟩`.
Rank-one refers to the fact that each constraint involves one product operation involving (a linear combina-
tion of) elements of z, namely the multiplication of ⟨ai , z⟩ and ⟨bi , z⟩.
