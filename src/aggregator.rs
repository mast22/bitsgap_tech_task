use std::sync::Arc;

use chrono::{Duration, Timelike, Utc};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration as TokioDuration},
};

use crate::{
    common::{models::TimeFrame, utils::make_kline_from_trades},
    State,
};

/// As stated per tech task requirement, the system should convert recent trades into klines
/// My approach to that would be to save all RTs, and then convert them into klines after specified time has passed

/// The `Aggregator` schedules tasks to run at regular intervals based on the specified timeframes.
/// Each task generates uniform Klines by aligning to fixed intervals of the timeframe.
///
/// For example, if the timeframe is 15 minutes, the aggregator will create a Kline at
/// 00, 15, 30, and 45 minutes of every hour.
pub struct Aggregator {
    timeframes: Vec<TimeFrame>,
    state: Arc<Mutex<State>>,
}

impl Aggregator {
    pub fn new(timeframes: Vec<TimeFrame>, state: Arc<Mutex<State>>) -> Self {
        Self { timeframes, state }
    }

    pub async fn run(&self) {
        let mut handles = vec![];

        for timeframe in &self.timeframes {
            let timeframe_clone = timeframe.clone();
            let state_clone = self.state.clone();
            let handle = tokio::spawn(async move {
                Self::calc(timeframe_clone, state_clone).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    async fn calc(timeframe: TimeFrame, state: Arc<Mutex<State>>) {
        loop {
            let duration = Self::calc_next_run_time(timeframe.clone());
            // Wait for next time it should run
            sleep(TokioDuration::from_secs(duration.num_seconds() as u64)).await;

            let now = Utc::now();
            let start_time = match timeframe {
                TimeFrame::Minutes15 => now - Duration::minutes(15),
                TimeFrame::Hour => now - Duration::hours(1),
            };
            let end_time = now;
            let state = state.lock().await;
            let trades = state.db.retrieve_trades_in_interval(&start_time, &end_time);
            let kline = make_kline_from_trades(trades, timeframe.clone());

            state.db.insert_kline(&kline.unwrap()).unwrap();
        }
    }

    fn calc_next_run_time(timeframe: TimeFrame) -> Duration {
        let now = Utc::now();
        let next_time = match timeframe {
            TimeFrame::Minutes15 => {
                let minutes = now.minute();
                let next_minute = (minutes / 15 + 1) * 15;
                now.with_minute(next_minute)
                    .unwrap()
                    .with_second(0)
                    .unwrap()
            }
            TimeFrame::Hour => {
                now.with_minute(0).unwrap().with_second(0).unwrap() + Duration::hours(1)
            }
        };
        next_time - now
    }
}
