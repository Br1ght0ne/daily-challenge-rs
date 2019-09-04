use itertools::Itertools;

fn faro_shuffle<T: Clone>(deck: &[T]) -> Vec<T> {
    let mid = deck.len() / 2;
    deck[..mid]
        .iter()
        .interleave(deck[mid..].iter())
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck_of_even_length() {
        assert_eq!(
            vec![1, 4, 2, 5, 3, 6],
            faro_shuffle(&vec![1, 2, 3, 4, 5, 6])
        )
    }

    #[test]
    fn test_deck_of_odd_length() {
        assert_eq!(vec![1, 3, 2, 4, 5], faro_shuffle(&vec![1, 2, 3, 4, 5]));
    }
}
