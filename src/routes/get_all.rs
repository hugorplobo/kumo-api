use actix_web::{Responder, get, HttpResponse, web::{Query, Data}, HttpRequest};
use serde::Deserialize;

use crate::{types::AppState, auth::validate_token};

#[derive(Deserialize)]
struct ToGet {
    user_id: String,
    page: i32
}

#[get("/get_all")]
pub async fn get_all(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let to_get = Query::<ToGet>::from_query(req.query_string()).unwrap();

    if let Some(jwt) = req.headers().get("Authorization") {
        let jwt = jwt.to_str().unwrap().replace("Bearer ", "");

        if let Err(_) = validate_token(&to_get.user_id, &jwt) {
            return HttpResponse::Unauthorized().body("");
        }

        if let Ok(file) = state.database.get_all(&to_get.user_id, to_get.page).await {
            return HttpResponse::Ok()
                .body(serde_json::to_string(&file).unwrap());
        } else {
            return HttpResponse::InternalServerError().body("");
        }
    }

    HttpResponse::Unauthorized().body("")
}