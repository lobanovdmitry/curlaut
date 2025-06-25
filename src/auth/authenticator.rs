use crate::auth::HttpAuthorization;
use crate::keycloak::keycloak_config::KeycloakConfig;
use crate::output::CurlautOutput;
use anyhow::{bail, Context};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Write;
use std::time::Duration;
use url::Url;

#[derive(Debug)]
pub struct JwtToken {
    pub token_value: String,
}

impl HttpAuthorization for JwtToken {
    fn get_authorization_value(&self) -> String {
        self.token_value.clone()
    }
}

impl Display for JwtToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_value)
    }
}

pub fn get_jwt(config: &KeycloakConfig, io: &mut impl CurlautOutput) -> anyhow::Result<JwtToken> {
    let http_client = reqwest::blocking::Client::new();
    let keycloak_url = &config.url;
    let token_url = build_token_url(keycloak_url, &config.realm)
        .with_context(|| format!("Failed to build auth token url to {}", config.alias))?;
    let mut params = HashMap::new();
    params.insert("grant_type", "password");
    params.insert("client_id", &config.client_id);
    params.insert("client_secret", &config.client_secret);
    params.insert("username", &config.username);
    params.insert("password", &config.password);
    params.insert("scope", "openid profile email");
    writeln!(io.verbose(), "Requesting JWT token using POST {token_url}")?;
    let get_jwt_request = http_client
        .post(token_url)
        .form(&params)
        .timeout(Duration::from_secs(60))
        .build()
        .with_context(|| "Failed to build auth request")?;
    let jwt_result = http_client
        .execute(get_jwt_request)
        .with_context(|| "Failed to execute auth request")?;
    if !jwt_result.status().is_success() {
        bail!(
            "Auth request failed with status: {}",
            jwt_result.status()
        );
    }
    let response: HashMap<String, Value> = jwt_result
        .json()
        .with_context(|| "Failed to parse auth response as JSON")?;
    let access_token = response
        .get("access_token")
        .with_context(|| "Missing access token")?;
    let access_token_value = access_token.as_str()
        .with_context(|| "Invalid access token value: must be a string")?;
    let token_value = access_token_value.to_owned();
    Ok(JwtToken { token_value })
}

fn build_token_url(keycloak_url: &Url, realm: &str) -> Result<Url, url::ParseError> {
    let mut token_url = keycloak_url.join("realms/")?;
    token_url = token_url.join(format!("{realm}/").as_str())?;
    token_url = token_url.join("protocol/openid-connect/token")?;
    Ok(token_url)
}
