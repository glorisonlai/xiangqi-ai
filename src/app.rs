use crate::*;

mod components;

pub fn new_app(cx: Scope) -> Element {
    let _game = game::board::create_new_board();
    let mut _red_turn = true;

    cx.render(rsx! (
        div { background_color: "black", height: "10px", width: "10px" }
        div { background_color: "yellow", height: "10px", width: "10px" }
    ))
}
