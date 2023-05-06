use anyhow::Result;
use clap::Parser;

mod app;
mod queue;
mod worker;

/// Simple application to create thumbnails using multiprocess
#[derive(Parser)]
#[command(about)]
struct Cli {
    /// Run app in the worker mode
    #[arg(short, long, default_value_t = false, group = "mode")]
    worker: bool,
    /// Run app in the automatic application mode (for testing)
    #[arg(short, long, default_value_t = false, group = "mode")]
    auto_app: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    if args.worker {
        let worker = worker::Worker::new();
        worker.run().await?;
    } else {
        let app = app::ConsoleApp::new();
        app.run(args.auto_app).await?;
    }

    Ok(())
}
