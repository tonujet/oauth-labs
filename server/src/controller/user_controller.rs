use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::FromRequest;
use rocket::serde::json::Json;
use rocket::{request, Request, Route};

use crate::service::oauth_service::User;
use crate::service::{auth_service, jwt_service};

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
        let header = request.headers().get_one("Authorization");
        if let Some(header) = header {
            if let Ok(token) = jwt_service::get_token_from_header(header) {
                let user = auth_service::get_user(token).await;
                if let Ok(user) = user {
                    return Outcome::Success(UserGuard(user));
                }
            };
        }

        Outcome::Error((Status::Unauthorized, ApiTokenError::Missing))
    }
}
