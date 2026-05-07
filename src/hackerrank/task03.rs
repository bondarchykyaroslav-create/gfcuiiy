pub fn staircase(n: i32) -> String {
    let mut result = String::new();
    let n_usize = n as usize;

    for i in 1..=n_usize {
        let line = format!("{:>width$}", "#".repeat(i), width = n_usize);
        result.push_str(&line);
        if i < n_usize {
            result.push('\n');
        }
    }
    
    println!("{}", result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_6() {
        let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";
        assert_eq!(staircase(6), expected);
    }
}