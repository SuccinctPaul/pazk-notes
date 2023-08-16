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




## FRI
10.4

## Ligero and Breakdown
10.5
