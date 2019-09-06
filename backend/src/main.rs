//#![deny(warnings)]
use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use url::form_urlencoded;

mod handlers;

use ini::Ini;
use postgres::{Connection, TlsMode};
use postgres::params::{ConnectParams, Host};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::Duration;

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
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/tables/list") => response_from_json(handlers::tables::handle_get().await?),
        (&Method::GET, "/booking/list") => {
            let query_map =
                get_query_map(&req).ok_or(serde_json::to_string(&Status::new("No query"))?)?;
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
            let query_map =
                get_query_map(&req).ok_or(serde_json::to_string(&Status::new("No query"))?)?;
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
            let query_map =
                get_query_map(&req).ok_or(serde_json::to_string(&Status::new("No query"))?)?;
            let id = get_param(&query_map, "id")?.parse::<i32>()?;
            response_from_json(handlers::booking::handle_item(id).await?)
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

fn params() -> (ConnectParams, TlsMode<'static>) {
    let conf = Ini::load_from_file(".yabook").unwrap();
    let general = conf.general_section();

    let host = general.get("host").unwrap();
    let port = general.get("port").unwrap();
    let dbname = general.get("dbname").unwrap();
    let user = general.get("user").unwrap();
    let pass = general.get("pass").unwrap();

    return (
        ConnectParams::builder()
            .port(port.parse::<u16>().unwrap())
            .user(&user, Some(pass))
            .database(&dbname)
            .connect_timeout(Some(Duration::from_secs(30)))
            .build(Host::Tcp(host.clone())),
        postgres::TlsMode::None,
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (params, sslmode) = params();
    let db = Connection::connect(params, sslmode).unwrap();

    db::init::init_db(&db);

    let addr = ([0, 0, 0, 0], 8080).into();
    let service = make_service_fn(|_| async { Ok::<_, handlers::Error>(service_fn(echo)) });
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
//    let res = db::table::insert_user(&db, "test_user");
//    if res.is_ok() {
//        println!("Ok insert");
//    } else {
//        println!("Err: {}", res.err().unwrap());
//    }
//    let user: Option<db::table::User> = db::table::find_user(&db, "test_user");
//    match user {
//        Some(name) => println!("User is present"),
//        None       => println!("User not found"),
//    }
    server.await?;
    Ok(())
}
