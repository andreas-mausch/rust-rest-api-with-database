extern crate router;

use clap::Parser;
use env_logger::{Builder, Env};
use iron::Iron;
use log::info;

use rust_rest_api_with_database::{connect, DATABASE, my_router};

/// Basic HTTP REST API Service
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to listen on
    #[arg(short, long, env, default_value_t = 8080)]
    port: u16,
    /// Database URL
    #[arg(short, long, env)]
    database_url: String,
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args = Args::parse();
    *DATABASE.lock().unwrap() = Some(connect(&args.database_url));

    info!("Starting server on port 127.0.0.1:{}", args.port);

    Iron::new(my_router()).http(("127.0.0.1", args.port)).expect("Cannot start server");
}
