use reqwest::Method;
use reqwest::Url;

use super::models::PoloniexRequest;
use super::models::KL;

const POLONIEX_ENDPOINT: &str = "https://api.poloniex.com/v3/market/";

pub struct PoloniexRest {
    session: reqwest::Client,
}

impl PoloniexRest {
    pub fn new() -> Self {
        Self {
            session: reqwest::Client::new(),
        }
    }

    fn build_request(&self, req: PoloniexRequest) -> reqwest::Request {
        let base_url = format!("{}{}", POLONIEX_ENDPOINT, req.as_ref());

        let url = match req {
            PoloniexRequest::Candles {
                symbol,
                interval,
                start_time,
                end_time,
            } => {
                let params = [
                    ("symbol", symbol),
                    ("interval", interval.as_ref().to_string()),
                    ("startTime", start_time.to_string()),
                    ("endTime", end_time.to_string()),
                ];

                Url::parse_with_params(&base_url, &params)
                    .expect("Failed to parse URL with parameters")
            }
        };

        self.session
            .request(Method::GET, url)
            .build()
            .expect("failed to call poloniex")
    }

    pub async fn request(&self, req: PoloniexRequest) -> Result<KL, reqwest::Error> {
        let build_request = self.build_request(req);
        let response = self.session.execute(build_request).await?;
        let text = response.text().await?;

        tracing::info!("Received KL");

        Ok(serde_json::from_str(&text).expect("Failed to deserialize kline json"))
    }
}

#[cfg(test)]
mod tests {
    use crate::client::models::PoloniexKLineIntervals;

    use super::*;

    #[test]
    fn check_build_request() {
        let req = PoloniexRequest::Candles {
            symbol: String::from("BTC_USDT_PERP"),
            interval: PoloniexKLineIntervals::Minute15,
            start_time: 10000,
            end_time: 10001,
        };
        let client = PoloniexRest::new();
        let request = client.build_request(req);

        assert_eq!(
            request.url().as_str(),
            format!(
                "{}{}",
                POLONIEX_ENDPOINT, "candles?symbol=BTC_USDT_PERP&interval=MINUTE_15"
            )
        )
    }
}
