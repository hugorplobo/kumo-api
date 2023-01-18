use actix_web::{Responder, get, HttpResponse, HttpRequest, web::{Data, Query}};
use serde::Deserialize;

use crate::{types::AppState, auth::validate_token};

#[derive(Deserialize)]
struct ToGet {
    file_id: i32,
    user_id: String
}

#[get("/get")]
pub async fn get(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let to_get = Query::<ToGet>::from_query(req.query_string()).unwrap();

    if let Some(jwt) = req.headers().get("Authorization") {
        let jwt = jwt.to_str().unwrap().replace("Bearer ", "");

        if let Err(_) = validate_token(&to_get.user_id, &jwt) {
            return HttpResponse::Unauthorized().body("");
        }

        if let Ok(file) = state.database.get(to_get.file_id, &to_get.user_id).await {
            return HttpResponse::Ok()
                .body(serde_json::to_string(&file).unwrap());
        } else {
            return HttpResponse::InternalServerError().body("");
        }
    }

    HttpResponse::Unauthorized().body("")
}