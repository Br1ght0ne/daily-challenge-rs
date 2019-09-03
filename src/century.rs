fn century(year: u32) -> String {
    let century = year / 100 + 1;
    let suffix = match century % 100 {
        11 | 12 | 13 => "th",
        _ => match century % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{}{}", century, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2019_is_21st() {
        assert_eq!(String::from("21st"), century(2019));
    }
}
