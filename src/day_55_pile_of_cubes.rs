fn pile_of_cubes_iterative(m: i64) -> i64 {
    let mut total = 0;
    let mut n: i64 = 0;

    while total < m {
        n += 1;
        total += n.pow(3)
    }

    if total == m {
        n
    } else {
        0
    }
}

fn pile_of_cubes_formula(m: i64) -> i64 {
    let n = ((8.0 * (m as f64).sqrt() + 1.0).sqrt() - 1.0) / 2.0;
    if n == n.floor() {
        n.floor() as i64
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_valid_volume() {
        assert_eq!(45, pile_of_cubes_iterative(1071225))
    }

    #[test]
    fn test_invalid_volume() {
        assert_eq!(0, pile_of_cubes_formula(91716553919377))
    }

    #[bench]
    fn bench_iterative(b: &mut Bencher) {
        b.iter(|| pile_of_cubes_iterative(test::black_box(91716553919377)));
    }

    #[bench]
    fn bench_formula(b: &mut Bencher) {
        b.iter(|| pile_of_cubes_formula(test::black_box(91716553919377)));
    }
}
