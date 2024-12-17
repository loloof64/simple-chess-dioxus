use dioxus::prelude::*;

use dioxus_free_icons::icons::ld_icons::LdArrowUpDown;
use dioxus_free_icons::Icon;

mod gui;
use gui::chess_board::{Chessboard, ChessboardColors, ChessboardParams};

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
    let mut board_reversed = use_signal(|| false);
    rsx! {
        div {
            class: "main-zone",
            div {
                class: "buttons-zone",
                button {
                    onclick: move |_event| {board_reversed.set(!board_reversed());},
                    Icon {
                        width: 20,
                        height: 20,
                        icon: LdArrowUpDown,
                    }
                }
            }
            Chessboard{
                params: ChessboardParams {
                    size: "90vmin".to_string(),
                    position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string(),
                    colors: ChessboardColors::default(),
                    reversed: board_reversed(),
                 }
            }
        }
    }
}
