use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionRequest {
    pub inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    pub mgn_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cxl: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
