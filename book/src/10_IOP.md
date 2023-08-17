# Interactive Oracle Proof


## Definition
IOPs in fact generalize both PCPs and IPs.

* IOP is IP
  An IOP is an IP where, in each round the verifier is not forced to read the prover’s entire message, but rather is given query access
  to it, meaning it can choose to look at any desired symbol of the message at the “cost” of a single query.

### Components of IOP
The IOP can be understood as a combination of two constituent protocols.
* The first is a so-called polynomial IOP(PIOP); this is a variant of the IOP model described shortly.
* The second is a polynomial commitment scheme that is itself instantiated via an IOP. 

    We give two such IOP-based polynomial commitment schemes in this chapter: 
        * one called FRI (aka. Fast Reed-Solomon Interactive Oracle Proof of Proximity) with polylogarithmic proof length,
        * and another implicit in a system called Ligero [AHIV17] with larger proofs but a concretely faster prover.

### PIOP
Polynomial IOP is an interactive proof, except that a subset of the prover’s messages are not read in full by the verifier V.

In a standard IOP, each special message is a string, and the verifier is given query access to individual symbols of the string. 

In a polynomial IOP, each special message i specifies a polynomial hi over a finite field F, with degree at most some specified upper bound di.



## PIOP succinct argument
We can obtain a succinct argument via the following three-step design process: 

* First, design a public-coin polynomial IOP for circuit- or R1CS-satisfiability.
* Obtain a public-coin, interactive succinct argument by replacing each “special” message hi in the
polynomial IOP with a polynomial commitment scheme. 
* Remove interaction via Fiat-Shamir.

In fact, all SNARKs covered in this survey are designed via this recipe, with the lone exception of those based on linear PCPs.


## univariate sum-check protocol
10.3.1 - todo.

* Fact 10.1
used the Lagrange Interapote 

* Proof
When H is a multiplicative subgroup of order n, it follows from Lagrange’s Theorem in group theory
that an = 1 for any a ∈ H. Hence, H is precisely the set of n roots of the polynomial X n − 1, i.e.,
$∏ (X − a) = X n − 1.$

* Lemma 10.2

* univariate sum-check protocol



## FRI
FRI is short for Fast Reed-Solomon Interactive Oracle Proof of Proximity, also abbreviated as "Fast RS-IOPP".

On a high-level, FRI enables proving whether a given function `f:H→F_p`is “close” to a certain polynomial of 
low degree. Hence the term proof of proximity.

Loosely put, the FRI protocol allows for a random set of queries (requests for openings of polynomials at randomly 
selected field elements), used by the verifier to ascertain with high probability, the prover's knowledge of a committed polynomial.

FRI is in fact a Merkle commitment scheme where commitments are roots of Merkle trees, and therefore needs no trusted setup, 
as it only uses hash functions.

### Why FRI protocol is considered fast:
1. Due to its resemblance of the ubiquitous Fast Fourier Transforms (FFTs). 
2. The arithmetic complexity of the prover is strictly linear. 
3. The size of the proof is O(nlog(n)). 
4. The arithmetic complexity of the verifier is strictly logarithmic.

## Ligero and Breakdown
10.5
