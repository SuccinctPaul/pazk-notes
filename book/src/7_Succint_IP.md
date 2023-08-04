# A First Succinct Argument for Circuit Satisfiability, from Interactive Proofs


* Goal
  This transformation is only useful in the context of interactive proofs and arguments if we can design
  efficient proof systems for solving instances of circuit satisfiability.

  In this section, we will see our first example of such an argument system, by combining the GKR protocol with a cryptographic primitive called
  a polynomial commitment scheme.   



## preliminaries
### Circuit-SAT
> arithmetic circuit satisfiability problem (aka. circuit-SAT):

The circuit C takes two inputs, x and w. The first input x is public and fixed. The second input w is often called the witness,
or the non-deterministic/auxiliary input.
Given the first input x and output y, the goal is to determine whether there exists a w such that `C(x, w) = y`.

eg: In RAM case, P claims it knows some witness w(not known to the verifier) which applying C to (x, w), or running M on (x, w), yields output y.

In this section, we will see our first
example of such an argument system, by combining the GKR protocol with a cryptographic primitive called
a polynomial commitment scheme,


### polynomial commitment scheme
> 6.5.2


### Merkle-hashing


## Arguments of Knowledge and SNARKs
Arguments for circuit satisfiability are particularly useful when
they satisfy an enhanced soundness property called knowledge-soundness. Informally, this means that the
prover establishes not only that there exists a witness w such that C(x, w) = y, but in fact that the prover
knows such a w.

Knowledge-soundness can be a meaningful notion even when standard soundness is not. For example,
suppose the prover and verifier agree on a cryptographic hash function h and hash value y, and the prover
claims to know a w such that h(w) = y. The prover can establish this by applying a knowledge-sound

argument for circuit-satisfiability to a circuit C that takes w as input and computes h(w).
An argument satisfying standard soundness, which merely guarantees the existence of a witness w such
that C(w) = y, would be useless in this context. This is because cryptographic hash functions are typically
surjective, meaning for any y there will exist many pre-images w. Accordingly, the trivial proof system
where the verifier always accepts satisfies standard soundness in this context, but not knowledge-soundness.
Knowledge-sound arguments can be particularly useful when they are non-interactive, meaning the proof
is just a static string that is accepted or rejected by the verifier, and succinct, meaning that the proofs are
very short. Such arguments are called SNARKs. In Section 7.4, we explain that the succinct arguments we
give in this chapter are in fact SNARKs.