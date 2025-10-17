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
use std::fs::File;
use std::io::Read;
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
        body: get_body(args)?,
        authorization: Box::new(auth),
        http1: args.http1,
        timeout: args
            .timeout_millis
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(60)),
        insecure: args.insecure,
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

fn get_body(args: &HttpRequestArgs) -> anyhow::Result<HttpRequestBody> {
    args.json_body
        .as_ref()
        .map(|json| Ok(HttpRequestBody::Json(json.to_owned())))
        .or_else(|| {
            args.json_body_file
                .as_ref()
                .map(|path| get_body_from_file(path))
        })
        .unwrap_or(Ok(HttpRequestBody::Empty))
}

fn get_body_from_file(body_file_path: &str) -> anyhow::Result<HttpRequestBody> {
    File::open(body_file_path)
        .with_context(|| "Can't open body file")
        .and_then(|mut f| {
            let mut buffer = String::new();
            f.read_to_string(&mut buffer)
                .with_context(|| "Can't read body file")?;
            Ok(buffer)
        })
        .map(|buffer| HttpRequestBody::Json(buffer))
}
