pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0; 6];
    
    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut max_freq = 0;
    let mut result_id = 0;

    for (id, &freq) in counts.iter().enumerate().skip(1) {
        if freq > max_freq {
            max_freq = freq;
            result_id = id as i32;
        }
    }

    result_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds() {
        assert_eq!(migratory_birds(&[1, 1, 2, 2, 3]), 1);
        assert_eq!(migratory_birds(&[1, 4, 4, 4, 5, 3]), 4);
        assert_eq!(migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]), 3);
    }
}
