use dioxus::prelude::*;

mod app;
mod game;

fn main() {
    dioxus::desktop::launch(app::new_app);
}
