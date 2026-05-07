pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return String::from("NO");
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_impossible_catch_up() {
        assert_eq!(kangaroo(10, 2, 20, 2), "NO");
    }

    #[test]
    fn test_exact_division() {
        assert_eq!(kangaroo(21, 6, 47, 3), "NO");
    }
}
