fn pile_of_cubes(m: i64) -> i64 {
    let n = ((8.0 * (m as f64).sqrt() + 1.0).sqrt() - 1.0) / 2.0;
    if n == n.floor() {
        n.floor() as i64
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_volume() {
        assert_eq!(45, pile_of_cubes(1071225))
    }

    #[test]
    fn test_invalid_volume() {
        assert_eq!(0, pile_of_cubes(91716553919377))
    }
}
