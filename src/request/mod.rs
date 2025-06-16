pub mod request_executor;
pub mod request_spec;

#[derive(thiserror::Error, Debug)]
pub enum RequestExecutionError {
    #[error("Failed to build http request: {cause}")]
    FailedToBuildHttpRequest { cause: reqwest::Error },
    #[error("Failed to execute http request: {cause}")]
    FailedToExecuteRequest { cause: reqwest::Error },
    #[error("Failed to read http response: {cause}")]
    FailedToReadResponseBody { cause: reqwest::Error },
}
