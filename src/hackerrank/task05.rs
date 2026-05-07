pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let apple_count = apples
        .iter()
        .filter(|&&d| {
            let pos = a + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    let orange_count = oranges
        .iter()
        .filter(|&&d| {
            let pos = b + d;
            pos >= s && pos <= t
        })
        .count() as i32;

    println!("{}", apple_count);
    println!("{}", orange_count);

    (apple_count, orange_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_apples_and_oranges() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let (apples_res, oranges_res) = count_apples_and_oranges(s, t, a, b, &apples, &oranges);

        assert_eq!(apples_res, 1);
        assert_eq!(oranges_res, 1);
    }

    #[test]
    fn test_no_fruits_in_range() {
        let s = 10;
        let t = 20;
        let a = 5;
        let b = 25;
        let apples = vec![1, 2];
        let oranges = vec![-1, -2];

        let (apples_res, oranges_res) = count_apples_and_oranges(s, t, a, b, &apples, &oranges);

        assert_eq!(apples_res, 0);
        assert_eq!(oranges_res, 0);
    }
}
