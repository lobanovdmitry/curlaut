#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .default_bin_name("curlaut")
        .case("tests/cmd/*/*.toml");
}
