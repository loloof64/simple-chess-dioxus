use std::rc::Rc;

use dioxus::prelude::*;

#[derive(Debug, Clone)]
struct CellParams {
    color: String,
    coord_color: String,
    file_coord: Option<String>,
    rank_coord: Option<String>,
}

#[component]
fn Cell(params: ReadOnlySignal<CellParams>) -> Element {
    let coord_cell_proportion = 0.15;
    let coord_margin_proportion = 0.025;
    let mut node_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let color = params.read().clone().color;
    let coord_color = params.read().clone().coord_color;
    let file_coord = params.read().clone().file_coord;
    let rank_coord = params.read().clone().rank_coord;

    let font_size = use_resource(move || async move {
        match node_ref() {
            Some(ref node) => {
                let size = node.get_client_rect().await;
                let size = match size {
                    Ok(size) => size.size.width as f64,
                    _ => 0.0,
                };
                format!("{}px", size * coord_cell_proportion)
            }
            None => "0px".to_string(),
        }
    });
    let font_size = font_size().unwrap_or("0px".to_string());

    let margin_size = use_resource(move || async move {
        match node_ref() {
            Some(ref node) => {
                let size = node.get_client_rect().await;
                let size = match size {
                    Ok(size) => size.size.height as f64,
                    _ => 0.0,
                };
                format!("{}px", size * coord_margin_proportion)
            }
            None => "0px".to_string(),
        }
    });
    let margin_size = margin_size().unwrap_or("0px".to_string());

    let opp_margin_size = use_resource(move || async move {
        match node_ref() {
            Some(ref node) => {
                let size = node.get_client_rect().await;
                let size = match size {
                    Ok(size) => size.size.height as f64,
                    _ => 0.0,
                };
                format!("{}px", -size * coord_margin_proportion)
            }
            None => "0px".to_string(),
        }
    });
    let opp_margin_size = opp_margin_size().unwrap_or("0px".to_string());

    rsx! {
        div {
            onmounted: move |element| node_ref.set(Some(element.data())),
            position: "relative",
            width: "100%",
            height: "100%",
            background_color: color.as_str(),
            if file_coord.is_some() {
                div {
                    position: "absolute",
                    width: "100%",
                    height: "100%",
                    margin_left: margin_size.as_str(),
                    margin_bottom: margin_size.as_str(),
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "flex-end",
                    align_items: "flex-start",
                    span {
                        color: coord_color.as_str(),
                        font_size: font_size.as_str(),
                        match file_coord {
                            Some(ref coord) => &coord[0..1],
                            None => "",
                        }
                    }
                 }

            }
            if rank_coord.is_some() {
                div {
                    position: "absolute",
                    width: "100%",
                    height: "100%",
                    margin_left: opp_margin_size.as_str(),
                    margin_top: margin_size.as_str(),
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "flex-start",
                    align_items: "flex-end",
                    span {
                        color: coord_color.as_str(),
                        font_size: font_size.as_str(),
                        match rank_coord {
                            Some(ref coord) => &coord[0..1],
                            None => "",
                        }
                    }
                 }

            }
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
                            coord_color: if (row + col) % 2 == 0 { colors.black_cell.clone() } else { colors.white_cell.clone() },
                            file_coord: if row == 7 { Some(format!("{}", (char::from_u32(97 + col as u32).unwrap()).to_string())) } else {None},
                            rank_coord: if col == 7 { Some(format!("{}", 8 - row)) } else {None},
                        }
                    }
                }
            }
        }

    }
}
