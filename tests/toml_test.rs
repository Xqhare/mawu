use mawu::read::toml;

#[test]
fn name() {
    let path = "test.toml";
    let result = toml(path);
    assert!(result.is_ok());
}
