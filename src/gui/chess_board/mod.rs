use dioxus::prelude::*;


#[derive(Debug, Clone)]
struct CellParams {
    color: String,
}

#[component]
fn Cell(params: ReadOnlySignal<CellParams>) -> Element {
    let color = params.read().clone().color;
    rsx! {
        div {
            width: "100%",
            height: "100%",
            background_color: color
        }
    }
}


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
            display: "grid",
            grid_template: "repeat(8, 1fr) / repeat(8, 1fr)",
            width: size.as_str(),
            height: size.as_str(),

            for row in (0..8) {
                for col in (0..8) {
                    Cell {
                        params: CellParams {
                            color: if (row + col) % 2 == 0 { "navajowhite".to_string() } else { "peru".to_string() },
                        }
                    }
                }
            }
        }
            
    }
}
