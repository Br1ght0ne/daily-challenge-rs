#[derive(Debug, PartialEq)]
enum Piece {
    Archer,
    Swordsman,
    Pikeman,
    Cavalry,
}

fn fight_resolve<'a>(attacker: &'a Piece, defender: &'a Piece) -> &'a Piece {
    use Piece::*;

    match (attacker, defender) {
        (Swordsman, Archer) | (Pikeman, Swordsman) | (Cavalry, Pikeman) | (Archer, Cavalry) => {
            defender
        }
        (attacker, _) => attacker,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight_resolve() {
        use Piece::*;

        let (attacker, defender) = (Pikeman, Swordsman);
        assert_eq!(&defender, fight_resolve(&attacker, &defender));
    }
}
