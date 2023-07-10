use clap::Parser;
use env_logger::{Builder, Env};
use iron::{Iron, IronResult, Request, Response, status};
use log::info;

/// Basic HTTP REST API Service
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

fn hello(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, world!")))
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args = Args::parse();

    info!("Starting server on port 127.0.0.1:{}", args.port);
    Iron::new(hello).http(("127.0.0.1", args.port)).expect("Cannot start server");
}
