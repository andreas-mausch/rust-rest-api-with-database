use std::sync::Mutex;

use diesel::{Connection, Insertable, PgConnection, Queryable, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use iron::{IronResult, Request, Response, status};
use iron::headers::ContentType;
use iron::mime::Attr::Charset;
use iron::mime::Mime;
use iron::mime::SubLevel::Json;
use iron::mime::TopLevel::Application;
use iron::mime::Value::Utf8;
use iron::modifiers::Header;
use log::info;
use router::{Router, router};
use serde::{Deserialize, Serialize};

use crate::schema::users::dsl::users;

pub mod schema;

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

pub fn connect(database_url: &str) -> PgConnection {
    info!("Connecting to database {}", database_url);
    PgConnection::establish(database_url).expect("Cannot connect to database")
}

pub static DATABASE: Mutex<Option<PgConnection>> = Mutex::new(None);

pub fn my_router() -> Router {
    router!(
        index: get "/" => hello,
        insert: get "/insert" => insert,
        query: get "/query" => query
    )
}
