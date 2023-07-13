# Introduction

## verifiable computing (VC)
> This manuscript is about verifiable computing (VC).

VC refers to cryptographic protocols called interactive proofs (IPs) and arguments that enable a prover to provide a guarantee to a verifier that the prover performed
a requested computation correctly.

### proof
Proof is anything that convinces someone that a statement is true,

* Traditionally, a proof is a static object that can be easily checked step-by-step for correctness, because each individual step of the proof should
  be trivial to verify.
* IPs allow for interaction between prover and verifier, as well as a tiny but nonzero probability that an invalid proof passes verification.

### proof vs arguments
The difference between IPs and arguments is that arguments (but not IPs) permit the existence of “proofs” of incorrect statements, so long as those “proofs” require exorbitant computational power to find.
For example, an argument, but not an IP, might make use of a cryptosystem, such that it is possible for a cheating prover to find a convincing “proof” of a false statement if (and only if) the prover can break the cryptosystem.


## proof system
“proof system” is any procedure that decides what is and is not a convincing proof.
A proof system is specified by a verification procedure that takes as input any statement and a claimed “proof” that the statement is true, and decides whether or not the proof is valid
* Ideally, the verification procedure will be “efficient”. Roughly, this means that simple statements
  should have short (convincing) proofs that can be checked quickly.
* Ideally, proving should be efficient too. Roughly, this means that simple statements should have short
  (convincing) proofs that can be found quickly.

### properties
* completeness.Any true statement should have a convincing proof of its validity.
* soundness. False statement shouldn't have a convincing proof.
* Ideally, the verification procedure will be “efficient”. Roughly, this means that simple statements
  should have short (convincing) proofs that can be checked quickly. 
* Ideally, proving should be efficient too. Roughly, this means that simple statements should have short
  (convincing) proofs that can be found quickly.


## zkSNARK
Succinct Non-interactive Arguments of Knowledge, or zk-SNARKs for short.

#### propertity
* “Succinct” means that the proofs are short. 
* “Non-interactive” means that the proof is static, consisting of a single message from the prover. 
* “Of Knowledge” roughly means that the protocol establishes not only that a statement is true, but also that the prover knows a “witness” to the veracity of the statement.
* "zero-knowledge" This means that the proof or argument reveals nothing but its own validity.


## Approaches to construct zkSnark protocol
Argument systems are typically developed in a two-step process.

1. First, an information-theoretically secure protocol, such as an
IP, multi-prover interactive proof (MIP), or probabilistically checkable proof (PCP), is developed for a
model involving one or more provers that are assumed to behave in some restricted manner (e.g., in an MIP,
the provers are assumed not to send information to each other about the challenges they receive from the
verifier).

2. Second, the information-theoretically secure protocol is combined with cryptography to “force”
  a (single) prover to behave in the restricted manner, thereby yielding an argument system. This second
  step also often endows the resulting argument system with important properties, such as zero-knowledge,
  succinctness, and non-interactivity. If the resulting argument satisfies all of these properties, then it is in fact
  a zk-SNARK.

* Argument systems are IPs, but where the soundness guarantee need only hold against cheating provers that
  run in polynomial time.


### information-theoretically secure protocol
There are a variety of promising approaches to developing efficient zk-SNARKs, which can be categorized by the type of 
information-theoretically secure protocol upon which they are based. These include: 
1. IPs,
[source impl](../../../1-ip)
2. MIPs,
An MIP is like an IP, except that there are multiple provers, and these provers are assumed not to share
information with each other regarding what challenges they receive from the verifier.
A common analogy for MIPs is placing two or more criminal suspects in separate rooms before interrogating them, to see if they
can keep their story straight.

3. PCPs, or more precisely a related notion called interactive oracle proofs(IOPs), which is a hybrid between an IP and a PCP
4. linear PCPs.
