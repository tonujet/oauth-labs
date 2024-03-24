use std::fmt::{Display, Formatter};

use anyhow::anyhow;
use chrono::{DateTime, Local};
use rocket::serde::json::serde_json::json;
use rocket::serde::Deserialize;
use serde::Serialize;

use crate::config::config;
use crate::controller::auth_controller::RegisterDto;
use crate::service::jwt_service;

#[derive(Deserialize, Debug)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub expires_in: u32,
    pub token_type: String,
}

#[derive(Deserialize, Debug)]
pub struct OAuthTokensError {
    error: String,
    error_description: String,
}

impl Display for OAuthTokensError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OAuth token error: {}. More details: {}",
            self.error, self.error_description
        )
    }
}

impl std::error::Error for OAuthTokensError {}

pub async fn login(
    username: &str,
    password: &str,
    http_client: &reqwest::Client,
) -> anyhow::Result<OAuthTokens> {
    let endpoint = config().OAUTH.TOKEN.as_str();
    let res = http_client
        .post(endpoint)
        .json(&json!({
                "grant_type": "password",
                "scope": "offline_access profile openid email",
                "username": username,
                "password": password,
                "audience": config().OAUTH.API,
                "client_id": config().OAUTH.CLIENT_ID,
                "client_secret": config().OAUTH.CLIENT_SECRET
        }))
        .send()
        .await?;
    Ok(tokens_or_error(res).await?)
}

pub async fn login_auth0(code: &str, http_client: &reqwest::Client) -> anyhow::Result<OAuthTokens> {
    let endpoint = config().OAUTH.TOKEN.as_str();
    let res = http_client
        .post(endpoint)
        .json(&json!({
                "grant_type": "authorization_code",
                "client_id": config().OAUTH.CLIENT_ID,
                "client_secret": config().OAUTH.CLIENT_SECRET,
                "code": code,
                "redirect_uri": config().OAUTH.REDIRECT_URI,
        }))
        .send()
        .await?;
    Ok(tokens_or_error(res).await?)
}

// pub async fn user_info(access_token: &str) -> anyhow::Result<User> {
//     let endpoint =
// }

pub async fn register(
    dto: &RegisterDto<'_>,
    http_client: &reqwest::Client,
) -> anyhow::Result<OAuthTokens> {
    let access_token_endpoint = config().OAUTH.TOKEN.as_str();
    let res = http_client
        .post(access_token_endpoint)
        .json(&json!({
                "audience": config().OAUTH.API,
                "grant_type":"client_credentials",
                "client_id": config().OAUTH.CLIENT_ID,
                "client_secret": config().OAUTH.CLIENT_SECRET
        }))
        .send()
        .await?;
    let OAuthTokens { access_token, .. } = res.json().await?;

    let create_user_endpoint = config().OAUTH.API.clone().join("users")?;

    let RegisterDto {
        email,
        password,
        username,
    } = dto;
    let res = http_client
        .post(create_user_endpoint)
        .header("Authorization", format!("Bearer {access_token}"))
        .json(&json!({
                "email": email,
                "password": password,
                "nickname": username,
                "connection": "Username-Password-Authentication"
        }))
        .send()
        .await?;

    if !res.status().is_success() {
        Err(anyhow!(
            "Wrong input fields. Email can be already taken or password to weak"
        ))?
    }

    let tokens = login(email, password, &http_client).await?;
    return Ok(tokens);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub nickname: String,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub async fn get_user_by_token(access_token: &str, http_client: &reqwest::Client) -> anyhow::Result<User> {
    let user_id = get_user_id_from_token(&access_token).await?;
    let mut endpoint = config().OAUTH.API.clone().join("users")?;
    endpoint.path_segments_mut().unwrap().push(&user_id);
    let res = http_client
        .get(endpoint)
        .header("Authorization", format!("Bearer {access_token}"))
        .send()
        .await?;

    if !res.status().is_success() {
        Err(anyhow!(
            "Your access token not befit for any of the available authorization flows"
        ))?
    }

    let user: User = res.json().await?;
    Ok(user)
}

async fn get_user_id_from_token(token: &str) -> anyhow::Result<String> {
    let payload = jwt_service::decode_token(token).await?.claims;
    Ok(payload.sub)
}

pub async fn refresh(
    refresh_token: &str,
    http_client: &reqwest::Client,
) -> anyhow::Result<OAuthTokens> {
    let endpoint = config().OAUTH.TOKEN.as_str();
    let res = http_client
        .post(endpoint)
        .json(&json!({
                "grant_type": "refresh_token",
                "refresh_token": refresh_token,

                "client_id": config().OAUTH.CLIENT_ID,
                "client_secret": config().OAUTH.CLIENT_SECRET
        }))
        .send()
        .await?;
    Ok(tokens_or_error(res).await?)
}

async fn tokens_or_error(res: reqwest::Response) -> anyhow::Result<OAuthTokens> {
    if !res.status().is_success() {
        let err = res.json::<OAuthTokensError>().await?;
        Err(err)?
    } else {
        let tokens: OAuthTokens = res.json().await?;
        Ok(tokens)
    }
}