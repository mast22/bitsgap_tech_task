pub mod queries;

use chrono::{DateTime, Utc};

use crate::database::queries::*;
use crate::{client::models::Trade, common::models::Kline};

const SQLX_ADDR: &str = ":memory:";

pub struct Database {
    connection: sqlite::Connection,
}

impl Database {
    pub fn new() -> Self {
        let connection = sqlite::open(SQLX_ADDR).unwrap();
        connection.execute(CREATE_CANDLES_TABLE_SQL).unwrap();
        connection.execute(CREATE_TRADES_TABLE_SQL).unwrap();

        let database = Self { connection };
        tracing::info!("Database created");

        database
    }

    pub fn insert_recent_trades(&self, trades: &[Trade]) {
        let mut statement = self.connection.prepare(INSERT_TRADE_SQL).unwrap();

        self.connection.execute("BEGIN TRANSACTION;").unwrap();

        for trade in trades {
            statement.bind((1, trade.id.as_str())).unwrap();
            statement.bind((2, trade.symbol.as_str())).unwrap();
            statement.bind((3, trade.amount.as_str())).unwrap();
            statement.bind((4, trade.taker_side.as_str())).unwrap();
            statement.bind((5, trade.quantity.as_str())).unwrap();
            statement.bind((6, trade.create_time as i64)).unwrap();
            statement.bind((7, trade.price.as_str())).unwrap();
            statement.bind((8, trade.ts as i64)).unwrap();

            statement.next().unwrap();
            statement.reset().unwrap();
        }

        self.connection.execute("COMMIT;").unwrap();
    }

    pub fn retrieve_trades_in_interval(
        &self,
        start_time: &DateTime<Utc>,
        end_time: &DateTime<Utc>,
    ) -> Vec<Trade> {
        let mut statement = self
            .connection
            .prepare(RETRIEVE_TRADES_BY_TIMEFRAME_SQL)
            .unwrap();

        statement
            .bind((1, start_time.to_rfc3339().as_str()))
            .unwrap();
        statement.bind((2, end_time.to_rfc3339().as_str())).unwrap();

        let mut trades = Vec::new();
        while let Ok(sqlite::State::Row) = statement.next() {
            let trade = Trade {
                id: statement.read::<String, _>(0).unwrap(),
                symbol: statement.read::<String, _>(1).unwrap(),
                amount: statement.read::<String, _>(2).unwrap(),
                taker_side: statement.read::<String, _>(3).unwrap(),
                quantity: statement.read::<String, _>(4).unwrap(),
                create_time: statement.read::<i64, _>(5).unwrap() as u64,
                price: statement.read::<String, _>(6).unwrap(),
                ts: statement.read::<i64, _>(7).unwrap() as u64,
            };
            trades.push(trade);
        }

        trades
    }

    // TODO Generalize `insert_kline` and `insert_candles`
    pub fn insert_kline(&self, kline: &Kline) -> Result<(), sqlite::Error> {
        let mut statement = self.connection.prepare(INSERT_CANDLE_SQL).unwrap();

        statement.bind((1, kline.pair.as_str())).unwrap();
        statement.bind((2, kline.pair.as_str())).unwrap();
        statement.bind((3, kline.low)).unwrap();
        statement.bind((4, kline.high)).unwrap();
        statement.bind((5, kline.open)).unwrap();
        statement.bind((6, kline.close)).unwrap();
        statement
            .bind((7, kline.volume_bs.buy_quote + kline.volume_bs.sell_quote))
            .unwrap();
        statement
            .bind((8, kline.volume_bs.buy_base + kline.volume_bs.sell_base))
            .unwrap();
        statement.bind((9, 0)).unwrap();
        statement
            .bind((
                10,
                DateTime::from_timestamp(kline.utc_begin, 0)
                    .unwrap()
                    .to_rfc3339()
                    .as_str(),
            ))
            .unwrap();
        statement
            .bind((
                11,
                DateTime::from_timestamp(kline.utc_end, 0)
                    .unwrap()
                    .to_rfc3339()
                    .as_str(),
            ))
            .unwrap();

        statement.next()?;
        Ok(())
    }

    pub fn insert_candles(&self, symbol: String, data: Vec<Vec<String>>) {
        let mut statement = self.connection.prepare(INSERT_CANDLE_SQL).unwrap();

        for row in data {
            statement.bind((1, symbol.as_str())).unwrap();
            statement
                .bind((2, row[0].parse::<f64>().unwrap_or(0.0)))
                .unwrap();
            statement
                .bind((3, row[1].parse::<f64>().unwrap_or(0.0)))
                .unwrap();
            statement
                .bind((4, row[2].parse::<f64>().unwrap_or(0.0)))
                .unwrap();
            statement
                .bind((5, row[3].parse::<f64>().unwrap_or(0.0)))
                .unwrap();
            statement
                .bind((6, row[4].parse::<f64>().unwrap_or(0.0)))
                .unwrap();
            statement
                .bind((7, row[5].parse::<f64>().unwrap_or(0.0)))
                .unwrap();
            statement
                .bind((8, row[6].parse::<i64>().unwrap_or(0)))
                .unwrap();
            statement.bind((9, row[7].as_str())).unwrap();
            statement.bind((10, row[8].as_str())).unwrap();

            statement.next().unwrap();
            statement.reset().unwrap();
        }
    }
}
