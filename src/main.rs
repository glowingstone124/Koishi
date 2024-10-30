mod request;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, Error, HttpMessage};
use actix_web::guard::Get;
use reqwest::{Method};
use crate::request::{convert_actix_to_http, match_http_method, send_request};

const SOURCE: &str = "http://example.com";

async fn log_request(req: HttpRequest, body: web::Bytes) -> Result<impl Responder, Error> {
    let me= req.method();
    let response = send_request(
        SOURCE,
        match_http_method(me),
        convert_actix_to_http(req.headers()),
        Some(&String::from_utf8_lossy(&body)),
    ).await?;

    println!("Response from SOURCE: {:?}", response.body);

    let ip = if let Some(peer_addr) = req.peer_addr() {
        format!("{:?}", peer_addr)
    } else {
        "Unknown".to_string()
    };

    let header = format!(
        "[{}]<{}> -> {} with headers [{}]",
        ip,
        req.path(),
        req.method(),
        req.headers()
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value.to_str().unwrap_or("Invalid UTF-8")))
            .collect::<Vec<String>>()
            .join(" | ")
    );

    println!("{:?}", header);
    println!("Body: {:?}", String::from_utf8_lossy(&body));

    Ok("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>  {

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/{param:.*}")
                         .route(web::post().to(log_request))  // C
                         .route(web::get().to(log_request))   // R
                         .route(web::put().to(log_request))    // U
                         .route(web::delete().to(log_request)) // D
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
