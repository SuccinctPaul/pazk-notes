use crate::gkr_sumcheck::F_r_Poly;
use crate::poly::{MPolynomial, Polynomial};
use crate::utils::convert_to_binary;
use bls12_381::Scalar;
use std::ops::{Add, Mul};
use std::path::Iter;

pub struct Prover {
    v_l: usize, // the constants_part var_num.  v_l + v_r = ki + 2*k_i_plus_1
    v_r: usize, // the variable_part var_num. equals to `v` in standard sumcheck.
    add: MPolynomial,
    mult: MPolynomial,
    w_i_plus_1: MPolynomial,
    r_i: Vec<usize>, // the constant var part.
}

impl Prover {
    pub fn new((add, mult, w_i_plus_1): F_r_Poly, r_i: Vec<usize>) -> Self {
        let (v_l, v_r) = (r_i.len(), 2 * w_i_plus_1.var_num);

        Self {
            v_l,
            v_r,
            add,
            mult,
            w_i_plus_1,
            r_i,
        }
    }

    // obtain m0 by $\sum_{b,c \in (0,1)^{k_{i+1}}}f_{r_i} = m_i $ , m1 means C1.
    #[deprecated]
    pub fn proof(&self) -> Scalar {
        let k_i_plus_1 = self.w_i_plus_1.var_num;

        let mut res = Scalar::zero();
        for i in 0..k_i_plus_1 {
            let a = convert_to_binary(&k_i_plus_1, i);
            let w_a = self.w_i_plus_1.evaluate(&a);

            // ops_domain = (ri, a, b)
            let mut ops_domain = self.r_i.clone();
            ops_domain.append(&mut a.clone());

            for j in 0..k_i_plus_1 {
                let b = convert_to_binary(&k_i_plus_1, j);

                let w_b = self.w_i_plus_1.evaluate(&b);

                ops_domain.clone().append(&mut b.clone());
                let add_i = self.add.evaluate(&ops_domain);
                let multi = self.mult.evaluate(&ops_domain);

                res += add_i * (w_a + w_b) + multi * (w_a * w_b);
            }
        }
        res
    }

    // Return g1(X) = sum g(X, x_2, ..., x_v)
    // obtain  $g1(X) =  = m_i $ , m1 means C1.
    // Return g1(x) = add(r_i, (X, a2, ...,a_k_1), (b1, ..., b_k_1) * (W(X, a2, ...,a_k_1) + W(b1,...,b_k_1))
    //              + mult(r_i, (X, a2, ...,a_k_1), (b1, ..., b_k_1) * (W(X, a2, ...,a_k_1) * W(b1,...,b_k_1))
    //              = poly_add * (poly_w_a + w_b) + poly_mult * (poly_w_a * w_b)
    //              = poly_add * poly_w_a + poly_add * w_b + poly_mult * (poly_w_a * w_b)
    pub fn round_1(&self) -> Polynomial {
        let poly_add = self.add.partial_evaluate(&self.r_i);
        let poly_mult = self.mult.partial_evaluate(&self.r_i);

        let poly_w_a = self.w_i_plus_1.partial_evaluate(&vec![]);
        let w_b = self.w_i_plus_1.sum_all_evals();

        // poly_add * poly_w_a + poly_add * w_b + poly_mult * (poly_w_a * w_b)
        poly_add
            .mul(&poly_w_a)
            .add(&poly_add.mul(&w_b).add(&poly_mult.mul(&poly_w_a).mul(&w_b)))
    }

    // 1 < j < v_r, total v_r-2 rounds
    // Return g_j = (r1, ..., r_j-1, X, x_j+1, ..., x_v)
    pub fn recursive_round_j(&self, challenges: &Vec<usize>) -> Polynomial {
        assert!(self.v_r > challenges.len() || challenges.len() >= 1);

        // partial_evaluate with (r_i, challenge, X, x_i)
        let mut ops_challenge_domain = self.r_i.clone();
        ops_challenge_domain.append(&mut challenges.clone());
        let poly_add = self.add.partial_evaluate(&ops_challenge_domain);
        let poly_mult = self.mult.partial_evaluate(&ops_challenge_domain);

        let (poly_w, w_value) = if challenges.len() < self.v_r / 2 {
            //challenges only support partial of a
            let poly_w_a = self.w_i_plus_1.partial_evaluate(challenges);
            let w_b = self.w_i_plus_1.sum_all_evals();
            (poly_w_a, w_b)
        } else {
            //challenges support all a and partial b
            let mut c = challenges.chunks(self.v_r / 2);
            // evaluate all with challenge
            let w_a = self.w_i_plus_1.evaluate(&Vec::from(c.next().unwrap()));
            let poly_w_b = self
                .w_i_plus_1
                .partial_evaluate(&Vec::from(c.next().unwrap()));
            (poly_w_b, w_a)
        };

        // poly_add * poly_w_a + poly_add * w_b + poly_mult * (poly_w_a * w_b)
        poly_add.mul(&poly_w).add(
            &poly_add
                .mul(&w_value)
                .add(&poly_mult.mul(&poly_w).mul(&w_value)),
        )
    }

