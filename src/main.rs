use Attr::Charset;
use clap::Parser;
use env_logger::{Builder, Env};
use iron::{Iron, IronResult, Request, Response, status};
use iron::headers::ContentType;
use iron::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use iron::modifiers::Header;
use log::info;
use SubLevel::Json;
use TopLevel::Application;
use Value::Utf8;

use serde::{Deserialize, Serialize};

/// Basic HTTP REST API Service
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    year_of_birth: u32,
}

fn hello(_: &mut Request) -> IronResult<Response> {
    let user = User {
        name: "David Fincher".to_string(),
        year_of_birth: 1962,
    };
    let json = serde_json::to_string(&user).unwrap();

    let header = Header(ContentType(Mime(Application, Json, vec![(Charset, Utf8)])));
    Ok(Response::with((status::Ok, json, header)))
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args = Args::parse();

    info!("Starting server on port 127.0.0.1:{}", args.port);
    Iron::new(hello).http(("127.0.0.1", args.port)).expect("Cannot start server");
}
