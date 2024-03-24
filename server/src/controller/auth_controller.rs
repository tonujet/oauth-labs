use rocket::response::status::Conflict;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{Route, State};
use rocket_validation::{Validate, Validated};
use serde::Serialize;

use crate::service::auth_service;
use crate::service::auth_service::AuthTokens;
use crate::ApiState;

pub fn routes() -> Vec<Route> {
    routes![login, logout, refresh, register, login_auth0]
}

#[post("/logout")]
fn logout() -> Redirect {
    Redirect::to("/")
}

#[post("/login", format = "json", data = "<dto>")]
async fn login(
    dto: Json<LoginDto<'_>>,
    state: &State<ApiState>,
) -> Result<Json<AuthTokens>, Conflict<String>> {
    let tokens = auth_service::login(&dto, &state.http_client).await;
    TokensWrapper(tokens).into()
}

#[post("/login/auth0/<code>")]
async fn login_auth0(
    code: &str,
    state: &State<ApiState>,
) -> Result<Json<AuthTokens>, Conflict<String>> {
    let tokens = auth_service::login_auth0(code, &state.http_client).await;
    TokensWrapper(tokens).into()
}

#[post("/register", format = "json", data = "<dto>")]
async fn register(
    dto: Validated<Json<RegisterDto<'_>>>,
    state: &State<ApiState>,
) -> Result<Json<AuthTokens>, Conflict<String>> {
    let tokens = auth_service::register(&dto.0, &state.http_client).await;
    TokensWrapper(tokens).into()
}

#[post("/refresh", format = "json", data = "<tokens>")]
async fn refresh(
    tokens: Json<AuthTokens>,
    state: &State<ApiState>,
) -> Result<Json<AuthTokens>, Conflict<String>> {
    let tokens = auth_service::refresh(tokens.0, &state.http_client).await;
    TokensWrapper(tokens).into()
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginDto<'r> {
    #[validate(length(min = 3, max = 50))]
    pub login: &'r str,

    #[validate(length(min = 8, max = 50))]
    pub password: &'r str,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct RegisterDto<'r> {
    #[validate(email)]
    pub email: &'r str,

    #[validate(length(min = 8, max = 50))]
    pub password: &'r str,

    #[validate(length(min = 3, max = 50))]
    pub username: &'r str,
}

struct TokensWrapper(anyhow::Result<AuthTokens>);

impl From<TokensWrapper> for Result<Json<AuthTokens>, Conflict<String>> {
    fn from(wrapper: TokensWrapper) -> Self {
        match wrapper.0 {
            Ok(tokens) => Ok(Json(tokens)),
            Err(e) => Err(Conflict(e.to_string())),
        }
    }
}

