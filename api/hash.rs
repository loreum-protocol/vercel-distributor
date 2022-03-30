use std::collections::HashMap;
use vercel_lambda::{
    error::VercelError,
    http::{StatusCode, Version},
    lambda, IntoResponse, Request, Response,
};
use sha2::{Sha256, Digest};

fn parse_query(querystring: &str) -> HashMap<&str, &str> {
    let iter = querystring.split('&').map(|pair| {
        pair.split_once('=').unwrap()
    });
    HashMap::from_iter(iter)
}

fn handler(req: Request) -> Result<impl IntoResponse, VercelError> {
    let query = parse_query(req.uri().query().unwrap());
    let hash = Sha256::digest(query.get("value").unwrap());
    let response = Response::builder()
        .version(Version::HTTP_2)
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(base16ct::lower::encode_string(&hash))?;

    Ok(response)
}

fn main() {
    lambda!(handler)
}
