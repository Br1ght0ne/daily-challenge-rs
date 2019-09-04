fn coffee(_change: f64) -> String {
    // TODO: f64 is not Eq or Hash
    // Thus can't use HashMap.
    // Also, no matching on floats in match arms.
    // Seems like it should be skipped for now.
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_latte() {
        assert_eq!("Here is your Latte, have a nice day!", coffee(2.2));
    }

    #[test]
    #[ignore]
    fn test_surplus() {
        assert_eq!("Sorry, exact change only, try again tomorrow!", coffee(2.7));
    }
}
