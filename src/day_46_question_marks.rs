fn remove_question_marks(input: &str) -> String {
    input.replace("?", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_question_marks() {
        assert_eq!("Yes/No", remove_question_marks("?Yes?/No?"));
    }
}
