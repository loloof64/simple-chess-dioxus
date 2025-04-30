use dioxus::prelude::*;

use dioxus_free_icons::icons::ld_icons::LdArrowUpDown;
use dioxus_free_icons::Icon;

mod gui;
use gui::chess_board::{Chessboard, ChessboardColors, ChessboardParams};

use chess::Board;

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
    let board_logic = use_signal(|| Board::default());
    rsx! {
        div {
            class: "main-zone",
            div {
                class: "buttons-zone",
                button {
                    class: "button",
                    onclick: move |_event| {board_reversed.set(!board_reversed());},
                    Icon {
                        class: "button-icon",
                        icon: LdArrowUpDown,
                    }
                },
                div {
                    class: "button turn-button",
                    background_color: "white",
                }
            }
            Chessboard{
                params: ChessboardParams {
                    size: "90vmin".to_string(),
                    position: board_logic.read().to_string(),
                    colors: ChessboardColors::default(),
                    reversed: board_reversed(),
                 }
            }
        }
    }
}
