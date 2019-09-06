//#![deny(warnings)]
use futures_util::TryStreamExt;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

mod handlers;

use ini::Ini;
use postgres::{Connection, TlsMode};
use postgres::params::{ConnectParams, Host};
use std::time::Duration;
use std::borrow::Borrow;

mod db;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, handlers::Error> {
    let res = match (req.method(), req.uri().path()) {
        (&Method::GET, "/tables/list") => Ok(Response::new(Body::from(
            serde_json::to_string(&handlers::tables::handle_get().await?)?))),
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),
        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),
        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/uppercase") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
        }
        (&Method::POST, "/echo/reversed") => {
            let whole_chunk = req.into_body().try_concat().await;
            let reversed_chunk =
                whole_chunk.map(move |chunk| chunk.iter().rev().cloned().collect::<Vec<u8>>())?;
            Ok(Response::new(Body::from(reversed_chunk)))
        }
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }

}

fn params() -> (ConnectParams, TlsMode<'static>) {
    let conf = Ini::load_from_file(".yabook").unwrap();
    let general = conf.general_section();


    let host = general.get("host").unwrap();
    let port = general.get("port").unwrap();
    let dbname = general.get("dbname").unwrap();
    let user = general.get("user").unwrap();
    let pass = general.get("pass").unwrap();

    return (ConnectParams::builder()
        .port(port.parse::<u16>().unwrap())
        .user(&user, Some(pass))
        .database(&dbname)
        .connect_timeout(Some(Duration::from_secs(30)))
        .build(Host::Tcp(host.clone()))
    , postgres::TlsMode::None
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
