use dioxus::prelude::*;

#[derive(Debug, Clone)]
pub struct ChessboardParams {
    pub position: String,
    pub size: String,
}


#[component]
pub fn Chessboard(params: ReadOnlySignal<ChessboardParams>) -> Element {
    let _position = params.read().clone().position;
    let size = params.read().clone().size;

    rsx!    {
        div {
            background_color: "blue",
            width: size.as_str(),
            height: size.as_str(),
        }
    }
}
