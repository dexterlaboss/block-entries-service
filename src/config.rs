
use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "block-entry-service", version, about)]
pub struct Config {
    /// Path to Solana validator ledger (RocksDB)
    #[arg(long, default_value = "/solana/ledger")]
    pub ledger_path: PathBuf,

    /// IP address to bind
    #[arg(long, default_value = "0.0.0.0")]
    pub bind_addr: String,

    /// Port to listen on
    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}