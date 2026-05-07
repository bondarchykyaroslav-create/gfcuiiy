pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let l = a.iter().fold(a[0], |acc, &x| lcm(acc, x));
    let g = b.iter().fold(b[0], |acc, &x| gcd(acc, x));

    let mut count = 0;
    let mut multiple = l;
    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }
    count
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b).abs() / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_between_two_sets() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
        assert_eq!(get_total_x(&[2, 6], &[24, 36]), 2);
    }

    #[test]
    fn test_gcd_lcm() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(lcm(4, 6), 12);
    }
}
