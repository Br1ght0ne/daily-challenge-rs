fn shift<T: Clone>(matrix: Vec<Vec<T>>, n: usize) -> Vec<Vec<T>> {
    match matrix.get(1).map(|row| row.len()) {
        None => matrix,
        Some(len) => {
            let mut flat = matrix.into_iter().flatten().collect::<Vec<_>>();
            flat.rotate_right(n);
            flat.chunks_exact(len).map(|chunk| chunk.to_vec()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4_1() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['a', 'b', 'c', 'd'],
            vec!['1', '2', '3', '4'],
            vec!['c', 'o', 'd', 'e'],
            vec!['b', 'l', 'a', 'h'],
        ];
        let expected: Vec<Vec<char>> = vec![
            vec!['h', 'a', 'b', 'c'],
            vec!['d', '1', '2', '3'],
            vec!['4', 'c', 'o', 'd'],
            vec!['e', 'b', 'l', 'a'],
        ];
        assert_eq!(shift(matrix, 1), expected);
    }

    #[test]
    fn test_4x3_1() {
        let matrix: Vec<Vec<char>> = vec![
            vec!['a', 'b', 'c', 'd'],
            vec!['1', '2', '3', '4'],
            vec!['c', 'o', 'd', 'e'],
        ];
        let expected: Vec<Vec<char>> = vec![
            vec!['e', 'a', 'b', 'c'],
            vec!['d', '1', '2', '3'],
            vec!['4', 'c', 'o', 'd'],
        ];
        assert_eq!(shift(matrix, 1), expected);
    }

    #[test]
    fn test_2x2_1() {
        let matrix: Vec<Vec<char>> = vec![vec!['a', 'b'], vec!['c', 'd']];
        let expected: Vec<Vec<char>> = vec![vec!['d', 'a'], vec!['b', 'c']];
        assert_eq!(shift(matrix, 1), expected);
    }

    #[test]
    fn test_empty_1() {
        assert_eq!(shift(Vec::<Vec<char>>::new(), 1), Vec::<Vec<char>>::new());
    }
}
