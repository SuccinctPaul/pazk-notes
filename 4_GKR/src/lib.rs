// https://github.com/Ethan-000/Linear_GKR_Protocol
// https://github.com/jeong0982/gkr

pub mod arithmetic;
pub mod constraint_system;
pub mod gkr;
pub mod gkr_sumcheck;
pub mod poly;
pub mod utils;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("Hello, world!");
    }
}
