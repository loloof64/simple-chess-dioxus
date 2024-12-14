use dioxus::prelude::*;

mod gui;
use gui::chess_board::{Chessboard, ChessboardParams};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        MainZone {  }
    }
}

#[component]
fn MainZone() -> Element {
    rsx! {
        div {  
            class: "main-zone",
            Chessboard{
                params: ChessboardParams { position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string() }
            }
        }
    }
}
