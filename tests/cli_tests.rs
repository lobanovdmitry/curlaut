use assert_cmd::Command;
use httpmock::{Method, MockServer};
use predicates::prelude::predicate;

#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .default_bin_name("curlaut")
        .case("tests/cmd/config-*/*.toml");
}

#[test]
fn http_get() {
    // given
    let mock_server = MockServer::start();
    mock_server.mock(|when, then| {
        when.path("/api/v1/get").method(Method::GET);
        then.status(200).body("ok");
    });
    // when/then
    get_cmd()
        .args(vec!["GET", mock_server.url("/api/v1/get").as_str()])
        .env("HOME", "tests/cmd/config-list/fs")
        .assert()
        .success()
        .stdout("ok");
}

#[test]
fn http_get_verbose() {
    // given
    let mock_server = MockServer::start();
    mock_server.mock(|when, then| {
        when.path("/api/v1/get").method(Method::GET);
        then.status(200)
            .header("test-header", "test-header-value")
            .body("ok");
    });
    // when/then
    get_cmd()
        .args(vec!["GET", mock_server.url("/api/v1/get").as_str(), "-v"])
        .env("HOME", "tests/cmd/config-list/fs")
        .assert()
        .success()
        .stdout("ok")
        .stderr(predicate::str::contains("HTTP/1.1 200 OK"))
        .stderr(predicate::str::contains(
            r#"< "test-header": "test-header-value""#,
        ));
}

#[test]
fn http_post_verbose() {
    // given
    let mock_server = MockServer::start();
    mock_server.mock(|when, then| {
        when.path("/api/v1/post").method(Method::POST);
        then.status(201)
            .header("test-header", "test-header-value")
            .body("ok");
    });
    // when/then
    get_cmd()
        .args(vec!["POST", mock_server.url("/api/v1/post").as_str(), "-v"])
        .env("HOME", "tests/cmd/config-list/fs")
        .assert()
        .success()
        .stdout("ok")
        .stderr(predicate::str::contains("HTTP/1.1 201 Created"))
        .stderr(predicate::str::contains(
            r#"< "test-header": "test-header-value""#,
        ));
}

#[test]
fn http_put_verbose() {
    // given
    let mock_server = MockServer::start();
    mock_server.mock(|when, then| {
        when.path("/api/v1/put").method(Method::PUT);
        then.status(200)
            .header("test-header", "test-header-value")
            .body("ok");
    });
    // when/then
    get_cmd()
        .args(vec!["PUT", mock_server.url("/api/v1/put").as_str(), "-v"])
        .env("HOME", "tests/cmd/config-list/fs")
        .assert()
        .success()
        .stdout("ok")
        .stderr(predicate::str::contains("HTTP/1.1 200 OK"))
        .stderr(predicate::str::contains(
            r#"< "test-header": "test-header-value""#,
        ));
}

#[test]
fn http_delete_verbose() {
    // given
    let mock_server = MockServer::start();
    mock_server.mock(|when, then| {
        when.path("/api/v1/delete").method(Method::DELETE);
        then.status(204)
            .header("test-header", "test-header-value");
    });
    // when/then
    get_cmd()
        .args(vec![
            "DELETE",
            mock_server.url("/api/v1/delete").as_str(),
            "-v",
        ])
        .env("HOME", "tests/cmd/config-list/fs")
        .assert()
        .success()
        .stderr(predicate::str::contains("HTTP/1.1 204 No Content"))
        .stderr(predicate::str::contains(
            r#"< "test-header": "test-header-value""#,
        ));
}

fn get_cmd() -> Command {
    Command::cargo_bin("curlaut").unwrap()
}
