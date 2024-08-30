use clap::Parser;
use research_prover_market_tools::cli;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    let cmd = cli::Command::parse();
    match cmd.run().await {
        Ok(_) => {}
        Err(err) => {
            tracing::error!("{err:?}");
            std::process::exit(1);
        }
    }
}
