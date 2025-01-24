// use super::models::{Kline, RecentTrade, TimeFrame, Vbs};

// pub fn make_kline_from_trades(trades: Vec<RecentTrade>, time_frame: TimeFrame) -> Option<Kline> {
//     if trades.is_empty() {
//         return None;
//     }

//     let utc_begin = trades.iter().map(|t| t.timestamp).min().unwrap();
//     let utc_end = match time_frame {
//         "1m" => utc_begin + 60 * 1_000_000_000,
//         "1h" => utc_begin + 3600 * 1_000_000_000,
//         "1d" => utc_begin + 86400 * 1_000_000_000,
//         _ => return None,
//     };

//     let filtered_trades: Vec<_> = trades
//         .into_iter()
//         .filter(|t| t.timestamp >= utc_begin && t.timestamp < utc_end)
//         .collect();

//     if filtered_trades.is_empty() {
//         return None;
//     }

//     let o = filtered_trades
//         .first()
//         .unwrap()
//         .price
//         .parse::<f64>()
//         .unwrap();
//     let c = filtered_trades
//         .last()
//         .unwrap()
//         .price
//         .parse::<f64>()
//         .unwrap();
//     let h = filtered_trades
//         .iter()
//         .map(|t| t.price.parse::<f64>().unwrap())
//         .fold(f64::MIN, f64::max);
//     let l = filtered_trades
//         .iter()
//         .map(|t| t.price.parse::<f64>().unwrap())
//         .fold(f64::MAX, f64::min);

//     let mut buy_base = 0.0;
//     let mut sell_base = 0.0;
//     let mut buy_quote = 0.0;
//     let mut sell_quote = 0.0;

//     for trade in &filtered_trades {
//         let amount = trade.amount.parse::<f64>().unwrap();
//         let price = trade.price.parse::<f64>().unwrap();
//         match trade.side.as_str() {
//             "buy" => {
//                 buy_base += amount;
//                 buy_quote += amount * price;
//             }
//             "sell" => {
//                 sell_base += amount;
//                 sell_quote += amount * price;
//             }
//             _ => {}
//         }
//     }

//     Some(Kline {
//         pair: filtered_trades[0].pair.clone(),
//         time_frame,
//         o,
//         h,
//         l,
//         c,
//         utc_begin,
//         utc_end,
//         volume_bs: Vbs {
//             buy_base,
//             sell_base,
//             buy_quote,
//             sell_quote,
//         },
//     })
// }
