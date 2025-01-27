use chrono::{DateTime, Utc};

use crate::{client::models::Trade, common::models::Kline};

const SQLX_ADDR: &str = ":memory:";

const CREATE_CANDLES_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS candles (
    id TEXT PRIMARY KEY,
    symbol TEXT NOT NULL,
    lowest_price REAL NOT NULL,
    highest_price REAL NOT NULL,
    opening_price REAL NOT NULL,
    closing_price REAL NOT NULL,
    trading_unit_quote_currency REAL NOT NULL,
    trading_unit_base_currency REAL NOT NULL,
    trades INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL
);";

const CREATE_TRADES_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS trades (
    id TEXT PRIMARY KEY,
    symbol TEXT NOT NULL,
    amount TEXT NOT NULL,
    taker_side TEXT NOT NULL,
    quantity TEXT NOT NULL,
    create_time DATETIME NOT NULL,
    price TEXT NOT NULL,
    ts DATETIME NOT NULL
);";

const RETRIEVE_RECENT_TRADES_BY_TIMEFRAME: &str = "
    SELECT id, symbol, amount, taker_side, quantity, create_time, price, ts
    FROM trades
    WHERE create_time >= ? AND create_time < ?
    ORDER BY create_time ASC
";

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

    pub fn insert_candles(&self, symbol: String, data: Vec<Vec<String>>) {
        tracing::info!("Saving {} data", symbol);

        for row in data {
            self.connection
                .execute(format!(
                    "INSERT INTO candles (
                    symbol,
                    lowest_price, 
                    highest_price, 
                    opening_price, 
                    closing_price, 
                    trading_unit_quote_currency, 
                    trading_unit_base_currency, 
                    trades, 
                    start_time, 
                    end_time
                ) VALUES (
                    '{}', {}, {}, {}, {}, {}, {}, {}, {}, {}
                )",
                    symbol,
                    row[0].parse::<f64>().unwrap_or(0.0),
                    row[1].parse::<f64>().unwrap_or(0.0),
                    row[2].parse::<f64>().unwrap_or(0.0),
                    row[3].parse::<f64>().unwrap_or(0.0),
                    row[4].parse::<f64>().unwrap_or(0.0),
                    row[5].parse::<f64>().unwrap_or(0.0),
                    row[6].parse::<i64>().unwrap_or(0),
                    row[7],
                    row[8]
                ))
                .unwrap();
        }
    }

    pub fn insert_recent_trades(&self, trades: &[Trade]) {
        tracing::info!("Saving {} trades", trades.len());

        self.connection.execute("BEGIN TRANSACTION;").unwrap();

        for trade in trades {
            self.connection
                .execute(format!(
                    "INSERT INTO trades (
                        id, 
                        symbol, 
                        amount, 
                        taker_side, 
                        quantity, 
                        create_time, 
                        price, 
                        ts
                    ) VALUES (
                        '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}'
                    )",
                    trade.id,
                    trade.symbol,
                    trade.amount,
                    trade.taker_side,
                    trade.quantity,
                    trade.create_time,
                    trade.price,
                    trade.ts
                ))
                .unwrap();
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
            .prepare(RETRIEVE_RECENT_TRADES_BY_TIMEFRAME)
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

    pub fn insert_kline(&self, kline: &Kline) -> Result<(), sqlite::Error> {
        self.connection.execute(format!(
            "INSERT INTO candles (
                id,
                symbol,
                lowest_price,
                highest_price,
                opening_price,
                closing_price,
                trading_unit_quote_currency,
                trading_unit_base_currency,
                trades,
                start_time,
                end_time
            ) VALUES (
                '{}', '{}', {}, {}, {}, {}, {}, {}, {}, '{}', '{}'
            )",
            kline.pair,
            kline.pair,
            kline.low,
            kline.high,
            kline.open,
            kline.close,
            kline.volume_bs.buy_quote + kline.volume_bs.sell_quote,
            kline.volume_bs.buy_base + kline.volume_bs.sell_base,
            0,
            DateTime::from_timestamp(kline.utc_begin, 0)
                .unwrap()
                .to_rfc3339(),
            DateTime::from_timestamp(kline.utc_end, 0)
                .unwrap()
                .to_rfc3339()
        ))
    }
}
