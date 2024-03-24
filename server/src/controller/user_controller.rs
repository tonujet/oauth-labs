use anyhow::anyhow;
use rocket::{request, Request, Route};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::serde::json::Json;

use crate::ApiState;
use crate::service::{auth_service, jwt_service};
use crate::service::oauth2_service::User;

pub fn routes() -> Vec<Route> {
    routes![user]
}

#[get("/info")]
async fn user(user_guard: UserGuard) -> Json<User> {
    Json(user_guard.0)
}

struct UserGuard(User);

#[derive(Debug)]
enum ApiTokenError {
    Missing,
    Invalid,
}

#[async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = ApiTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        async fn get_user<'r>(request: &'r Request<'_>) -> anyhow::Result<User> {
            let header = request.headers().get_one("Authorization").ok_or(anyhow!("Invalid header"))?;
            let token = jwt_service::get_token_from_header(header)?;
            let state = request.rocket().state::<ApiState>().unwrap();
            let user = auth_service::get_user(token, &state.http_client).await?;
            Ok(user)
        }
        let user = get_user(request).await;
        match user {
            Ok(user) => Outcome::Success(UserGuard(user)),
            Err(_) => Outcome::Error((Status::Unauthorized, ApiTokenError::Missing))
        }
    }
}
