use std::collections::HashMap;

const COINS: [u32; 4] = [25, 10, 5, 1];

fn change(mut money: u32) -> HashMap<u32, u32> {
    let mut change = HashMap::new();
    for &coin in &COINS {
        change.insert(coin, money / coin);
        money %= coin;
    }
    change
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_31() {
        assert_eq!(hashmap! { 25 => 1, 10 => 0, 5 => 1, 1 => 1 }, change(31));
    }
}
