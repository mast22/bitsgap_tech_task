use chrono::{DateTime, Duration};

use crate::client::models::Trade;

use super::models::{Kline, TimeFrame, Vbs};

pub fn make_kline_from_trades(trades: Vec<Trade>, timeframe: TimeFrame) -> Option<Kline> {
    if trades.is_empty() {
        return None;
    }

    // Преобразуем минимальный `ts` в `DateTime<Utc>`
    let utc_begin = DateTime::from_timestamp(trades.iter().map(|t| t.ts as i64).min()?, 0)
        .ok_or("Invalid timestamp")
        .unwrap();

    // Вычисляем конец временного интервала
    let utc_end = match timeframe {
        TimeFrame::Minutes15 => utc_begin + Duration::minutes(15),
        TimeFrame::Hour => utc_begin + Duration::hours(1),
    };

    // Преобразуем DateTime<Utc> в Unix timestamp (i64)
    let utc_begin_timestamp = utc_begin.timestamp();
    let utc_end_timestamp = utc_end.timestamp();

    // Фильтруем трейды по временному интервалу
    let filtered_trades: Vec<_> = trades
        .iter()
        .filter(|t| t.ts as i64 >= utc_begin_timestamp && (t.ts as i64) < utc_end_timestamp)
        .collect();

    if filtered_trades.is_empty() {
        return None;
    }

    // Вычисляем open, close, high, low
    let open = filtered_trades.first()?.price.parse::<f64>().ok()?;
    let close = filtered_trades.last()?.price.parse::<f64>().ok()?;
    let high = filtered_trades
        .iter()
        .map(|t| t.price.parse::<f64>().unwrap_or(f64::NAN))
        .fold(f64::NEG_INFINITY, f64::max);
    let low = filtered_trades
        .iter()
        .map(|t| t.price.parse::<f64>().unwrap_or(f64::NAN))
        .fold(f64::INFINITY, f64::min);

    // Вычисляем объемы для buy и sell
    let mut buy_base = 0.0;
    let mut sell_base = 0.0;
    let mut buy_quote = 0.0;
    let mut sell_quote = 0.0;

    for trade in &filtered_trades {
        let amount = trade.quantity.parse::<f64>().unwrap_or(0.0); // Используем `quantity` вместо `amount`
        let price = trade.price.parse::<f64>().unwrap_or(0.0);
        match trade.taker_side.as_str() {
            "buy" => {
                buy_base += amount;
                buy_quote += amount * price;
            }
            "sell" => {
                sell_base += amount;
                sell_quote += amount * price;
            }
            _ => {}
        }
    }

    // Возвращаем Kline
    Some(Kline {
        pair: filtered_trades[0].symbol.clone(), // Используем `symbol` вместо `pair`
        timeframe,
        open,
        high,
        low,
        close,
        utc_begin: utc_begin_timestamp,
        utc_end: utc_end_timestamp,
        volume_bs: Vbs {
            buy_base,
            sell_base,
            buy_quote,
            sell_quote,
        },
    })
}
