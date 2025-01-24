#[derive(Debug, Clone)]
pub struct RecentTrade {
    pub tid: String,
    pub pair: String,
    pub price: String,
    pub amount: String,
    pub side: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct Kline {
    pub pair: String,
    pub time_frame: TimeFrame,
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
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
