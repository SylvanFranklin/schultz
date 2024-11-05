use clap::Parser;
use schultz::commands::bootstrap;
use schultz::Cli;
use schultz::Commands;
use schultz::Context;

extern crate core;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let ctx = Context::for_cli(&cli)?;
    match cli.command {
        Commands::Bootstrap {
            addr,
            bootnode,
            chainspec,
        } => bootstrap::run(ctx, addr, bootnode, chainspec).await,
    }
}