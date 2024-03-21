use anyhow::anyhow;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;

use crate::config::config;

pub fn get_token_from_header(header: &str) -> anyhow::Result<&str> {
    let split: Vec<&str> = header.split_whitespace().collect();
    if split.len() != 2 {
        Err(anyhow!("Wrong header"))?
    }
    let token_type = split[0];

    if token_type != "Bearer" {
        Err(anyhow!("Wrong token type"))?
    }

    let token = split[1];
    Ok(token)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: usize,
    pub exp: usize,
    pub scope: String,
    pub gty: String,
    pub azp: String,
}

pub async fn decode_token(token: &str) -> anyhow::Result<TokenData<Claims>> {
    let mut file = File::open("public.pem").await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&config().OAUTH.AUDIENCE]);
    let token_message: TokenData<Claims> =
        jsonwebtoken::decode(&token, &DecodingKey::from_rsa_pem(&buffer)?, &validation)?;
    Ok(token_message)
}
