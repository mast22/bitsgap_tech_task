const SQLX_ADDR: &str = ":memory:";

const CREATE_TABLE_SQL: &str = "CREATE TABLE IF NOT EXISTS price_data (
    symbol TEXT NOT NULL,
    lowest_price REAL NOT NULL,
    highest_price REAL NOT NULL,
    opening_price REAL NOT NULL,
    closing_price REAL NOT NULL,
    trading_unit_quote_currency REAL NOT NULL,
    trading_unit_base_currency REAL NOT NULL,
    trades INTEGER NOT NULL,
    start_time DATETIME NOT NULL,
    end_time DATETIME NOT NULL,
    PRIMARY KEY (symbol, start_time)
);";

pub struct Database {
    connection: sqlite::Connection,
}

impl Database {
    pub fn new() -> Self {
        let connection = sqlite::open(SQLX_ADDR).unwrap();
        connection.execute(CREATE_TABLE_SQL).unwrap();

        let database = Self { connection };
        tracing::info!("Database created");

        database
    }

    pub fn insert_price_data(&self, symbol: String, data: Vec<Vec<String>>) {
        tracing::info!("Saving {} data", symbol);

        for row in data {
            self.connection
                .execute(format!(
                    "
                INSERT INTO price_data (
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
}
