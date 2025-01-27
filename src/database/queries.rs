pub const CREATE_CANDLES_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS candles (
    id TEXT PRIMARY KEY,
    symbol TEXT,
    lowest_price REAL,
    highest_price REAL,
    opening_price REAL,
    closing_price REAL,
    trading_unit_quote_currency REAL,
    trading_unit_base_currency REAL,
    trades INTEGER,
    start_time TEXT,
    end_time TEXT
);";

pub const CREATE_TRADES_TABLE_SQL: &str = "
CREATE TABLE IF NOT EXISTS trades (
    id TEXT PRIMARY KEY,
    symbol TEXT,
    amount TEXT,
    taker_side TEXT,
    quantity TEXT,
    create_time INTEGER,
    price TEXT,
    ts INTEGER
);";

pub const INSERT_CANDLE_SQL: &str = "
INSERT INTO candles (
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
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

pub const INSERT_TRADE_SQL: &str = "
INSERT INTO trades (
    id,
    symbol,
    amount,
    taker_side,
    quantity,
    create_time,
    price,
    ts
) VALUES (?, ?, ?, ?, ?, ?, ?, ?);";

pub const RETRIEVE_TRADES_BY_TIMEFRAME_SQL: &str = "
SELECT id, symbol, amount, taker_side, quantity, create_time, price, ts
FROM trades
WHERE create_time BETWEEN ? AND ?;";
