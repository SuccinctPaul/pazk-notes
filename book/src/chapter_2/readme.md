# The Power of Randomness: Fingerprinting and Freivalds’ Algorithm


## Reed-Solomon Fingerprinting

> The proof systems covered in this survey derive much of their power and efficiency from their use of ran- domness

> Before we discuss the details of such proof systems, let us first develop an appreciation for how randomness can be exploited to dramatically improve the efficiency of certain algorithms.


* Fact 2.1. For any two distinct (i.e., unequal) polynomials pa, pb of degree at most n with coefficients in Fp, pa(x) = pb(x) for at most n values of x in Fp.

Let pa(x) = ∑ni=1 ai · xi−1 and similarly pb(x) = ∑ni=1 bi · xi−1. Observe that both pa and pb are polyno- mials in x of degree at most n − 1. The value v that Alice sends to Bob in the communication protocol is precisely pa(r), and Bob compares this value to pb(r).

* Fact 2.2. Any nonzero polynomial of degree at most n over any field has at most n roots.


* reference
  https://zh.wikipedia.org/wiki/%E9%87%8C%E5%BE%B7-%E6%89%80%E7%BD%97%E9%97%A8%E7%A0%81
  
  
  
## The power of randomness
In summary, both protocols reduced the task of checking equality of two large objects (the vectors a and b in the fingerprinting protocol,
and the claimed answer matrix and true answer matrix in Freivalds’ algorithm) to checking equality of just a single random entry of
distance-amplified encodings of those objects. While deterministically checking equality of the two large objects would be very
expensive in terms of either communication or computation time, evaluating a single entry of the each object’s encoding can be
done with only logarithmic communication or in just linear time.


## Implement
[Reed-Solomon Fingerprinting](../../../2_Reed_Solomon_Fingerprinting)
[Freivalds’ Algorithm](../../../2_Freivalds_Algorithm)
