use serde::{Deserialize, Serialize};

use crate::controller::auth_controller::{LoginDto, RegisterDto};
use crate::service::oauth_service::{OAuthTokens, User};

use super::oauth_service;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<OAuthTokens> for AuthTokens {
    fn from(value: OAuthTokens) -> Self {
        let OAuthTokens {
            access_token,
            refresh_token,
            ..
        } = value;
        AuthTokens {
            access_token,
            refresh_token: refresh_token.unwrap(),
        }
    }
}

pub async fn login(
    LoginDto {
        login: username,
        password,
    }: &LoginDto<'_>,
    http_client: &reqwest::Client,
) -> anyhow::Result<AuthTokens> {
    let oauth_tokens = oauth_service::login(username, password, http_client).await?;
    Ok(oauth_tokens.into())
}

pub async fn register(
    dto: &RegisterDto<'_>,
    http_client: &reqwest::Client,
) -> anyhow::Result<AuthTokens> {
    let oauth_tokens = oauth_service::register(&dto, &http_client).await?;
    Ok(oauth_tokens.into())
}

pub async fn refresh(
    auth_tokens: AuthTokens,
    http_client: &reqwest::Client,
) -> anyhow::Result<AuthTokens> {
    let OAuthTokens { access_token, .. } =
        oauth_service::refresh(&auth_tokens.refresh_token, http_client).await?;

    Ok(AuthTokens {
        access_token,
        refresh_token: auth_tokens.refresh_token,
    })
}

pub async fn get_user(access_token: &str) -> anyhow::Result<User> {
    oauth_service::get_user_by_token(&access_token).await
}
