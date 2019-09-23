fn next_happy_year(year: u32) -> u32 {
    let mut year = year + 1;
    while !happy(year) {
        year += 1;
    }
    year
}

fn happy(year: u32) -> bool {
    let mut chars = year.to_string().chars().collect::<Vec<_>>();
    chars.sort();
    let len = chars.len();
    chars.dedup();
    chars.len() == len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_after_unhappy_one_duplicate() {
        assert_eq!(7801, next_happy_year(7712));
    }

    #[test]
    fn test_happy_after_unhappy_two_duplicates() {
        assert_eq!(1023, next_happy_year(1001));
    }

    #[test]
    fn test_happy_after_happy() {
        assert_eq!(2019, next_happy_year(2018));
    }
}
