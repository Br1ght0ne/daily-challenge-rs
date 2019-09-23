fn meeting(list: &str) -> String {
    let list = list.to_uppercase();
    let list: Vec<_> = list.split(|c| [';', ':'].contains(&c)).collect();
    let mut list: Vec<_> = list
        .chunks_exact(2)
        .map(|chunk| format!("({}, {})", chunk[1], chunk[0]))
        .collect();
    list.sort();
    list.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_example() {
        let list = "Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill".to_string();
        assert_eq!(
            meeting(&list),
            "(CORWILL, ALFRED)(CORWILL, FRED)(CORWILL, RAPHAEL)(CORWILL, WILFRED)(TORNBULL, BARNEY)(TORNBULL, BETTY)(TORNBULL, BJON)"
        )
    }
}
