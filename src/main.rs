
mod config;
mod rpc_server;
mod ledger;
mod rpc_handler;
mod rpc_router;
mod rpc_request;
mod rpc_response;
use anyhow::{Context, Result};
use env_logger::Env;
use tokio::runtime::Builder as TokioBuilder;
use clap::Parser;

use crate::{
    config::Config,
    rpc_server::RpcServer,
};

fn main() -> Result<()> {
    let config = Config::parse();

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Starting RPC server with config: {:?}", config);

    let rt = TokioBuilder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to build Tokio runtime")?;

    rt.block_on(async {
        let mut server = RpcServer::new(config);
        server.run().await
    })?;

    Ok(())
}