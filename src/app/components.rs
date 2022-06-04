fn app(cx: Scope) -> Element {
    let game = create_new_board();
    let mut red_turn = true;
    cx.render(rsx! (
        div { background_color: "black", height: "10px", width: "10px" }
        div { background_color: "yellow", height: "10px", width: "10px" }
    ))
}
