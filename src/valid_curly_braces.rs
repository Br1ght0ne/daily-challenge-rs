fn valid_curly_braces(input: &str) -> bool {
    let mut counter = 0;
    for c in input.chars() {
        match c {
            '{' => counter += 1,
            '}' => counter -= 1,
            _ => (),
        };
        if counter < 0 {
            return false;
        }
    }
    counter == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oocc_is_valid() {
        assert!(valid_curly_braces("{{}}"));
    }

    #[test]
    fn test_oo_is_not_valid() {
        assert!(!valid_curly_braces("{{"));
    }

    #[test]
    fn test_occ_is_not_valid() {
        assert!(!valid_curly_braces("{}}"));
    }

    #[test]
    fn test_empty_is_valid() {
        assert!(valid_curly_braces(""));
    }

    #[test]
    fn test_valid_mixed_nesting() {
        assert!(valid_curly_braces("{{{}{}}}"));
    }
}
