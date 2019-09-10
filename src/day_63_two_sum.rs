use std::collections::HashMap;

fn two_sum(ns: &[i32], sum: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &n) in ns.iter().enumerate() {
        let diff = sum - n;
        if let Some(&j) = map.get(&diff) {
            return Some((j, i));
        }
        map.insert(n, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_elements() {
        assert_eq!(Some((0, 2)), two_sum(&[1, 2, 3], 4));
    }

    #[test]
    fn test_2_elements() {
        assert_eq!(Some((0, 1)), two_sum(&[4, 5], 9));
    }

    #[test]
    fn test_short_array() {
        assert_eq!(None, two_sum(&[1], 2));
    }

    #[test]
    fn test_no_elements() {
        assert_eq!(None, two_sum(&[1, 3, 5], 7));
    }
}
