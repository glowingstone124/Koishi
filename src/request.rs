use reqwest::header::{HeaderMap as ReqwestHeaderMap};
use reqwest::{Client, Method};
use std::error::Error;
use actix_web::http::header;
use actix_web::web::Header;

#[derive(Debug)]
pub struct Response {
    http_code: u16,
    pub(crate) body: String,
}

//usable
pub async fn send_request(
    url: &str,
    method: Method,
    headers: Option<ReqwestHeaderMap>,
    body: Option<&str>,
) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();

    let mut request_builder = client.request(method, url);

    if let Some(headers) = headers {
        request_builder = request_builder.headers(headers);
    }

    if let Some(body) = body {
        request_builder = request_builder.body(body.to_string());
    }

    let response = request_builder.send().await?;

    let response_data = Response {
        http_code: response.status().as_u16(),
        body: response.text().await?,
    };

    Ok(response_data)
}

pub fn match_http_method(input: &actix_web::http::Method) -> reqwest::Method {
    match input {
        &actix_web::http::Method::GET => reqwest::Method::GET,
        &actix_web::http::Method::POST => reqwest::Method::POST,
        &actix_web::http::Method::PUT => reqwest::Method::PUT,
        &actix_web::http::Method::DELETE => reqwest::Method::DELETE,
        _ => panic!("Unsupported HTTP method"),
    }
}

pub fn convert_actix_to_http(input: &header::HeaderMap) -> Option<ReqwestHeaderMap> {
    let mut headers = ReqwestHeaderMap::new();
    for (key, value) in input.iter() {
        let reqwest_key = reqwest::header::HeaderName::from_bytes(key.as_ref()).ok()?;
        let reqwest_value = value.to_str().ok()?.parse().ok()?;

        headers.insert(reqwest_key, reqwest_value);
    }
    Some(headers)
}

