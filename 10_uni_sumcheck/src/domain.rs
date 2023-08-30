pub struct Domain {
    h: Vec<F>,
}

impl Domain {
    pub fn new(k: usize) -> Self {
        let n = 1 << k;

        let mut extended_omega = F::ROOT_OF_UNITY;

        // Get omega, the 2^{k}'th root of unity (i.e. n'th root of unity)
        // The loop computes omega = extended_omega ^ {2 ^ (extended_k - k)}
        //           = (omega^{2 ^ (S - extended_k)})  ^ {2 ^ (extended_k - k)}
        //           = omega ^ {2 ^ (S - k)}.
        // Notice that omega ^ {2^k} = omega ^ {2^S} = 1.
        let mut omega = extended_omega;
        for _ in k..extended_k {
            omega = omega.square();
        }
        let omega = omega;
        let mut omega_inv = omega; // Inversion computed later

        {
            // Compute the evaluations of t(X) = X^n - 1 in the coset evaluation domain.
            // We don't have to compute all of them, because it will repeat.
            let orig = F::ZETA.pow_vartime(&[n as u64, 0, 0, 0]);
            let step = extended_omega.pow_vartime(&[n as u64, 0, 0, 0]);
            let mut cur = orig;
            loop {
                t_evaluations.push(cur);
                cur *= &step;
                if cur == orig {
                    break;
                }
            }
            assert_eq!(t_evaluations.len(), 1 << (extended_k - k));

            // Subtract 1 from each to give us t_evaluations[i] = t(zeta * extended_omega^i)
            for coeff in &mut t_evaluations {
                *coeff -= &F::ONE;
            }

            // Invert, because we're dividing by this polynomial.
            // We invert in a batch, below.
        }
    }
}
