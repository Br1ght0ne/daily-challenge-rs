fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => String::from("no one likes this"),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        len => format!(
            "{}, {} and {} others like this",
            names[0],
            names[1],
            len - 2
        ),
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
