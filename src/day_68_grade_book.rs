fn grade(a: u32, b: u32, c: u32) -> String {
    let mean = (a + b + c) / 3;
    let letter = match dbg!(mean) {
        _ if (90..=100).contains(&mean) => "A",
        _ if (80..90).contains(&mean) => "B",
        _ if (70..80).contains(&mean) => "C",
        _ if (60..70).contains(&mean) => "D",
        _ if (0..60).contains(&mean) => "F",
        _ => unreachable!("mean ({}) can't be < 0", mean),
    };
    let sign = if mean % 10 < 5 { "-" } else { "+" };
    format!("{}{}", letter, sign)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade_c_minus() {
        assert_eq!("C-", grade(64, 55, 92));
    }

    #[test]
    fn test_grade_a_minus() {
        assert_eq!("A-", grade(99, 89, 93));
    }

    #[test]
    fn test_grade_c_plus() {
        assert_eq!("C+", grade(33, 99, 95));
    }
}
