//! commitment scheme
//! Zero-Knowledge PoK of Opening of Pedersen Commitments Satisfying Product Relationship
//! The protocol-10 in pazk-book#196
mod keygen;
mod prover;
mod verifier;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pok() {}
}
