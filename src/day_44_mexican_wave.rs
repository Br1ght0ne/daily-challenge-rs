fn wave(input: &str) -> Vec<String> {
    use std::iter;

    iter::repeat(input)
        .take(input.len())
        .enumerate()
        .map(|(i, part)| {
            part.chars()
                .enumerate()
                .map(|(j, c)| if i == j { c.to_ascii_uppercase() } else { c })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave() {
        assert_eq!(
            vec!["Abc".to_string(), "aBc".to_string(), "abC".to_string()],
            wave("abc")
        );
    }
}
