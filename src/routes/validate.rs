use actix_web::{Responder, get, web, HttpRequest, HttpResponse};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use urlencoding::decode;
use std::env;
use hex_slice::AsHex;

use crate::{types::Query, auth::generate_token};

#[get("/validate")]
pub async fn validate(req: HttpRequest) -> impl Responder {
    let params = req.query_string();
    if let Ok(query) = web::Query::<Query>::from_query(params) {
        if let Err(_) = validate_query(params, &query.hash) {
            return HttpResponse::Unauthorized().body("");
        }

        if let Ok(jwt) = generate_token(&query.user.id.to_string()) {
            return HttpResponse::Ok().body(jwt);
        } else {
            return HttpResponse::InternalServerError().body("");
        }
    } else {
        return HttpResponse::BadRequest().body("");
    }
}

fn validate_query(query: &str, hash: &str) -> Result<(), ()> {
    type HmacSha256 = Hmac<Sha256>;
    let token = env::var("TELEGRAM_TOKEN")
        .expect("The telegram token is necessary");
    
    let mut mac = HmacSha256::new_from_slice(b"WebAppData").unwrap();
    mac.update(token.as_bytes());

    let secret = mac.finalize().into_bytes();

    let query = decode(query).expect("UTF-8").into_owned();
    let mut query: Vec<_> = query.split("&").filter(|x| !x.starts_with("hash")).collect();

    query.sort();
    
    let query = query.join("\n");

    let mut mac = HmacSha256::new_from_slice(&secret).unwrap();
    mac.update(query.as_bytes());

    let res = mac.finalize().into_bytes();
    let final_hash = format!("{:02x}", res.as_hex())
        .replace("[", "")
        .replace("]", "")
        .split(" ")
        .collect::<Vec<&str>>()
        .join("");

    if final_hash == hash {
        return Ok(());
    } else {
        return Err(());
    }
}