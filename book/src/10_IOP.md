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


## univariate sumcheck protocol
> Reference: https://eprint.iacr.org/2018/828.pdf

Univariate sumcheck, an RS-encoded IOPP for testing whether a low-degree univariate polynomial(f(x)) sums to zero on a given subspace H ⊆ F.

If f has degree less than d, then f can be uniquely decomposed into polynomials g, h of degrees less
than `|H|` and `d−|H|` (respectively) such that `f ≡ g + ZH · h`, where ZH is the vanishing polynomial of
H. T
  * The vanish poly - Z_H

  * Lagrange Interapote
  Lagrange's theorem — If H is a subgroup of a group G, then $\displaystyle \left|G\right|=\left[G:H\right]\cdot \left|H\right|$ .

* Proof
When H is a multiplicative subgroup of order n, it follows from Lagrange’s Theorem in group theory
that an = 1 for any a ∈ H. Hence, H is precisely the set of n roots of the polynomial X n − 1, i.e.,
$∏ (X − a) = X n − 1.$

* Lemma 10.2
  
* univariate sum-check protocol
  


## FRI
> see ldt implment.
FRI is short for Fast Reed-Solomon Interactive Oracle Proof of Proximity, also abbreviated as "Fast RS-IOPP".

On a high-level, FRI proves that a function `f : H→F` is close to a polynomial of low degree d.  Here,
by low degree, we mean that `d ≪ |H|`. 


### Two Phases
### Commit Phase
The prover commits to (via Merkle trees) a series of functions generated from f and random elements v0, v1, . . . from K provided by the verifier at each round.

* split-and-fold
The whole derivation of pi+1 from pi is often known as split-and-fold due to the prover splitting the initial polynomial into two and then folding it into one using a random value.
  It works using split-and-fold:
  * split-and-fold, eg:
    if poly p(x) = 1 + 2x + 3x^2 + 4x^3
    Then it can be split into p(x) = p_L(x^2) + x*p_R(x^2), and then use a random `a` to fold p_L,p_R into new poly: p(x)=p_L(x^2) + a*p_R(x^2)
  * pros
    As the d+1 points can defined a poly with degree-d. So that we can check d+1 points to check the poly's degree.
    However, with ldt, every iteration the poly half itself, which means only need log(d) point can confirm its degree.


### Query Phase
The prover provides a set of evaluations of the previously committed functions at a point 
randomly chosen by the verifier.

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
> They are much faster prover than FRI but larger evaluation proofs.

### Tensor Product Structure


### error-correcting codes
An error-correcting code is specified by an encoding function E. E maps vectors in Fm to slightly longer vectors, in Fρ ·m , where ρ is called the rate of the code
(think of ρ as a constant such as 1/4). E must be “distance-amplifying”. This means that if messages
u1 , u2 ∈ Fm disagree in even a single coordinate, then E(u1 ) and E(u2 ) should disagree in a constant fraction
of coordinates. The distance of the code is the minimum disagreement between any two codewords E(u1 )
and E(u2 ). The relative distance γ of the code is the distance divided by the codeword length.

The code is linear if E is a linear function. That is, E(a·u1+b·u2)=a·E(u1)+b·E(u2) for any messages u1, u2 ∈ Fm and scalars a, b ∈ F.

A classic example of a linear code is the Reed-Solomon code.
