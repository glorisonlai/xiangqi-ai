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
}


fn main() {
    dioxus::desktop::launch(app);
}

const STARTING_POSITION="cheagaehc//1r5r/s1s1s1s1s///S1S1S1S1S/1R5R//CHEAGAEHC";

fn create_new_board(): &[Option<Piece>, 90] {
    let board = [None; 90];
    let mut index = 0;
    STARTING_POSITION.iter().
}

fn app(cx: Scope) -> Element {
    let game = create_new_board();
    cx.render(rsx! (
        div { background_color: "black", height: "10px", width: "10px" }
        div { background_color: "yellow", height: "10px", width: "10px" }
    ))
}
