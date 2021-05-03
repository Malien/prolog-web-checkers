use crate::ts_type;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Sizes {
    pub cell: usize,
    pub row: usize,
    pub board: usize,
}

ts_type!(
    Sizes,
    TSSizes,
    "Sizes",
    r#"interface Sizes{
        cell: number
        row: number
        board:number
    }"#
);