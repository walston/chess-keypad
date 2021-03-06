enum Key {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Pound,
    Star,
}

mod pieces {
    pub struct Pawn;
    pub struct Rook;
    pub struct Knight;
    pub struct Bishop;
    pub struct Queen;
    pub struct King;

    pub type Piece = dyn Move;

    pub trait Move {
        fn next(_start: &char) -> Option<Vec<char>> {
            None
        }
    }

    impl Move for Pawn {
        fn next(start: &char) -> Option<Vec<char>> {
            match start {
                '1' => Some(vec![]),
                '2' => Some(vec![]),
                '3' => Some(vec![]),
                '4' => Some(vec!['1']),
                '5' => Some(vec!['2']),
                '6' => Some(vec!['3']),
                '7' => Some(vec!['4']),
                '8' => Some(vec!['5']),
                '9' => Some(vec!['6']),
                '0' => Some(vec!['8']),
                _ => None,
            }
        }
    }


    impl Move for Rook {
        fn next(start: &char) -> Option<Vec<char>> {
            match start {
                '1' => Some(vec!['2', '3', '4', '7']),
                '2' => Some(vec!['1', '3', '5', '8', '0']),
                '3' => Some(vec!['1', '2', '6', '9']),
                '4' => Some(vec!['1', '5', '6', '7']),
                '5' => Some(vec!['2', '4', '6', '8', '0']),
                '6' => Some(vec!['3', '4', '5', '9']),
                '7' => Some(vec!['1', '4', '8', '9']),
                '8' => Some(vec!['2', '5', '7', '9', '0']),
                '9' => Some(vec!['3', '6', '7', '8']),
                '0' => Some(vec!['3', '5', '8', '0']),
                _ => None,
            }
        }
    }

    impl Move for Knight {
        fn next(start: &char) -> Option<Vec<char>> {
            match start {
                '1' => Some(vec!['6', '8']),
                '2' => Some(vec!['7', '9']),
                '3' => Some(vec!['4', '8']),
                '4' => Some(vec!['2', '9']),
                '5' => None,
                '6' => Some(vec!['1', '7']),
                '7' => Some(vec!['2', '6']),
                '8' => Some(vec!['1', '3']),
                '9' => Some(vec!['2', '4']),
                '0' => Some(vec!['4', '6']),
                _ => None,
            }
        }
    }

    impl Move for Bishop {
        fn next(start: &char) -> Option<Vec<char>> {
            match start {
                '1' => Some(vec!['5', '9']),
                '2' => Some(vec!['4', '6']),
                '3' => Some(vec!['5', '7']),
                '4' => Some(vec!['2', '8']),
                '5' => Some(vec!['1', '3', '7', '9']),
                '6' => Some(vec!['2', '8']),
                '7' => Some(vec!['5', '3', '0']),
                '8' => Some(vec!['4', '6']),
                '9' => Some(vec!['5', '1', '0']),
                '0' => Some(vec!['7', '9']),
                _ => None,
            }
        }
    }

    impl Move for Queen {
        fn next(start: &char) -> Option<Vec<char>> {
            match start {
                '1' => Some(vec!['2', '3', '4', '5', '7', '9']),
                '2' => Some(vec!['1', '3', '4', '5', '6', '8', '0']),
                '3' => Some(vec!['1', '2', '5', '6', '7', '9']),
                '4' => Some(vec!['1', '2', '5', '6', '7', '8']),
                '5' => Some(vec!['1', '2', '3', '4', '6', '7', '8', '9', '0']),
                '6' => Some(vec!['2', '3', '4', '5', '8', '9']),
                '7' => Some(vec!['1', '3', '4', '5', '8', '9', '0']),
                '8' => Some(vec!['2', '4', '5', '6', '7', '9', '0']),
                '9' => Some(vec!['1', '3', '5', '6', '7', '8', '0']),
                '0' => Some(vec!['2', '5', '7', '8', '9']),
                _ => None,
            }
        }
    }

    impl Move for King {
        fn next(start: &char) -> Option<Vec<char>> {
            match start {
                '1' => Some(vec!['2', '4', '5']),
                '2' => Some(vec!['1', '3', '4', '5', '6']),
                '3' => Some(vec!['2', '5', '6']),
                '4' => Some(vec!['1', '2', '5', '7', '8']),
                '5' => Some(vec!['1', '2', '3', '4', '6', '7', '8', '9']),
                '6' => Some(vec!['2', '3', '5', '8', '9']),
                '7' => Some(vec!['4', '5', '8', '0']),
                '8' => Some(vec!['4', '5', '6', '7', '9', '0']),
                '9' => Some(vec!['5', '6', '8', '0']),
                '0' => Some(vec!['7', '8', '9']),
                _ => None,
            }
        }
    }
}
#[cfg(test)]
mod test {
    use pieces::*;

    use super::*;

    #[test]
    fn self_returning_moves() {
        let keys = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        let pieces: Vec<dyn Move> = vec![Rook{}, Knight{}, Bishop{}, Queen{}, King{}];


        for piece in pieces.iter() {
            for key in keys.iter() {
                let moves = piece.next(key).unwrap();
                for next in moves.iter() {
                    Rook::next(next).unwrap().contains(key);
                }
            }
        }
    }
}
