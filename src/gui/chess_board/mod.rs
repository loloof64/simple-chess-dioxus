use std::rc::Rc;

use dioxus::prelude::*;

const WP: Asset = asset!("/assets/chess_vectors/Chess_plt45.svg");
const WN: Asset = asset!("/assets/chess_vectors/Chess_nlt45.svg");
const WB: Asset = asset!("/assets/chess_vectors/Chess_blt45.svg");
const WR: Asset = asset!("/assets/chess_vectors/Chess_rlt45.svg");
const WQ: Asset = asset!("/assets/chess_vectors/Chess_qlt45.svg");
const WK: Asset = asset!("/assets/chess_vectors/Chess_klt45.svg");

const BP: Asset = asset!("/assets/chess_vectors/Chess_pdt45.svg");
const BN: Asset = asset!("/assets/chess_vectors/Chess_ndt45.svg");
const BB: Asset = asset!("/assets/chess_vectors/Chess_bdt45.svg");
const BR: Asset = asset!("/assets/chess_vectors/Chess_rdt45.svg");
const BQ: Asset = asset!("/assets/chess_vectors/Chess_qdt45.svg");
const BK: Asset = asset!("/assets/chess_vectors/Chess_kdt45.svg");

#[derive(Debug, Clone)]
struct CellParams {
    color: String,
    coord_color: String,
    file_coord: Option<String>,
    rank_coord: Option<String>,
    value: Option<String>,
}

#[component]
fn Cell(params: ReadOnlySignal<CellParams>) -> Element {
    let coord_cell_proportion = 0.15;
    let coord_margin_proportion = 0.025;
    let mut node_ref: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let params = params.read().clone();
    let color = params.color;
    let coord_color = params.coord_color;
    let file_coord = params.file_coord;
    let rank_coord = params.rank_coord;
    let value = params.value;

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
            if value.is_some() {
                div {
                    position: "absolute",
                    width: "100%",
                    height: "100%",
                    img{
                        src: match value.clone().unwrap().as_str() {
                            "P" => WP,
                            "N" => WN,
                            "B" => WB,
                            "R" => WR,
                            "Q" => WQ,
                            "K" => WK,
                            "p" => BP,
                            "n" => BN,
                            "b" => BB,
                            "r" => BR,
                            "q" => BQ,
                            "k" => BK,
                            _ => panic!("illegal piece  '{}'", value.unwrap())
                        },
                        width: "100%",
                        height: "100%",
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
    pub reversed: bool,
}

#[component]
pub fn Chessboard(params: ReadOnlySignal<ChessboardParams>) -> Element {
    let params = params.read().clone();
    let position_str = params.position;
    let size = params.size;
    let colors = params.colors;
    let reversed = params.reversed;

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
                            file_coord: if row == 7 { Some(format!("{}", (char::from_u32(97 + if reversed {7-col} else {col} as u32).unwrap()).to_string())) } else {None},
                            rank_coord: if col == 7 { Some(format!("{}", if reversed {row+1} else {8 - row})) } else {None},
                            value: match position_to_values_array(position_str.clone()) {
                                Some(position_2) => position_2[if reversed {7-row} else  {row}][if reversed {7-col} else {col}].clone(),
                                _ => panic!("illegal position {}", position_str.clone()),
                            },
                        }
                    }
                }
            }
        }

    }
}

fn position_to_values_array(position_str: String) -> Option<[[Option<String>; 8]; 8]> {
    if position_str.is_empty() {
        return None;
    }
    let board_part = position_str.split_whitespace().nth(0);
    match board_part {
        Some(board_part) => {
            let board_part = board_part.to_string();
            let lines = board_part.split("/");
            if lines.clone().count() == 8 {
                let mut value: [[Option<String>; 8]; 8] = Default::default();

                for (i, line) in lines.enumerate() {
                    let mut j = 0;
                    for cell in line.chars() {
                        if let Ok(num) = cell.to_string().parse::<usize>() {
                            for _ in 0..num {
                                value[i][j] = None;
                                j += 1;
                            }
                        } else {
                            value[i][j] = Some(cell.to_string());
                            j += 1;
                        }
                    }
                }
                Some(value)
            } else {
                None
            }
        }
        _ => None,
    }
}
