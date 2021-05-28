extern crate libsmartcalc;

use libsmartcalc::app::SmartCalc;
use lazy_static::*;
use std::vec::Vec;
use urlencoding::decode;

lazy_static! {
    pub static ref SMART_CALC: SmartCalc = {
        let m = SmartCalc::default();
        m
    };
}

static DEFAULT_LANGUAGE: &str = "en";

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::header::CONTENT_TYPE;

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => {
            let mut language: Option<&str> = None;
            let mut code: Option<&str> = None;

            match req.uri().query() {
                Some(query) => {
                    let query_params = querystring::querify(&query);

                    for (key, value) in query_params {
                        match key.as_ref() {
                            "code" => code = Some(value),
                            "lang" => language = Some(value),
                             _ => ()
                        };
                    }
                },
                None => ()
            };

            if let Some(data) = code {
                let decoded = decode(data);

                let results = SMART_CALC.execute(match language {
                    Some(lang) => lang,
                    None => DEFAULT_LANGUAGE
                }, &decoded.unwrap());
                
                let mut response = Vec::new();

                for result in results {
                    match result {
                        Ok((_, ast)) => {
                            response.push(SMART_CALC.format_result(match language {
                                Some(lang) => lang,
                                None => DEFAULT_LANGUAGE
                            }, ast));

                            let mut response = Response::new(Body::from(response.join("")));
                            let headers = response.headers_mut();
                            headers.insert(CONTENT_TYPE, "text/plain;charset=UTF-8".parse().unwrap());

                            return Ok(response);
                        },
                        Err(error) => println!("Error : {}", error)
                    };
                };
            }

            Ok(Response::new(Body::from(
                "Try '/?code=yesterday'",
            )))
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
