use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

// WS models

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum PoloniexWsEvent {
    Trades {
        channel: String,
        data: Vec<Trade>,
    },
    Confirmation {
        channel: String,
        event: String,
        symbols: Vec<String>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub symbol: String,
    pub amount: String,
    pub taker_side: String,
    pub quantity: String,
    pub create_time: u64,
    pub price: String,
    pub id: String,
    pub ts: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "event")]
pub enum WebSocketMessage {
    Subscribe {
        channel: Vec<String>,
        symbols: Vec<String>,
    },
    Ping,
}

// REST models

#[derive(Debug, AsRefStr)]
pub enum PoloniexRequest {
    #[strum(serialize = "candles")]
    Candles {
        symbol: String,
        interval: PoloniexKLineIntervals,
        start_time: u64,
        end_time: u64,
    },
}

pub type RawKLHistory = Vec<Vec<String>>;

#[derive(Serialize, Debug, Deserialize)]
pub struct KL {
    code: u32,
    msg: String,
    pub data: RawKLHistory,
}

#[derive(Debug, AsRefStr)]
pub enum PoloniexKLineIntervals {
    #[strum(serialize = "MINUTE_1")]
    Minute1,
    #[strum(serialize = "MINUTE_5")]
    Minute5,
    #[strum(serialize = "MINUTE_15")]
    Minute15,
    #[strum(serialize = "MINUTE_30")]
    Minute30,
    #[strum(serialize = "HOUR_1")]
    Hour1,
    #[strum(serialize = "HOUR_2")]
    Hour2,
    #[strum(serialize = "HOUR_4")]
    Hour4,
    #[strum(serialize = "HOUR_12")]
    Hour12,
    #[strum(serialize = "DAY_1")]
    Day1,
    #[strum(serialize = "DAY_3")]
    Day3,
    #[strum(serialize = "WEEK_1")]
    Week1,
}
