# The Power of Randomness: Fingerprinting and Freivalds’ Algorithm


## Reed-Solomon Fingerprinting

> The proof systems covered in this survey derive much of their power and efficiency from their use of ran- domness

> Before we discuss the details of such proof systems, let us first develop an appreciation for how randomness can be exploited to dramatically improve the efficiency of certain algorithms.


* Fact 2.1. For any two distinct (i.e., unequal) polynomials pa, pb of degree at most n with coefficients in Fp, pa(x) = pb(x) for at most n values of x in Fp.

Let pa(x) = ∑ni=1 ai · xi−1 and similarly pb(x) = ∑ni=1 bi · xi−1. Observe that both pa and pb are polyno- mials in x of degree at most n − 1. The value v that Alice sends to Bob in the communication protocol is precisely pa(r), and Bob compares this value to pb(r).

* Fact 2.2. Any nonzero polynomial of degree at most n over any field has at most n roots.


* reference
  https://zh.wikipedia.org/wiki/%E9%87%8C%E5%BE%B7-%E6%89%80%E7%BD%97%E9%97%A8%E7%A0%81



## Freivalds’ Algorithm
