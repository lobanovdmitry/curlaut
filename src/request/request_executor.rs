use crate::auth::HttpAuthorization;
use crate::output::CurlautOutput;
use crate::request::request_spec::{
    HttpRequestBody, HttpRequestHeaders, HttpRequestMethod, HttpRequestSpec,
};
use anyhow::Context;
use reqwest::Method;
use reqwest::blocking::{Request, RequestBuilder, Response};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::io::Write;
use std::net::{IpAddr, ToSocketAddrs};
use url::Host;

pub fn execute(request_spec: HttpRequestSpec, io: &mut impl CurlautOutput) -> anyhow::Result<()> {
    log_target_host(&request_spec, io)?;
    log_request_starts(&request_spec, io)?;

    let http_client = reqwest::blocking::Client::new();

    // start build http request
    let mut rb = http_client.request(to_reqwest_method(&request_spec.method), request_spec.url);
    rb = add_headers(rb, request_spec.headers);
    rb = add_auth(rb, request_spec.authorization);
    rb = add_body(rb, request_spec.body);
    let request = rb.build().with_context(|| "Failed to build http request")?;

    // log request
    log_request_content(&request, io)?;

    // do execute request
    let response = http_client
        .execute(request)
        .with_context(|| "Failed to execute http request")?;

    // log response
    log_response(response, io)?;

    Ok(())
}

fn log_target_host(request: &HttpRequestSpec, io: &mut impl CurlautOutput) -> anyhow::Result<()> {
    let host = request
        .url
        .host()
        .ok_or_else(|| anyhow::anyhow!("No host specified"))?;
    let port = request
        .url
        .port_or_known_default()
        .ok_or_else(|| anyhow::anyhow!("No port specified"))?;
    match host {
        Host::Domain(host) => {
            writeln!(io.verbose(), "* Host {host}")?;
            let addr = (host, port)
                .to_socket_addrs()
                .with_context(|| format!("Failed to resolve host {host}"))?;
            let ips: (Vec<IpAddr>, Vec<IpAddr>) =
                addr.map(|addr| addr.ip()).partition(|ip| ip.is_ipv4());
            if !ips.0.is_empty() {
                writeln!(io.verbose(), "* IPv4: {:?}", ips.0)?;
            }
            if !ips.1.is_empty() {
                writeln!(io.verbose(), "* IPv6: {:?}", ips.1)?;
            }
        }
        Host::Ipv4(addr) => writeln!(io.verbose(), "* Host (IPv4): {addr}")?,
        Host::Ipv6(addr) => writeln!(io.verbose(), "* Host (ipv6): {addr}")?,
    }
    Ok(())
}

fn log_request_starts(
    request_spec: &HttpRequestSpec,
    io: &mut impl CurlautOutput,
) -> anyhow::Result<()> {
    writeln!(
        io.verbose(),
        "> {} {} HTTP/1.1",
        request_spec.method,
        request_spec.url
    )?;
    Ok(())
}

fn log_request_content(request: &Request, io: &mut impl CurlautOutput) -> anyhow::Result<()> {
    for (key, value) in request.headers() {
        writeln!(io.verbose(), "> {key:?}: {value:?}")?;
    }
    writeln!(io.verbose(), ">")?;
    let bytes_sent = request
        .body()
        .map(|body| body.as_bytes())
        .flatten()
        .map(|bytes| bytes.len())
        .unwrap_or(0);
    writeln!(io.verbose(), "}} [{bytes_sent} bytes data]")?;
    Ok(())
}

fn add_headers(mut rb: RequestBuilder, headers: HttpRequestHeaders) -> RequestBuilder {
    for (header_name, header_value) in headers.0.iter() {
        rb = rb.header(*header_name, *header_value);
    }
    rb
}

fn add_auth(rb: RequestBuilder, http_auth: Box<dyn HttpAuthorization>) -> RequestBuilder {
    rb.header(AUTHORIZATION, format!("Bearer {}", http_auth))
}

fn add_body(rb: RequestBuilder, body: HttpRequestBody) -> RequestBuilder {
    match body {
        HttpRequestBody::Empty => rb,
        HttpRequestBody::Json(s) => rb
            .header(CONTENT_TYPE, "application/json")
            .body(s.to_owned()),
    }
}

fn to_reqwest_method(method: &HttpRequestMethod) -> Method {
    match method {
        HttpRequestMethod::GET => Method::GET,
        HttpRequestMethod::POST => Method::POST,
        HttpRequestMethod::PUT => Method::PUT,
        HttpRequestMethod::DELETE => Method::DELETE,
    }
}

fn log_response(response: Response, io: &mut impl CurlautOutput) -> anyhow::Result<()> {
    let response_status = &response.status();
    writeln!(io.verbose(), "< HTTP/1.1 {response_status}")?;
    for (key, value) in response.headers() {
        writeln!(io.verbose(), "< {key:?}: {value:?}")?;
    }
    writeln!(io.verbose(), "<")?;
    writeln!(
        io.verbose(),
        "{{ [{} bytes data]",
        response.content_length().unwrap_or(0) // todo doesn't work
    )?;
    let text = response
        .text()
        .with_context(|| "Failed to parse response body")?;
    write!(io.common(), "{text}")?;
    Ok(())
}
