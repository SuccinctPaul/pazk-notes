// convert a num into its binary form
// eg: 8 -> 1000, will output [1, 0, 0, 0]
pub fn convert_to_binary(bit_len: &usize, num: usize) -> Vec<usize> {
    let a: usize = 1 << bit_len;
    assert!(a >= num);
    (0..*bit_len)
        .map(|n| (num >> n) & 1)
        .rev()
        .collect::<Vec<usize>>()
}

// convert a num into its binary form
// eg: 8 -> 1000, will output [1, 0, 0, 0]
pub fn convert_from_binary(num: &Vec<usize>) -> usize {
    num.iter().rev().enumerate().map(|(i, n)| n << i).sum()
}

#[cfg(test)]
mod test {
    use crate::utils::{convert_from_binary, convert_to_binary};

    #[test]
    fn test_convert() {
        let raw: Vec<usize> = vec![1, 2, 3, 4, 8, 17];

        let binaries = raw
            .iter()
            .map(|i| convert_to_binary(&5, i.clone()))
            .collect::<Vec<_>>();

        let new_nums = binaries
            .iter()
            .map(|b| convert_from_binary(b))
            .collect::<Vec<_>>();

        assert_eq!(new_nums, raw);
    }

    #[test]
    fn test_convert_into_binary() {
        for i in [1, 2, 3, 4, 8, 17] {
            let binary = convert_to_binary(&4, i);
            println!("{:?} ->  {:?}", i, binary);
        }
    }

    #[test]
    fn test_convert_from_binary() {
        let mut binary: Vec<usize> = vec![1, 0, 0, 0, 1];
        let raw = convert_from_binary(&binary);
        println!("{:?} ->  {:?}", raw, binary);
    }
}
