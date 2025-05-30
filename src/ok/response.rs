use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response<T> {
    pub code: String,
    pub msg: String,
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionResponse {
    pub adl: String,
    pub avail_pos: String,
    /// 开仓均价
    pub avg_px: String,
    pub base_bal: String,
    pub base_borrowed: String,
    pub base_interest: String,
    pub be_px: String,
    pub biz_ref_id: String,
    pub biz_ref_type: String,
    pub c_time: String,
    /// 占用保证金的币种
    pub ccy: String,
    pub cl_spot_in_use_amt: String,
    pub close_order_algo: Vec<Value>,
    pub delta_b_s: String,
    pub delta_p_a: String,
    pub fee: String,
    pub funding_fee: String,
    pub gamma_b_s: String,
    pub gamma_p_a: String,
    pub idx_px: String,
    pub imr: String,
    pub inst_id: String,
    pub inst_type: String,
    pub interest: String,
    pub last: String,
    /// 杠杆倍数
    pub lever: String,
    pub liab: String,
    pub liab_ccy: String,
    pub liq_penalty: String,
    // 预估强平价
    pub liq_px: String,
    pub margin: String,
    /// 最新标记价格
    pub mark_px: String,
    pub max_spot_in_use_amt: String,
    pub mgn_mode: String,
    pub mgn_ratio: String,
    pub mmr: String,
    pub non_settle_avg_px: String,
    pub notional_usd: String,
    pub opt_val: String,
    pub pending_close_ord_liab_val: String,
    pub pnl: String,
    /// 持仓数量
    pub pos: String,
    pub pos_ccy: String,
    pub pos_id: String,
    /// 方向
    pub pos_side: String,
    pub quote_bal: String,
    pub quote_borrowed: String,
    pub quote_interest: String,
    pub realized_pnl: String,
    pub settled_pnl: String,
    pub spot_in_use_amt: String,
    pub spot_in_use_ccy: String,
    pub theta_b_s: String,
    pub theta_p_a: String,
    pub trade_id: String,
    pub u_time: String,
    /// 	未实现收益（以标记价格计算）
    pub upl: String,
    /// 以最新成交价格计算的未实现收益，主要做展示使用，实际值还是 upl
    pub upl_last_px: String,
    /// 未实现收益率（以标记价格计算
    pub upl_ratio: String,
    /// 以最新成交价格计算的未实现收益率
    pub upl_ratio_last_px: String,
    pub usd_px: String,
    pub vega_b_s: String,
    pub vega_p_a: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionResponse {
    pub cl_ord_id: String,
    pub inst_id: String,
    pub pos_side: String,
    pub tag: String,
}
