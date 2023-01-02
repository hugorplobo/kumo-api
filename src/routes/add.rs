use actix_web::{post, Responder, web::{Query, Data}, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{database::File, auth::validate_token, AppState};

#[derive(Deserialize)]
struct User {
    user_id: String
}

#[post("/add")]
pub async fn add(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let file = Query::<File>::from_query(req.query_string()).unwrap();
    let user = Query::<User>::from_query(req.query_string()).unwrap();

    if let Some(jwt) = req.headers().get("Authorization") {
        let jwt = jwt.to_str().unwrap().replace("Bearer ", "");

        if let Err(_) = validate_token(&user.user_id, &jwt) {
            return HttpResponse::Unauthorized();
        }

        if let Err(_) = state.database.add(&user.user_id, &file).await {
            return HttpResponse::InternalServerError();
        }
    } else {
        return HttpResponse::Unauthorized();
    }

    HttpResponse::Ok()
}