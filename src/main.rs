extern crate router;

use std::string::ToString;
use std::sync::Mutex;

use Attr::Charset;
use clap::Parser;
use diesel::{Connection, Insertable, PgConnection, Queryable, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use env_logger::{Builder, Env};
use iron::{Iron, IronResult, Request, Response, status};
use iron::headers::ContentType;
use iron::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use iron::modifiers::Header;
use log::info;
use router::router;
use serde::{Deserialize, Serialize};
use SubLevel::Json;
use TopLevel::Application;
use Value::Utf8;

use crate::schema::users::dsl::users;

pub mod schema;

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

#[derive(Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::users, primary_key(id))]
struct User {
    #[diesel(deserialize_as = i64)]
    id: Option<i64>,
    name: String,
    year_of_birth: i64,
}

fn hello(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World.")))
}

fn insert(_: &mut Request) -> IronResult<Response> {
    let user = User {
        id: None,
        name: "David Fincher".to_string(),
        year_of_birth: 1962,
    };

    DATABASE.lock().unwrap().as_mut().and_then(|connection| {
        diesel::insert_into(users)
            .values(&user)
            .execute(connection)
            .ok()
    }).expect("Cannot insert user into database");

    Ok(Response::with((status::Ok, "New user inserted successfully")))
}

fn query(_: &mut Request) -> IronResult<Response> {
    let user = DATABASE.lock().unwrap().as_mut().and_then(|connection| {
        users.select(User::as_select())
            .load(connection)
            .ok()
    }).expect("Cannot query users from database");

    let json = serde_json::to_string(&user).unwrap();

    let header = Header(ContentType(Mime(Application, Json, vec![(Charset, Utf8)])));
    Ok(Response::with((status::Ok, json, header)))
}

fn connect(database_url: &str) -> PgConnection {
    info!("Connecting to database {}", database_url);
    PgConnection::establish(database_url).expect("Cannot connect to database")
}

static DATABASE: Mutex<Option<PgConnection>> = Mutex::new(None);

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args = Args::parse();
    *DATABASE.lock().unwrap() = Some(connect(&args.database_url));

    info!("Starting server on port 127.0.0.1:{}", args.port);

    let router = router!(
        index: get "/" => hello,
        insert: get "/insert" => insert,
        query: get "/query" => query
    );
    Iron::new(router).http(("127.0.0.1", args.port)).expect("Cannot start server");
}
