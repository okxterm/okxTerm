use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Ui {
    pub table: Table,
    pub input: Input,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Table {
    pub title: String,
    pub head: TableHead,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TableHead {
    pub tid: String,
    pub inst_id: String,
    pub pos: String,
    pub margin: String,
    pub avg_px: String,
    pub upl: String,
    pub mark_px: String,
    pub liq_px: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Input {
    pub close_position: ClosePosition,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClosePosition {
    pub title: String,
}
