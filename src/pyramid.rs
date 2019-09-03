fn pyramid(floors: usize) -> String {
    (0..floors)
        .map(|f| " ".repeat(floors - f - 1) + &"*".repeat(2 * f + 1))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyramid() {
        assert_eq!("  *\n ***\n*****", pyramid(3));
    }
}
