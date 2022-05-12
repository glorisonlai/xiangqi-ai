use dioxus::prelude::*;

enum PieceType {
    General,
    Advisor,
    Elephant,
    Horse,
    Chariot,
    Cannon,
    Soldier,
};

fn soldier_crossed_river(piece: Piece, position: u8) bool {
    if piece.piece_type != PieceType.Soldier {
        println!("Are you sure you called this function right?");
        false
    }

    (piece.piece_side == PieceSide.Red && position >= 45) ||
        (piece.piece_side == PieceSide.Black && position < 45)
}

fn get_piece_value(piece: Piece, position: u8) u8 {
    match (piece.piece_type) {
        General => 99,
        Advisor => 2,
        Elephant => 2,
        Soldier => soldier_crossed_river(piece, position).then(|| 2).unwrap_or(1),
        Horse => 4,
        Cannon => 5,
        Chariot => 10,
    }
};

enum PieceSide {
    Red,
    Black,
};

#[derive(Debug, Clone)]
struct Piece {
    piece_type: PieceType,
    piece_side: PieceSide,
    piece_tile: u8,
}


fn main() {
    dioxus::desktop::launch(app);
}

const STARTING_POSITION="cheagaehc//1r5r/s1s1s1s1s///S1S1S1S1S/1R5R//CHEAGAEHC";

fn create_new_board(): (&[Option<&Piece>, 90], &Vec<&Piece>, &Vec<&Piece>) {
    let mut board = [None; 90];
    let mut red_player_pieces = Vec<&Piece>::with_capacity(16);
    let mut black_player_pieces = Vec<&Piece>::with_capacity(16);
    let mut index: u8 = 0;
    let num_cols = 9;

    STARTING_POSITION
        .iter()
        .map(|token| {
            match token {
                '/' => {
                    index = (index + num_cols) / num_cols * num_cols
                },
                token if token.is_digit(10) => {
                    index += token.to_digit(10).unwrap();
                },
                _ => {
                    let piece = Piece {
                        piece_side: token.is_lowercase(),
                        piece_type: match token.to_lowercase().unwrap() {
                            'c' => PieceType.Chariot,
                            'h' => PieceType.Horse,
                            'e' => PieceType.Elephant,
                            'a' => PieceType.Advisor,
                            'g' => PieceType.General,
                            'r' => PieceType.Cannon,
                            's' => PieceType.Soldier,
                        },
                        piece_tile: index,
                    };

                    if token.is_lowercase() {
                        red_player_pieces.push(&piece);
                    } else {
                        black_player_pieces.push(&piece);
                    };

                    board[index] = &piece;

                    index += 1;
                },
            }
        })
    return &board, &whitePlayerPieces, &blackPlayerPieces;
}

fn generate_player_moves(board: &[Option<&Piece>], player_pieces)

fn app(cx: Scope) -> Element {
    let game = create_new_board();
    let mut red_turn = true;
    cx.render(rsx! (
        div { background_color: "black", height: "10px", width: "10px" }
        div { background_color: "yellow", height: "10px", width: "10px" }
    ))
}
