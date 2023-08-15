use rand::distributions::{Alphanumeric, DistString};
use rand_core::OsRng;

pub fn random_chars(k: usize) -> Vec<char> {
    let n = 1 << k;
    let random_code = Alphanumeric.sample_string(&mut OsRng, n);
    random_code.chars().collect::<Vec<char>>()
}

#[cfg(test)]
mod test {
    use crate::utils::random_chars;
    use rand::distributions::{Alphanumeric, DistString};
    use rand_core::OsRng;

    #[test]
    fn test_random_char() {
        let k = 5;
        let n = 1 << k;
        let random_code = Alphanumeric.sample_string(&mut OsRng, n);
        println!("{:?}", random_code);
        let a = random_code.chars().collect::<Vec<char>>();
        println!("{:?}", a);

        let b = random_chars(k);
        println!("{:?}", b);
    }
}
