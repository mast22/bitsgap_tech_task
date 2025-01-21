use reqwest::Method;
use reqwest::Url;
use serde::Deserialize;
use std::convert::AsRef;
use strum_macros::AsRefStr;
use serde::Serialize;

const POLONIEX_ENDPOINT: &str = "https://api.poloniex.com/v3/market/";

#[derive(Debug, AsRefStr)]
enum PoloniexRequest {
    #[strum(serialize = "candles")]
    Candles {
        symbol: String,
        interval: PoloniexKLineIntervals,
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct CandleData {
    #[serde(rename = "l")]
    pub lowest_price: String,
    #[serde(rename = "h")]
    pub highest_price: String,
    #[serde(rename = "o")]
    pub opening_price: String,
    #[serde(rename = "c")]
    pub closing_price: String,
    #[serde(rename = "amt")]
    pub trading_unit_quote_currency: String,
    #[serde(rename = "qty")]
    pub trading_unit_base_currency: String,
    #[serde(rename = "tC")]
    pub trades: String,
    #[serde(rename = "sT")]
    pub start_time: String,
    #[serde(rename = "cT")]
    pub end_time: String,
}

#[derive(Debug, AsRefStr)]
enum PoloniexKLineIntervals {
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

struct PoloniexRest {
    session: reqwest::Client,
    host: String,
}

impl PoloniexRest {
    pub fn new(host: String) -> Self {
        Self {
            session: reqwest::Client::new(),
            host,
        }
    }

    pub fn build_request(&self, req: PoloniexRequest) -> reqwest::Request {
        let base_url = format!("{}{}", self.host, req.as_ref());

        let url = match req {
            PoloniexRequest::Candles { symbol, interval } => {
                let params = [
                    ("symbol", symbol),
                    ("interval", interval.as_ref().to_string()),
                ];
    
                Url::parse_with_params(&base_url, &params)
                    .expect("Failed to parse URL with parameters")
            }
        };


        self.session.request(Method::GET, url).build().expect("failed to call poloniex")
    }

    pub async fn request(&self, req: PoloniexRequest) -> Result<CandleData, reqwest::Error> {
        let build_request = self.build_request(req);
        let response = self.session.execute(build_request).await?;
        let text = response.text().await?;

        Ok(serde_json::from_str(&text).unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_build_request() {
        let req = PoloniexRequest::Candles {
            symbol: String::from("BTC_USDT_PERP"),
            interval: PoloniexKLineIntervals::Minute15
        };
        let client = PoloniexRest::new(POLONIEX_ENDPOINT.to_string());
        let request = client.build_request(req);

        assert_eq!(request.url().as_str(), "https://api.poloniex.com/v3/market/candles?symbol=BTC_USDT_PERP&interval=MINUTE_15")
    }
}
