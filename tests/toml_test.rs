use mawu::read::toml;

#[test]
fn full_example_toml_parse() {
    let path = "test.toml";
    let result = toml(path);
    println!("{:#?}", result);
    assert!(result.is_ok());
}
