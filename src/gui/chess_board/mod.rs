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
            background_color: color.as_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChessboardColors {
    pub white_cell: String,
    pub black_cell: String,
}

impl Default for ChessboardColors {
    fn default() -> Self {
        Self {
            white_cell: "navajowhite".to_string(),
            black_cell: "peru".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChessboardParams {
    pub position: String,
    pub size: String,
    pub colors: ChessboardColors,
}

#[component]
pub fn Chessboard(params: ReadOnlySignal<ChessboardParams>) -> Element {
    let _position = params.read().clone().position;
    let size = params.read().clone().size;
    let colors = params.read().clone().colors;

    rsx! {
        div {
            display: "grid",
            grid_template: "repeat(8, 1fr) / repeat(8, 1fr)",
            width: size.as_str(),
            height: size.as_str(),

            for row in (0..8) {
                for col in (0..8) {
                    Cell {
                        params: CellParams {
                            color: if (row + col) % 2 == 0 { colors.white_cell.clone() } else { colors.black_cell.clone() },
                        }
                    }
                }
            }
        }

    }
}
