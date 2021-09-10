extern crate libsmartcalc;

use libsmartcalc::app::SmartCalc;
use libsmartcalc::token::ui_token::UiToken;
use std::vec::Vec;
use urlencoding::decode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::header::CONTENT_TYPE;
use serde_derive::Serialize;
use lazy_static::*;

lazy_static! {
    pub static ref SMART_CALC: SmartCalc = {
        SmartCalc::default()
    };
}

static DEFAULT_LANGUAGE: &str = "en";

#[derive(Serialize)]
struct ResultItem {
    pub status: bool,
    pub line: usize,
    pub result: Vec<UiToken>
}

#[derive(Serialize)]
struct QueryResult {
    pub status: bool,
    pub query: String,
    pub result: Vec<ResultItem>
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            let mut language: Option<&str> = None;
            let mut code: Option<&str> = None;

            if let Some(query) = req.uri().query() {
                let query_params = querystring::querify(&query);

                for (key, value) in query_params {
                    match key {
                        "code" => code = Some(value),
                        "lang" => language = Some(value),
                            _ => ()
                    };
                }
            }

            if let Some(data) = code {
                let decoded = decode(data);

                let results = SMART_CALC.execute(match language {
                    Some(lang) => lang,
                    None => DEFAULT_LANGUAGE
                }, decoded.unwrap());
                
                let mut response = Vec::new();

                for result in results.lines.iter() {
                    match result {
                        Some(result) => match &result.result {
                            Ok(line) => response.push(&line.output[..]),
                            Err(error) => println!("Error : {}", error)
                        },
                        None => ()
                    }
                };

                return match response.is_empty() {
                    true =>  Ok(Response::new(Body::from("No output"))),
                    false => {
                        let mut response = Response::new(Body::from(response.join("")));
                        let headers = response.headers_mut();
                        headers.insert(CONTENT_TYPE, "text/plain;charset=UTF-8".parse().unwrap());
                        Ok(response)
                    }
                }
            }

            Ok(Response::new(Body::from("Try '/?code=yesterday'")))
        },

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub async fn start_webserver() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.with_graceful_shutdown(shutdown_signal()).await?;

    Ok(())
}
