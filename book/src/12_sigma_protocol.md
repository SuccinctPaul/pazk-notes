# Σ-Protocols and Commitments from Hardness of Discrete Logarithm

## Cryptographic Background

### Introduction to Groups
* gruop
A group is a collection of elements equipped with a binary operation (which we denote by `·` and refer
to in this manuscript as multiplication) that satisfies the following four properties:
  * Closure: the product of two elements in G are also in G, i.e., for all a, b ∈ G, a · b is also in G. 
  * Associativity: for all a, b, c ∈ G, a · (b · c) = (a · b) · c. 
  * Identity: there an element denoted e ∈ G such that e · g = g · e = g for all g ∈ G. 
  * Invertibility: For each g ∈ G, there is an element h in G such that g · h = e . This element h is denoted g^(−1) .

* abelian group
A gourp, which operation is commutative(means `x*y=y*x`, is abelian group.


* cyclic
A group G is said to be cyclic if there is some group element g such that all group elements can be
generated by repeatedly multiplying g with itself,
Any cyclic group is abelian.

* Order of group
  The cardinality |G| is called the order of G. A basic fact from group theory is that for any element
  g ∈ G, g|G| = e .

* subgroup
  A subgroup of a group G is a subset H of G that itself forms a group under the same binary operation as
  G itself. Another basic fact from group theory states that the order of any subgroup H of G divides the order
  of G itself.

### Discrete Logarithm Problem
For a specified group G the discrete logarithm problem takes as input two group elements g and h, and the
goal is to output a positive integer i such that g^i = h (if G is of prime order then such an i is guaranteed to
exist).


### Elliptic curve groups
Any elliptic curve group is defined with respect to a (finite) field F, called the base field of the curve.

Group elements correspond to pairs of points `(x, y) ∈ F×F` that satisfy an equation of the form `y2 = x3 + ax + b` for designated field elements a and b.



#### Scalar field vs. base field.
Elliptic curve groups used in practice are chosen to have large prime order.
The field of size equal to the (prime) order of the elliptic curve group G is typically referred to as the scalar field of G.

Recall that prime-order groups G are cyclic: for any group element g ̸= 1G , we can write `G = {gx : x = 0, 1, . . . , |G| − 1}` . 
Hence, we can think of elements x of the scalar field of G as exponents, when expressing G as powers of a generator g.