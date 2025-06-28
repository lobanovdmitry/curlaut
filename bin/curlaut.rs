use clap::Parser;
use curlaut::cli::auth_mgmt as auth;
use curlaut::cli::clap_config::Cli;
use curlaut::cli::clap_config::MainCommand::{
    Config, DeleteRequest, GetRequest, PostRequest, PutRequest,
};
use curlaut::cli::request_executor as req;
use curlaut::output::stdio::CurlautStdOutput;
use curlaut::request::request_spec::HttpRequestMethod::{DELETE, GET, POST, PUT};
use std::io::Write;
use std::process::exit;

fn main() {
    let mut io = CurlautStdOutput::new();
    let cli = Cli::parse();
    let result = match cli.command {
        Config { command } => auth::execute_command(command, &mut io),
        GetRequest(args) => req::execute_request(GET, args, &mut io),
        PostRequest(args) => req::execute_request(POST, args, &mut io),
        PutRequest(args) => req::execute_request(PUT, args, &mut io),
        DeleteRequest(args) => req::execute_request(DELETE, args, &mut io),
    };
    match result {
        Ok(_) => {
            exit(0);
        }
        Err(err) => {
            writeln!(std::io::stderr(), "{err:?}").expect("Failed to write to stderr");
            exit(1);
        }
    }
}
