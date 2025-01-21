pub mod common;
pub mod client;

use std::sync::Arc;

use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::Mutex;

use bitsgrap_tech_task::proxy_server::{Proxy, ProxyServer};
use bitsgrap_tech_task::{Empty, RecentTrade, Kline, Vbs};
use sqlx::sqlite::SqlitePool;

pub mod bitsgrap_tech_task {
    tonic::include_proto!("bitsgrap_tech_task");
}

pub struct State {
    sqlite_pool: SqlitePool,
}

#[derive(Debug, Default)]
pub struct PoloniexProxy {}

#[tonic::async_trait]
impl Proxy for PoloniexProxy {
    async fn get_rt(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<RecentTrade>, Status> {
        let reply = RecentTrade {
            tid: String::from("sample"),
            pair: String::from("sample"),
            price: String::from("sample"),
            amount: String::from("sample"),
            side: String::from("sample"),
            timestamp: 3000,
        };

        Ok(Response::new(reply))
    }

    async fn get_kl(
        &self,
        _request: Request<Empty>, 
    ) -> Result<Response<Kline>, Status> { 
        let vbs = Vbs {
            buy_base: 100.0,
            sell_base: 100.0,
            buy_quote: 100.0,
            sell_quote: 100.0,
        };
        let reply = Kline {
            pair: String::from("sample"),
            time_frame: String::from("sample"),
            o: 100.0,
            h: 100.0,
            l: 100.0,
            c: 100.0,
            utc_begin: 3000,
            utc_end: 3000,
            volume_bs: Some(vbs),
        };

        Ok(Response::new(reply)) 
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:4000".parse()?;

    // The easiest db to setup
    let state = State {
        sqlite_pool: SqlitePool::connect("sqlite::memory:").await?
    };
    let shared_state = Arc::new(Mutex::new(state));

    Server::builder()
        .add_service(ProxyServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

