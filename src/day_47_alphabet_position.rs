fn alphabet_position(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|char| (char as usize - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet_position() {
        assert_eq!(
            alphabet_position("The sunset sets at twelve o' clock."),
            String::from("20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11")
        );
    }
}
