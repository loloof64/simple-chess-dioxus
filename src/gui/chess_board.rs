use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct ChessboardParams {
    pub position: String,
}


#[component]
pub fn Chessboard(params: ReadOnlySignal<ChessboardParams>) -> Element {
    let _position = params.read().clone().position;

    rsx!    {
        div {
            background_color: "blue",
            width: "90px",
            height: "90px",
        }
    }
}
