
use std::collections::HashMap;

pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }

    counts.values().map(|&count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let ar = [10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_example_2() {
        let ar = [1, 2, 1, 2, 1, 3, 2];
        assert_eq!(sock_merchant(7, &ar), 2);
    }

    #[test]
    fn test_empty() {
        assert_eq!(sock_merchant(0, &[]), 0);
    }

    #[test]
    fn test_no_pairs() {
        let ar = [1, 2, 3, 4];
        assert_eq!(sock_merchant(4, &ar), 0);
    }
}