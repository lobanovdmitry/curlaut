use clap::Parser;
use cli::auth_mgmt::execute_command;
use curlaut::cli;
use curlaut::cli::clap_config::Cli;
use curlaut::cli::clap_config::MainCommand::{
    Config, DeleteRequest, GetRequest, PostRequest, PutRequest,
};
use curlaut::cli::request_executor::execute_request;
use curlaut::output::stdio::CurlautStdOutput;
use curlaut::request::request_spec::HttpRequestMethod::{DELETE, GET, POST, PUT};
use std::io::Write;
use std::process::exit;

fn main() {
    let mut io = CurlautStdOutput::new();
    let cli = Cli::parse();
    let result = match cli.command {
        Config { command } => execute_command(command, &mut io),
        GetRequest(args) => execute_request(GET, args, &mut io),
        PostRequest(args) => execute_request(POST, args, &mut io),
        PutRequest(args) => execute_request(PUT, args, &mut io),
        DeleteRequest(args) => execute_request(DELETE, args, &mut io),
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
