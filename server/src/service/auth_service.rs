use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::controller::auth_controller::{LoginDto, RegisterDto};
use crate::service::oauth2_service::{OAuthTokens, User};

use super::oauth2_service;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expiration_time: u128,
}

impl AuthTokens {
    pub fn new(access_token: String, refresh_token: String, expires_in: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        // Convert oauth seconds to milliseconds
        let expiration_time = now + (expires_in as u128 * 1000);
        Self {
            access_token,
            refresh_token,
            expiration_time,
        }
    }
}

impl From<OAuthTokens> for AuthTokens {
    fn from(value: OAuthTokens) -> Self {
        let OAuthTokens {
            access_token,
            refresh_token,
            expires_in,
            ..
        } = value;
        AuthTokens::new(access_token, refresh_token.unwrap(), expires_in)
    }
}

pub async fn login(
    LoginDto { login, password }: &LoginDto<'_>,
    http_client: &reqwest::Client,
) -> anyhow::Result<AuthTokens> {
    let oauth_tokens = oauth2_service::login(login, password, http_client).await?;
    Ok(oauth_tokens.into())
}

pub async fn login_auth0(code: &str, http_client: &reqwest::Client) -> anyhow::Result<AuthTokens> {
    let oauth_tokens = oauth2_service::login_auth0(code, http_client).await?;
    Ok(oauth_tokens.into())
}

pub async fn register(
    dto: &RegisterDto<'_>,
    http_client: &reqwest::Client,
) -> anyhow::Result<AuthTokens> {
    let oauth_tokens = oauth2_service::register(&dto, &http_client).await?;
    Ok(oauth_tokens.into())
}

pub async fn refresh(
    auth_tokens: AuthTokens,
    http_client: &reqwest::Client,
) -> anyhow::Result<AuthTokens> {
    let OAuthTokens {
        access_token,
        expires_in,
        ..
    } = oauth2_service::refresh(&auth_tokens.refresh_token, http_client).await?;
    Ok(AuthTokens::new(
        access_token,
        auth_tokens.refresh_token,
        expires_in,
    ))
}

pub async fn get_user(access_token: &str, http_client: &reqwest::Client) -> anyhow::Result<User> {
    oauth2_service::get_user_by_token(&access_token, http_client).await
}