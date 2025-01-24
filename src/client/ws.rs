use std::{sync::Arc, time::Duration};

use futures_util::{stream::StreamExt, SinkExt};
use tokio::{sync::Mutex, time::interval};
use tokio_tungstenite::connect_async;
use tungstenite::{error::Error, Message};

use super::models::{PoloniexWsEvent, WebSocketMessage};

const POLONIEX_ENDPOINT: &str = "wss://ws.poloniex.com/ws/public";
pub type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub struct PoloniexWs {
    stream: Arc<Mutex<WsStream>>,
}

impl PoloniexWs {
    pub async fn new() -> Result<Self, Error> {
        let (stream, _) = connect_async(POLONIEX_ENDPOINT).await?;

        Ok(Self {
            stream: Arc::new(Mutex::new(stream)),
        })
    }

    pub async fn subscribe(&self, channel: Vec<String>, symbols: Vec<String>) {
        let subscription_message =
            Self::create_subscribe_message(&channel.clone(), &symbols.clone());
        let json_message: String = serde_json::to_string(&subscription_message)
            .expect("failed to serialize subscription msg");

        let mut write = self.stream.lock().await;

        let _ = write.send(Message::Text(json_message.into())).await;

        tracing::info!("Sent subscription for {:?} {:?}", channel, symbols);
    }

    pub fn read_and_store(&self) {
        let stream = self.stream.clone();

        tokio::spawn(async move {
            let mut stream_lock = stream.lock().await;

            while let Some(msg) = stream_lock.next().await {
                match msg.expect("failed to read rt ws") {
                    Message::Text(data) => {
                        let data_string = data.to_string();
                        let ser_message: PoloniexWsEvent =
                            serde_json::from_str(&data_string).unwrap();

                        match ser_message {
                            PoloniexWsEvent::Trades {
                                channel: _,
                                data: _trades,
                            } => {
                                // tracing::info!("Received new trades {:?}", trades[0].id);
                            }
                            PoloniexWsEvent::Confirmation {
                                channel: _,
                                event: _,
                                symbols: _,
                            } => {
                                tracing::info!("Received confirmation on subscription");
                            }
                        }
                    }
                    _ => panic!("wrong message type received"),
                }
            }
        });
    }

    pub fn init_heartbeat(&self) {
        let ping_message = WebSocketMessage::Ping;
        let json_message: String =
            serde_json::to_string(&ping_message).expect("failed to serialize hearbeat ping msg");
        let stream = self.stream.clone();

        // Poloniex disconnects after 30 seconds with no ping
        let mut interval = interval(Duration::from_secs(29));

        tokio::spawn(async move {
            let mut stream_lock = stream.lock().await;

            interval.tick().await; // skip first
            loop {
                interval.tick().await;

                let _ = stream_lock
                    .send(Message::Text(json_message.clone().into()))
                    .await;

                tracing::info!("Sent heartbeat ping");
            }
        });
    }

    fn create_subscribe_message(channel: &Vec<String>, symbols: &Vec<String>) -> WebSocketMessage {
        WebSocketMessage::Subscribe {
            channel: channel.to_owned(),
            symbols: symbols.to_owned(),
        }
    }
}
