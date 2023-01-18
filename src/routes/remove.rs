use actix_web::{Responder, post, HttpRequest, web::{Data, Query}, HttpResponse};
use serde::Deserialize;

use crate::{types::AppState, auth::validate_token};

#[derive(Deserialize)]
struct ToRemove {
    file_id: i32,
    user_id: String
}

#[post("/remove")]
pub async fn remove(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let to_remove = Query::<ToRemove>::from_query(req.query_string()).unwrap();

    if let Some(jwt) = req.headers().get("Authorization") {
        let jwt = jwt.to_str().unwrap().replace("Bearer ", "");

        if let Err(_) = validate_token(&to_remove.user_id, &jwt) {
            return HttpResponse::Unauthorized();
        }

        if let Err(_) = state.database.remove(&to_remove.user_id, to_remove.file_id).await {
            return HttpResponse::InternalServerError();
        }
    } else {
        return HttpResponse::Unauthorized();
    }

    HttpResponse::Ok()
}