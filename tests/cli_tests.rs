use assert_cmd::Command;
use httpmock::MockServer;
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
        when.path("/api/v1/get");
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
        when.path("/api/v1/get");
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

fn get_cmd() -> Command {
    Command::cargo_bin("curlaut").unwrap()
}
