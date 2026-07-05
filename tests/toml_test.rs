use mawu::read::toml;

#[test]
fn full_example_toml_parse() {
    let path = "data/toml/toml-test-data/test.toml";
    let result = toml(path);
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(parsed.is_object());
    let parsed = parsed.as_object().unwrap();
    assert_eq!(parsed.get("boolean").unwrap().as_boolean().unwrap(), &true);
    assert_eq!(parsed.get("bool2").unwrap().as_boolean().unwrap(), &false);
    assert_eq!(
        parsed
            .get("key.with.dots")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0
    );
    assert_eq!(
        parsed
            .get("single quoted key with spaces")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );
    assert_eq!(parsed.get("ʎǝʞ").unwrap().as_string().unwrap(), "value");
    assert_eq!(parsed.get("").unwrap().as_string().unwrap(), "blank");
    assert_eq!(
        parsed.get("another").unwrap().as_string().unwrap(),
        "# This is not a comment"
    );
    assert_eq!(parsed.get("key").unwrap().as_string().unwrap(), "value");
    assert_eq!(
        parsed.get("bare_key").unwrap().as_string().unwrap(),
        "value"
    );
    assert_eq!(parsed.get("-key").unwrap().as_string().unwrap(), "value");
    assert_eq!(parsed.get("1234").unwrap().as_string().unwrap(), "value");
    assert_eq!(
        parsed.get("quoted \"value\"").unwrap().as_string().unwrap(),
        "value"
    );
    let table = parsed.get("table").unwrap().as_object().unwrap();
    assert_eq!(table.len(), 3);
    assert_eq!(table.get("key").unwrap().as_string().unwrap(), "value");
    assert_eq!(table.get("another").unwrap().as_string().unwrap(), "value");
    assert_eq!(
        table
            .get("yet.\"another\" key")
            .unwrap()
            .as_string()
            .unwrap(),
        "a value"
    );

    let dotted_keys = parsed.get("dotted-keys").unwrap().as_object().unwrap();
    assert_eq!(dotted_keys.len(), 4);
    assert_eq!(
        dotted_keys.get("name").unwrap().as_string().unwrap(),
        "Orange"
    );
    let physical = dotted_keys.get("physical").unwrap().as_object().unwrap();
    assert_eq!(physical.len(), 2);
    assert_eq!(
        physical.get("color").unwrap().as_string().unwrap(),
        "orange"
    );
    assert_eq!(physical.get("shape").unwrap().as_string().unwrap(), "round");
    let site = dotted_keys.get("site").unwrap().as_object().unwrap();
    assert_eq!(site.len(), 1);
    assert_eq!(site.get("google.com").unwrap().as_boolean().unwrap(), &true);
    assert_eq!(
        dotted_keys
            .get("look_a_key")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0
    );

    let more_dotted_keys = parsed.get("more-dotted-keys").unwrap().as_object().unwrap();
    assert_eq!(more_dotted_keys.len(), 1);
    let fruit = more_dotted_keys.get("fruit").unwrap().as_object().unwrap();
    assert_eq!(fruit.len(), 3);
    assert_eq!(fruit.get("name").unwrap().as_string().unwrap(), "banana");
    assert_eq!(fruit.get("color").unwrap().as_string().unwrap(), "yellow");
    assert_eq!(fruit.get("flavor").unwrap().as_string().unwrap(), "banana");

    let strings = parsed.get("strings").unwrap().as_object().unwrap();
    assert_eq!(strings.len(), 14);
    assert_eq!(
        strings.get("basic").unwrap().as_string().unwrap(),
        "This is a basic string"
    );
    assert_eq!(
        strings.get("multiline").unwrap().as_string().unwrap(),
        "This is a \nmultiline string"
    );
    assert_eq!(
        strings.get("literal").unwrap().as_string().unwrap(),
        "C:\\This is a literal string"
    );
    assert_eq!(
        strings
            .get("multiline-literal")
            .unwrap()
            .as_string()
            .unwrap(),
        "This is a multiline \nliteral!"
    );
    assert_eq!(
        strings.get("str").unwrap().as_string().unwrap(),
        "I'm a string. \"You can quote me\". Name\tJosé\nLocation\tSF."
    );
    assert_eq!(
        strings.get("alt_str").unwrap().as_string().unwrap(),
        "I'm a string. \"You can quote me\". Name\tJosé\nLocation\tSF."
    );
}
// [strings] # Example of defining a table
// basic = "This is a basic string"
// multiline = """This is a \
//                multiline string"""
// literal = 'C:\This is a literal string'
// multiline-literal = '''This is a multiline \
//                        literal!'''
// str = "I'm a string. \"You can quote me\". Name\tJos\xE9\nLocation\tSF."
// alt_str = """I'm a string. \"You can quote me\". Name\tJos\xE9\nLocation\tSF."""
// quot15 = '''Here are fifteen quotation marks: """""""""""""""'''
//
// # apos15 = '''Here are fifteen apostrophes: ''''''''''''''''''  # INVALID
// apos15 = "Here are fifteen apostrophes: '''''''''''''''"
//
// # 'That,' she said, 'is still pointless.'
// str2 = ''''That,' she said, 'is still pointless.''''
// # What you see is what you get.
// winpath  = 'C:\Users\nodejs\templates'
// winpath2 = '\\ServerX\admin$\system32\'
// quoted   = 'Tom "Dubs" Preston-Werner'
// regex    = '<\i\c*\s*>'
//
// more = "\rtext\ffile\be"
#[test]
fn simple_toml() {
    let path = "data/toml/toml-test-data/simple.toml";
    let result = toml(path);
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert!(parsed.is_object());
    let parsed = parsed.as_object().unwrap();
    assert_eq!(parsed.get("boolean").unwrap().as_boolean().unwrap(), &true);
    assert_eq!(
        parsed
            .get("integer")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        42
    );
    assert_eq!(
        parsed
            .get("float")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        3.1415
    );
    assert_eq!(
        parsed.get("string").unwrap().as_string().unwrap(),
        "Hello, world!"
    );
    let ary = parsed.get("array").unwrap().as_array().unwrap();
    assert_eq!(ary.len(), 6);
    assert_eq!(ary[0].is_string(), true);
    assert_eq!(ary[0].as_string().unwrap(), "one");
    assert_eq!(ary[1].is_string(), true);
    assert_eq!(ary[1].as_string().unwrap(), "two");
    assert_eq!(ary[2].is_string(), true);
    assert_eq!(ary[2].as_string().unwrap(), "three");
    assert_eq!(ary[3].is_number(), true);
    assert_eq!(ary[3].as_number().unwrap().into_usize().unwrap(), 1);
    assert_eq!(ary[4].is_number(), true);
    assert_eq!(ary[4].as_number().unwrap().into_usize().unwrap(), 2);
    assert_eq!(ary[5].is_number(), true);
    assert_eq!(ary[5].as_number().unwrap().into_usize().unwrap(), 3);
    let table = parsed.get("table").unwrap().as_object().unwrap();
    assert_eq!(table.len(), 1);
    assert_eq!(table.get("key").unwrap().as_string().unwrap(), "value");
    let empty_table = parsed.get("empty_table").unwrap().as_object().unwrap();
    assert_eq!(empty_table.len(), 0);
}
