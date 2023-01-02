use actix_web::{HttpRequest, web::{Data, Query}, Responder, get, HttpResponse};
use serde::Deserialize;

use crate::{types::AppState, auth::validate_token};

#[derive(Deserialize)]
struct ToSearch {
    user_id: String,
    search: String
}

#[get("/search")]
pub async fn search(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let to_search = Query::<ToSearch>::from_query(req.query_string()).unwrap();

    if let Some(jwt) = req.headers().get("Authorization") {
        let jwt = jwt.to_str().unwrap().replace("Bearer ", "");

        if let Err(_) = validate_token(&to_search.user_id, &jwt) {
            return HttpResponse::Unauthorized().body("");
        }

        if let Ok(file) = state.database.search(&to_search.user_id, &to_search.search).await {
            return HttpResponse::Ok()
                .body(serde_json::to_string(&file).unwrap());
        } else {
            return HttpResponse::InternalServerError().body("");
        }
    }

    HttpResponse::Unauthorized().body("")
}