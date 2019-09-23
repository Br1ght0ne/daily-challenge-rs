use itertools::Itertools;

fn maximum_thrill(atms: &[i32]) -> i32 {
    if atms.len() == 1 {
        return atms[0] * 2;
    }

    atms.into_iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i1, v1), (i2, v2))| v1 + v2 + (i1 as i32 - i2 as i32).abs())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::maximum_thrill;

    #[test]
    fn test_empty() {
        assert_eq!(0, maximum_thrill(&[]));
    }

    #[test]
    fn test_single_atm() {
        assert_eq!(6, maximum_thrill(&[3]));
    }

    #[test]
    fn test_one_simple() {
        assert_eq!(8, maximum_thrill(&[3, 1, 3]));
    }

    #[test]
    fn test_any_simple() {
        assert_eq!(10, maximum_thrill(&[2, 3, 4, 5]));
    }

    #[test]
    fn test_one_greater_distance() {
        assert_eq!(26, maximum_thrill(&[10, 10, 11, 13, 7, 8, 9]));
    }

    #[test]
    fn test_complicated_max_distance() {
        let atms = &[2, 3, 4, 5, 10, 6, 7, 8, 9, 10, 11, 12, 4, 4, 2, 2, 12, 8];
        assert_eq!(34, maximum_thrill(atms));
    }
}
