use clap::Parser;
use iron::{Iron, IronResult, Request, Response, status};

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
    let args = Args::parse();
    Iron::new(hello).http(("127.0.0.1", args.port)).unwrap();
}
