fn number_neighbor(number: u32) -> Vec<u32> {
    let len = number.to_string().len() as u32;
    (0..len)
        .map(|delta| 10u32.pow(delta))
        .flat_map(|delta| vec![number - delta, number + delta])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_neighbor() {
        assert_eq!(vec![221, 223, 212, 232, 122, 322], number_neighbor(222));
    }
}
