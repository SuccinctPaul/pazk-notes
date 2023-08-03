# Fiat Shamir

In public-coin interactive proof or argument, any coin tossed by the verifier V is visible for the prover P as soon as it's tossed.
These coin tosses are interpreted as “random challenges” sent by V to P. And in a public-coin protocol “random challenges” are the only messages sent from V to P.

The Fiat-Shamir can transform any public-coin protocol I into a non-interactive, publicly verifiable protocol Q. 


## Preliminaries

## Random Oracle Model-ROM
A random function R mapping some domain D to the k-bit range  ${0, 1}^k$ , means: for any input x ∈ D, R chooses its output R(x) uniformly at random
from ${0, 1}^k$ .
Meanwhile, the efficient hash function algorithms (e.g., SHA-3 or BLAKE3) is associated with random functions.

The random oracle model (ROM) is an ideal setting. So, the ROM simply assumes that the prover and verifier have query access to a random function R. 
This means there is an oracle (called a random oracle) such that the prover and verifier can submit any query x to the oracle, and the oracle will return R(x). 
That is, for each query x ∈ D posed to the oracle, the oracle makes an independent random choice to determine R(x) and responds with that value. 
It keeps a record of its responses to make sure that it repeats the same response if x is queried again.


The random oracle assumption is not valid in the real world, as specifying a random function R requires
|D| · κ bits—essentially one must list the value R(x) for every input x ∈ D—which is totally impractical
given that |D| must be huge to ensure cryptographic security levels (e.g., |D| ≥ 2256 or larger). In the real
world, the random oracle is replaced with a concrete hash function like SHA-3, which is succinctly specified
via, e.g., a small circuit or computer program that evaluates the hash function on any input. In principle,
it may be possible for a cheating prover in the real world to exploit access to this succinct representation
to break the security of the protocol, even if the protocol is secure in the random oracle model. However,
protocols that are proven secure in the random oracle model are often considered secure in practice, and
indeed no deployed protocols have been broken in this manner.64