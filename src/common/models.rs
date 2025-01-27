#[derive(Debug, Clone)]
pub struct Kline {
    pub pair: String,
    pub timeframe: TimeFrame,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub utc_begin: i64,
    pub utc_end: i64,
    pub volume_bs: Vbs,
}

#[derive(Debug, Clone)]
pub struct Vbs {
    pub buy_base: f64,
    pub sell_base: f64,
    pub buy_quote: f64,
    pub sell_quote: f64,
}

#[derive(Debug, Clone)]
pub enum TimeFrame {
    Minutes15,
    Hour,
}
