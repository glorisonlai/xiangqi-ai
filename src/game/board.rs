use crate::*;

const STARTING_POSITION: &str = "cheagaehc//1r5r/s1s1s1s1s///S1S1S1S1S/1R5R//CHEAGAEHC";

pub fn create_new_board() -> (
    &'static [Option<&'static game::piece::Piece>; 90],
    &'static Vec<&'static game::Piece>,
    &'static Vec<&'static game::Piece>,
) {
    let mut board = [None; 90];
    let mut red_player_pieces = Vec::<&'static game::Piece>::with_capacity(16);
    let mut black_player_pieces = Vec::<&'static game::Piece>::with_capacity(16);
    let mut index: usize = 0;
    let num_cols = 9;

    STARTING_POSITION.chars().map(|token| match token {
        '/' => index = (index + num_cols) / num_cols * num_cols,
        token if token.is_digit(10) => {
            index += token.to_digit(10).unwrap() as usize;
        }
        _ => {
            let piece = game::Piece {
                piece_side: if token.is_lowercase() {
                    game::PieceSide::Red
                } else {
                    game::PieceSide::Black
                },
                piece_type: match token.to_lowercase().next().unwrap() {
                    'c' => game::PieceType::Chariot,
                    'h' => game::PieceType::Horse,
                    'e' => game::PieceType::Elephant,
                    'a' => game::PieceType::Advisor,
                    'g' => game::PieceType::General,
                    'r' => game::PieceType::Cannon,
                    's' => game::PieceType::Soldier,
                },
                piece_tile: index,
            };

            if token.is_lowercase() {
                red_player_pieces.push(&piece);
            } else {
                black_player_pieces.push(&piece);
            };

            board[index] = Some(&piece);

            index += 1;
        }
    });

    return (&board, &red_player_pieces, &black_player_pieces);
}
