pub mod aggregator;
pub mod client;
pub mod common;
pub mod database;

use crate::client::ws::PoloniexWs;
use client::{
    models::{PoloniexKLineIntervals, PoloniexRequest},
    rest::PoloniexRest,
};
use database::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct State {
    db: Database,
}

type SharedState = Arc<Mutex<State>>;

#[tokio::main]
async fn main() {
    // simple logging
    let _ = tracing_subscriber::fmt::init();
    tracing::info!("Running the system");

    let kline_start_time = 1733011200;
    let kline_end_time = 1735689599;

    // The easiest db to setup
    let state = State {
        db: Database::new(),
    };
    let shared_state: SharedState = Arc::new(Mutex::new(state));

    // DOGE_USDC and BCH_USDC requests result in error. They are not available in Poloniex
    let symbols: Vec<String> = vec!["BTC_USDT", "TRX_USDT", "ETH_USDT"]
        .iter()
        .map(|sym| sym.to_string())
        .collect();

    let ws = PoloniexWs::new().await.unwrap();
    ws.subscribe(vec!["trades".to_string()], symbols.clone())
        .await;
    ws.read_and_store(shared_state.clone());
    ws.init_heartbeat();

    for sym in symbols {
        let payload = PoloniexRequest::Candles {
            symbol: sym.clone(),
            interval: PoloniexKLineIntervals::Week1,
            start_time: kline_start_time,
            end_time: kline_end_time,
        };
        let rest = PoloniexRest::new();
        let historical_data = rest.request(payload).await.unwrap();

        {
            let locked_state = shared_state.lock().await;
            locked_state.db.insert_candles(sym, historical_data.data);
        }
    }
    tracing::info!("Finished downloading KLines");

    // loop to keep up the event loop
    loop {}
}
