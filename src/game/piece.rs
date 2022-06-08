#[derive(PartialEq)]
pub enum PieceType {
    General,
    Advisor,
    Elephant,
    Horse,
    Chariot,
    Cannon,
    Soldier,
}

#[derive(PartialEq)]
pub enum PieceSide {
    Red,
    Black,
}

pub struct Piece {
    piece_type: PieceType,
    piece_side: PieceSide,
    piece_tile: usize,
}

fn soldier_crossed_river(piece: Piece, position: u8) -> bool {
    match piece.piece_type {
        PieceType::Soldier => {
            (piece.piece_side == PieceSide::Red && position >= 45)
                || (piece.piece_side == PieceSide::Black && position < 45)
        }
        _ => false,
    }
}

fn get_piece_value(piece: Piece, position: u8) -> u8 {
    match piece.piece_type {
        PieceType::General => 99,
        PieceType::Advisor => 2,
        PieceType::Elephant => 2,
        PieceType::Soldier => soldier_crossed_river(piece, position)
            .then(|| 2)
            .unwrap_or(1),
        PieceType::Horse => 4,
        PieceType::Cannon => 5,
        PieceType::Chariot => 10,
    }
}
