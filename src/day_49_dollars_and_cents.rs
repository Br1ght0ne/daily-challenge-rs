fn format_dollars(amount: f64) -> String {
    format!("${:.2}", amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6_dollars_20_cents() {
        assert_eq!("$6.20", format_dollars(6.2));
    }
}
