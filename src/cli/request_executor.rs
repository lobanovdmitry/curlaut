use crate::auth::authenticator;
use crate::auth::authenticator::JwtToken;
use crate::cli::auth_config_file_path;
use crate::cli::clap_config::HttpRequestArgs;
use crate::keycloak::keycloak_registry::KeycloakRegistry;
use crate::output::CurlautOutput;
use crate::request::request_executor::execute;
use crate::request::request_spec::{
    HttpRequestBody, HttpRequestHeaders, HttpRequestMethod, HttpRequestSpec,
};
use anyhow::Context;
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

pub fn execute_request(
    method: HttpRequestMethod,
    args: HttpRequestArgs,
    io: &mut impl CurlautOutput,
) -> anyhow::Result<()> {
    if args.verbose {
        io.enable_verbose();
    }
    let auth_config_file_path = auth_config_file_path()?;
    let keycloak_registry = KeycloakRegistry::load_from_file(auth_config_file_path.as_path())?;
    let keycloak_config = keycloak_registry
        .get_default()
        .with_context(|| "No default keycloak config")?;
    let jwt = authenticator::get_jwt(&keycloak_config, io)?;
    let request = build_request_spec(&args, method, jwt)?;
    execute(request, io)?;
    Ok(())
}

fn build_request_spec(
    args: &HttpRequestArgs,
    method: HttpRequestMethod,
    auth: JwtToken,
) -> anyhow::Result<HttpRequestSpec> {
    Ok(HttpRequestSpec {
        url: Url::parse(&args.url)?,
        method,
        headers: parse_headers(args.headers.iter().map(|s| s.as_str()).collect()),
        body: match &args.json_body {
            None => HttpRequestBody::Empty,
            Some(body) => HttpRequestBody::Json(body),
        },
        authorization: Box::new(auth),
        http1: args.http1,
        timeout: args
            .timeout_millis
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(60)),
    })
}

fn parse_headers(headers: Vec<&str>) -> HttpRequestHeaders {
    let map = headers
        .iter()
        .map(|header| header.split_once(":"))
        .flatten()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.insert(key, value);
            acc
        });
    HttpRequestHeaders(map)
}
