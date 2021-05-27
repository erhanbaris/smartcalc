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
                        Ok((tokens, ast)) => {
                            response.push(SMART_CALC.format_result(match language {
                                Some(lang) => lang,
                                None => DEFAULT_LANGUAGE
                            }, ast));

                            return Ok(Response::new(Body::from(response.join(""))));
                        },
                        Err(error) => println!("Error : {}", error)
                    };
                };
            }

            Ok(Response::new(Body::from(
                "Try '/?code=yesterday'",
            )))
        },

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive as we did with the above uppercase endpoint.
        // So here we do `.await` on the future, waiting on concatenating the full body,
        // then afterwards the content can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from("erhan")))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
