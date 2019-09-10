fn coffee(change: f64) -> String {
    let drink_type = |change: f64| {
        // Why no `match` statements?
        // Well, this was the case at the time of writing:
        // - floating-point patterns were unstable
        // TODO: revisit this when float pattern are stable
        if change == 2.2 {
            return Some("Americano");
        }
        if change == 2.3 {
            return Some("Latte");
        }
        if change == 2.4 {
            return Some("Flat white");
        }
        if change == 3.5 {
            return Some("Filter");
        }
        None
    };

    if let Some(drink) = drink_type(change) {
        format!("Here is your {}, have a nice day!", drink)
    } else {
        String::from("Sorry, exact change only, try again tomorrow!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latte() {
        assert_eq!("Here is your Latte, have a nice day!", coffee(2.3));
    }

    #[test]
    fn test_surplus() {
        assert_eq!("Sorry, exact change only, try again tomorrow!", coffee(2.7));
    }
}