    // Return g_v = (r1, r2, ..., r_v-1, X_v)
    pub fn round_v(&self, challenges: &Vec<usize>) -> Polynomial {
        assert_eq!(self.v_r - 1, challenges.len());

        // partial_evaluate with (r_i, challenge, X, x_i)
        let mut ops_challenge_domain = self.r_i.clone();
        ops_challenge_domain.append(&mut challenges.clone());
        let poly_add = self.add.partial_evaluate(&ops_challenge_domain);
        let poly_mult = self.mult.partial_evaluate(&ops_challenge_domain);

        //challenges support all a and partial b
        let mut c = challenges.chunks(self.v_r / 2);
        // evaluate all with challenge
        let w_a = self.w_i_plus_1.evaluate(&Vec::from(c.next().unwrap()));
        let poly_w_b = self
            .w_i_plus_1
            .partial_evaluate(&Vec::from(c.next().unwrap()));

        // poly_add * poly_w_a + poly_add * w_b + poly_mult * (poly_w_a * w_b)
        poly_add
            .mul(&poly_w_b)
            .add(&poly_add.mul(&w_a).add(&poly_mult.mul(&poly_w_b).mul(&w_a)))
    }

    // challenges include (u, v), here we need (r,u,v)
    pub fn evaluate(
        &self,
        challenges: &Vec<usize>,
    ) -> (Scalar, Scalar, Vec<Vec<usize>>, Polynomial) {
        assert_eq!(self.v_r, challenges.len());

        // 1. evaluate add/mult at (r,u,v)
        let mut ops_challenge_domain = self.r_i.clone();
        ops_challenge_domain.append(&mut challenges.clone());
        let add_value = self.add.evaluate(&ops_challenge_domain);
        let mult_value = self.mult.evaluate(&ops_challenge_domain);

        // 2 Obtain W_i_1(u) and W_i_1(v) for verifier's final check and prepare for the `r_i_plus_1` used in next round.
        let mut c = challenges.chunks(self.v_r / 2);
        let u = Vec::from(c.next().unwrap());
        let v = Vec::from(c.next().unwrap());

        // 2.1 Obtain the values: W_i_1(u) and W_i_1(v)
        let w_u_value = self.w_i_plus_1.evaluate(&u);
        let w_v_value = self.w_i_plus_1.evaluate(&v);

        // 2.2 Let l be the unique poly satisfying l(0)=u and l(1)=v
        //      As u,v are arrays, so that, l can be a set of poly_i.
        //      The poly_i satisfying poly_i(0)=u[i] and poly_i(1)=v[i]
        let l_polys = u
            .iter()
            .zip(v)
            .map(|(ui, vi)| {
                // l_i(x) = (vi-ui)x + ui <--> l(0)=u and l(1)=v
                // As the Polynomail only support the Scalar. So we use the arrays as a usize-poly:
                //      p(x) = = a_0 + a_1 * X + ... + a_n * X^(n-1)
                //      coeffs: [a_0, a_1, ..., a_n]
                vec![*ui, vi - *ui]
            })
            .collect::<Vec<_>>();

        // 2.3 Let q be the unique poly satisfying p(0)=W_i_1(u) and p(1)=W_i_1(v).
        //      Note: Here we regard domain(0,1) on Scalar
        let p_poly = Polynomial::lagrange_interpolate(
            vec![Scalar::zero(), Scalar::one()],
            vec![w_u_value, w_v_value],
        );

        // return the add(r_i,u,v), mult(r_i,u,v), l_poly, p_poly.
        (add_value, mult_value, l_polys, p_poly)
    }
}
