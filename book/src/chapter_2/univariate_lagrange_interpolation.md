# Univariate Lagrange Interpolation
TODO


## background
* aim
To encode vectors into polynomials with(coefficient form)

### Univariate Polynomial Repr
f(x): aX
* x: domain
* y: evaluation
* a: coefficient
* X: basis

### Lagrange Interpolation

So, when we wanna encode vector into polynomials, we can regard vector as
domain(x), evaluation(y), coefficient(a).
> why no one regard the encoded vector as domain?
> maybe poly can be express in coeffs and evaluate form(lots of points(x, y))

1. regard vector as coefficients
2. regard vector as evaluations



## Lagrange basis polynomials



## More about poly
### expression
1. evaluations form: (x,y)
2. coefficients: f = ax^2 + ax + cx + c
3. factorization form: f = (x+a)(x+c)

### convert
1. lagrange interpolation : evaluation form -> coefficient form.
   1. evaluate domain:  coefficient form -> evaluation form
2. gcd: coefficient form -> factorization form.

https://math.libretexts.org/Courses/Angelo_State_University/Finite_Mathematics/01%3A_Algebra_Essentials/1.05%3A_Factoring_Polynomials#:~:text=Howto%3A%20Given%20a%20polynomial%20expression%2C%20factor%20out%20the,of%20the%20terms%20we%20need%20to%20multiply%20by.
factor polynomials to coefficient

Write the factored expression as the product of the GCF and the sum of the terms we need to multiply by.