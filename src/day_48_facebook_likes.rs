fn likes(names: &[&str]) -> String {
    match names {
        [] => String::from("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, ..] => format!("{}, {} and {} others like this", a, b, names.len() - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_likes() {
        assert_eq!(String::from("no one likes this"), likes(&[]));
        assert_eq!(String::from("Alice likes this"), likes(&["Alice"]));
        assert_eq!(
            String::from("Alice and Bob like this"),
            likes(&["Alice", "Bob"])
        );
        assert_eq!(
            String::from("Alice, Bob and Charlie like this"),
            likes(&["Alice", "Bob", "Charlie"])
        );
        assert_eq!(
            String::from("Alice, Bob and 2 others like this"),
            likes(&["Alice", "Bob", "Charlie", "Danny"])
        );
    }
}
