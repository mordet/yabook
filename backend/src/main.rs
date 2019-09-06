//#![deny(warnings)]
#[macro_use]
extern crate lazy_static;
use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use url::form_urlencoded;

mod handlers;

use postgres::{NoTls, Client, Transaction};
use r2d2_postgres::{PostgresConnectionManager};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::Duration;

use db::user::{insert_user, find_user, User};
use db::table::{insert_table, find_table, tables, Table};

mod db;

#[derive(Serialize, Deserialize)]
struct Status {
    status: String,
}

impl Status {
    fn new<T: Into<String>>(status: T) -> Status {
        Status {
            status: status.into(),
        }
    }
}

fn response_from_json<T: Serialize>(res: T) -> Result<Response<Body>, handlers::Error> {
    Ok(Response::new(Body::from(serde_json::to_string(&res)?)))
}

fn get_query_map(req: &Request<Body>) -> Option<HashMap<String, String>> {
    match req.uri().query() {
        Some(query) => Some(
            form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect(),
        ),
        None => None,
    }
}

fn get_param(map: &HashMap<String, String>, name: &str) -> Result<String, handlers::Error> {
    Ok(map
        .get(name)
        .ok_or(format!("Need param '{}'", name))?
        .clone())
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, handlers::Error> {
    println!("request {} {}", &req.method(), &req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/tables/list") => response_from_json(handlers::tables::handle_get().await?),
        (&Method::GET, "/booking/list") => {
            let query_map = get_query_map(&req).ok_or("No query")?;
            let table_name = get_param(&query_map, "table")?;
            let from = get_param(&query_map, "from")?.parse::<i64>()?;
            let to = get_param(&query_map, "to")?.parse::<i64>()?;
            response_from_json(handlers::booking::handle_get(&table_name, from, to).await?)
        },
        (&Method::POST, "/booking/create") => {
            let whole_chunk = req.into_body().try_concat().await?.into_bytes();
            let body = String::from_utf8(whole_chunk.to_vec())?;
            let req = serde_json::from_str(&body)?;
            response_from_json(handlers::booking::handle_create(req).await?)
        },
        (&Method::DELETE, "/booking") => {
            let query_map = get_query_map(&req).ok_or("No query")?;
            let id = get_param(&query_map, "id")?.parse::<i32>()?;
            response_from_json(handlers::booking::handle_delete(id).await?)
        },
        (&Method::POST, "/booking/join") => {
            let whole_chunk = req.into_body().try_concat().await?.into_bytes();
            let body = String::from_utf8(whole_chunk.to_vec())?;
            let req = serde_json::from_str(&body)?;
            response_from_json(handlers::booking::handle_join(req).await?)
        },
        (&Method::POST, "/booking/decline") => {
            let whole_chunk = req.into_body().try_concat().await?.into_bytes();
            let body = String::from_utf8(whole_chunk.to_vec())?;
            let req = serde_json::from_str(&body)?;
            response_from_json(handlers::booking::handle_decline(req).await?)
        },
        (&Method::POST, "/booking/item") => {
            let query_map = get_query_map(&req).ok_or("No query")?;
            let id = get_param(&query_map, "id")?.parse::<i32>()?;
            response_from_json(handlers::booking::handle_item(id).await?)
        },
        (&Method::GET, "/user") => {
            let query_map = get_query_map(&req).ok_or("No query")?;
            let name = get_param(&query_map, "name")?;
            response_from_json(handlers::user::handle_get(&name).await?)
        },
        (&Method::POST, "/user/create") => {
            let whole_chunk = req.into_body().try_concat().await?.into_bytes();
            let body = String::from_utf8(whole_chunk.to_vec())?;
            let req = serde_json::from_str(&body)?;
            response_from_json(handlers::user::handle_create(req).await?)
        },
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, handlers::Error> {
    return match handle(req).await {
        Ok(r) => Ok(r),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(serde_json::to_string(&Status::new(
                e.to_string(),
            ))?))?),
    };
}

fn tests(mut db: &mut Transaction) {
    let res = insert_user(&mut db, "'); DROP SCHEMA db;--");
    if res.is_ok() {
        println!("Ok insert");
    } else {
        println!("Err: {}", res.err().unwrap());
    }
    let user: Option<User> = find_user(&mut db, "test_user");
    match user {
        Some(_name) => println!("User is present"),
        None       => println!("User not found"),
    }

    let res = insert_table(&mut db, "test_table", "benua");
    if res.is_ok() {
        println!("Ok insert");
    } else {
        println!("Err: {}", res.err().unwrap());
    }
    let table: Option<Table> = find_table(&mut db, "test_table", "benua");
    match table {
        Some(_table) => println!("Table is present"),
        None        => println!("Table not found"),
    }
}

fn connect() -> r2d2::Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new(
        "host=localhost port=5433 user=postgres password=yabook".parse().unwrap(),
        NoTls,

    );
    r2d2::Pool::new(manager).unwrap()
}

lazy_static! {
    static ref CONNECTION: r2d2::Pool<PostgresConnectionManager<NoTls>> = connect();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut pool = CONNECTION.clone().get()?;
    let mut trx = pool.transaction()?;
    db::init::init_db(&mut trx);

    let addr = ([0, 0, 0, 0], 8080).into();
    let service = make_service_fn(|_| async { Ok::<_, handlers::Error>(service_fn(echo)) });
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    tests(&mut trx);

    server.await?;
    Ok(())
}
