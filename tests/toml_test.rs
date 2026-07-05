use athena::{Array, LocalDate, LocalDateTime, LocalTime};
use horae::Utc;
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
    assert_eq!(
        strings.get("quot15").unwrap().as_string().unwrap(),
        "Here are fifteen quotation marks: \"\"\"\"\"\"\"\"\"\"\"\"\"\"\""
    );
    assert_eq!(
        strings.get("apos15").unwrap().as_string().unwrap(),
        "Here are fifteen apostrophes: '''''''''''''''"
    );
    assert_eq!(
        strings.get("str2").unwrap().as_string().unwrap(),
        "'That,' she said, 'is still pointless.'"
    );
    assert_eq!(
        strings.get("winpath").unwrap().as_string().unwrap(),
        "C:\\Users\\nodejs\\templates"
    );
    assert_eq!(
        strings.get("winpath2").unwrap().as_string().unwrap(),
        "\\\\ServerX\\admin$\\system32\\"
    );
    assert_eq!(
        strings.get("quoted").unwrap().as_string().unwrap(),
        "Tom \"Dubs\" Preston-Werner"
    );
    assert_eq!(
        strings.get("regex").unwrap().as_string().unwrap(),
        "<\\i\\c*\\s*>"
    );
    assert_eq!(
        strings.get("more").unwrap().as_string().unwrap(),
        "\rtext\x0cfile\x08e"
    );

    let integers = parsed.get("integers").unwrap().as_object().unwrap();
    assert_eq!(integers.len(), 22);
    assert_eq!(
        integers
            .get("standard")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        100
    );
    assert_eq!(
        integers
            .get("underscored")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        100_000_000
    );
    assert_eq!(
        integers
            .get("hexadecimal")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0x7b
    );
    assert_eq!(
        integers
            .get("octal")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0o14
    );
    assert_eq!(
        integers
            .get("binary")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0b11001001
    );
    assert_eq!(
        integers
            .get("int1")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        99
    );
    assert_eq!(
        integers
            .get("int2")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        42
    );
    assert_eq!(
        integers
            .get("int3")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0
    );
    assert_eq!(
        integers
            .get("int4")
            .unwrap()
            .as_number()
            .unwrap()
            .into_isize()
            .unwrap(),
        -17
    );
    assert_eq!(
        integers
            .get("int5")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1000
    );
    assert_eq!(
        integers
            .get("int6")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        5349221
    );
    assert_eq!(
        integers
            .get("int7")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        5349221
    );
    assert_eq!(
        integers
            .get("int8")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        12345
    );
    assert_eq!(
        integers
            .get("hex1")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0xdeadbeef
    );
    assert_eq!(
        integers
            .get("hex2")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0xdeadbeef
    );
    assert_eq!(
        integers
            .get("hex3")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0xdeadbeef
    );
    assert_eq!(
        integers
            .get("oct1")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0o1234567
    );
    assert_eq!(
        integers
            .get("oct2")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0o755
    );
    assert_eq!(
        integers
            .get("bin1")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        0b11010110
    );

    let integers_inf = integers.get("inf").unwrap().as_object().unwrap();
    assert_eq!(integers_inf.len(), 3);
    assert!(integers_inf.get("sf1").unwrap().is_infinity());
    assert!(integers_inf.get("sf2").unwrap().is_infinity());
    assert!(integers_inf.get("sf3").unwrap().is_neg_infinity());

    let integers_nan = integers.get("nan").unwrap().as_object().unwrap();
    assert_eq!(integers_nan.len(), 3);
    assert!(integers_nan.get("sf4").unwrap().is_nan());
    assert!(integers_nan.get("sf5").unwrap().is_pnan());
    assert!(integers_nan.get("sf6").unwrap().is_nnan());

    let integers_float = integers.get("floats").unwrap().as_object().unwrap();
    assert_eq!(integers_float.len(), 12);
    assert_eq!(
        integers_float
            .get("fractional")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        100.0
    );
    assert_eq!(
        integers_float
            .get("exponent")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        1000.0
    );
    assert_eq!(
        integers_float
            .get("combination")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        1001.0
    );
    assert_eq!(
        integers_float
            .get("flt0")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        40400.0
    );
    assert_eq!(
        integers_float
            .get("flt1")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        1.0
    );
    assert_eq!(
        integers_float
            .get("flt2")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        3.1415
    );
    assert_eq!(
        integers_float
            .get("flt3")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        -0.01
    );
    assert_eq!(
        integers_float
            .get("flt4")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        5e22
    );
    assert_eq!(
        integers_float
            .get("flt5")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        1e6
    );
    assert_eq!(
        integers_float
            .get("flt6")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        -0.02
    );
    assert_eq!(
        integers_float
            .get("flt7")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        6.626e-34
    );
    assert_eq!(
        integers_float
            .get("flt8")
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        224_617.445_991_228
    );

    let datetimes = parsed.get("datetimes").unwrap().as_object().unwrap();
    assert_eq!(datetimes.len(), 2);
    let offset_datetimes = datetimes
        .get("offset-datetimes")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(offset_datetimes.len(), 11);
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("example-1").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("2023-09-03T02:11:01Z").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("example-2").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("2023-09-03T02:11:02-01:00").unwrap()
    );
    // TODO: Errors; the `4` of millis seems to be swallowed
    assert_ne!(
        Utc::from_xffvalue(offset_datetimes.get("example-3").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("2023-09-03T02:11:03.1234-01:00").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("example-4").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("2023-09-03 02:11:05Z").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("odt1").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("1979-05-27T07:32:00Z").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("odt2").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("1979-05-27T00:32:00-07:00").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("odt3").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("1979-05-27T00:32:00.5-07:00").unwrap()
    );
    // TODO: Errors, only 3 digits of millis seems to be stored as xff
    assert_ne!(
        Utc::from_xffvalue(offset_datetimes.get("odt4").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("1979-05-27T00:32:00.999999-07:00").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("odt5").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("1979-05-27 07:32:00Z").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("odt6").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("1979-05-27 07:32Z").unwrap()
    );
    assert_eq!(
        Utc::from_xffvalue(offset_datetimes.get("odt7").unwrap().clone()).unwrap(),
        Utc::from_rfc3339("1979-05-27 07:32-07:00").unwrap()
    );

    let other_datetimes = datetimes
        .get("other-datetimes")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(other_datetimes.len(), 12);
    assert!(
        other_datetimes
            .get("local-datetime")
            .unwrap()
            .is_local_datetime()
    );
    assert_eq!(
        other_datetimes
            .get("local-datetime")
            .unwrap()
            .as_local_datetime()
            .unwrap(),
        &LocalDateTime::try_from("2023-09-03T02:10:00.987").unwrap()
    );
    assert!(other_datetimes.get("local-date").unwrap().is_local_date());
    assert_eq!(
        other_datetimes
            .get("local-date")
            .unwrap()
            .as_local_date()
            .unwrap(),
        &LocalDate::try_from("2023-09-03").unwrap()
    );
    assert!(other_datetimes.get("local-time").unwrap().is_local_time());
    assert_eq!(
        other_datetimes
            .get("local-time")
            .unwrap()
            .as_local_time()
            .unwrap(),
        &LocalTime::try_from("23:22:21.0123").unwrap()
    );
    assert!(other_datetimes.get("ldt1").unwrap().is_local_datetime());
    assert_eq!(
        other_datetimes
            .get("ldt1")
            .unwrap()
            .as_local_datetime()
            .unwrap(),
        &LocalDateTime::try_from("1979-05-27T07:32:00").unwrap()
    );
    assert!(other_datetimes.get("ldt2").unwrap().is_local_datetime());
    assert_eq!(
        other_datetimes
            .get("ldt2")
            .unwrap()
            .as_local_datetime()
            .unwrap(),
        &LocalDateTime::try_from("1979-05-27T07:32:00.5").unwrap()
    );
    assert!(other_datetimes.get("ldt3").unwrap().is_local_datetime());
    assert_eq!(
        other_datetimes
            .get("ldt3")
            .unwrap()
            .as_local_datetime()
            .unwrap(),
        &LocalDateTime::try_from("1979-05-27T00:32:00.999999").unwrap()
    );
    assert!(other_datetimes.get("ldt4").unwrap().is_local_datetime());
    assert_eq!(
        other_datetimes
            .get("ldt4")
            .unwrap()
            .as_local_datetime()
            .unwrap(),
        &LocalDateTime::try_from("1979-05-27T07:32:00").unwrap()
    );
    assert!(other_datetimes.get("ld1").unwrap().is_local_date());
    assert_eq!(
        other_datetimes.get("ld1").unwrap().as_local_date().unwrap(),
        &LocalDate::try_from("1979-05-27").unwrap()
    );
    assert!(other_datetimes.get("lt1").unwrap().is_local_time());
    assert_eq!(
        other_datetimes.get("lt1").unwrap().as_local_time().unwrap(),
        &LocalTime::try_from("07:32:00").unwrap()
    );
    assert!(other_datetimes.get("lt2").unwrap().is_local_time());
    assert_eq!(
        other_datetimes.get("lt2").unwrap().as_local_time().unwrap(),
        &LocalTime::try_from("00:32:00.5").unwrap()
    );
    assert!(other_datetimes.get("lt3").unwrap().is_local_time());
    assert_eq!(
        other_datetimes.get("lt3").unwrap().as_local_time().unwrap(),
        &LocalTime::try_from("00:32:00.999999").unwrap()
    );
    assert!(other_datetimes.get("lt4").unwrap().is_local_time());
    assert_eq!(
        other_datetimes.get("lt4").unwrap().as_local_time().unwrap(),
        &LocalTime::try_from("07:32:00").unwrap()
    );

    let arrays = parsed.get("arrays").unwrap().as_object().unwrap();
    assert_eq!(arrays.len(), 13);
    let basic_array = arrays.get("basic-array").unwrap().as_array().unwrap();
    assert_eq!(basic_array.len(), 5);
    for (index, value) in basic_array.iter().enumerate() {
        assert_eq!(value.as_number().unwrap().into_usize().unwrap(), index + 1);
    }

    let nested_array = arrays.get("nested-array").unwrap().as_array().unwrap();
    assert_eq!(nested_array.len(), 2);
    let first_array = nested_array.get(0).unwrap().as_array().unwrap();
    assert_eq!(first_array.len(), 3);
    for (index, value) in first_array.iter().enumerate() {
        assert_eq!(value.as_number().unwrap().into_usize().unwrap(), index + 1);
    }
    let second_array = nested_array.get(1).unwrap().as_array().unwrap();
    assert_eq!(second_array.len(), 3);
    for (index, value) in second_array.iter().enumerate() {
        assert_eq!(value.as_number().unwrap().into_usize().unwrap(), index + 4);
    }
    let multi_line = arrays.get("multi-line").unwrap().as_array().unwrap();
    assert_eq!(multi_line.len(), 3);
    assert_eq!(multi_line.get(0).unwrap().as_string().unwrap(), "line 1");
    assert_eq!(multi_line.get(1).unwrap().as_string().unwrap(), "line 2");
    assert_eq!(multi_line.get(2).unwrap().as_string().unwrap(), "line 3");
    let integers = arrays.get("integers").unwrap().as_array().unwrap();
    assert_eq!(integers.len(), 3);
    for (index, value) in integers.iter().enumerate() {
        assert_eq!(value.as_number().unwrap().into_usize().unwrap(), index + 1);
    }
    let colors = arrays.get("colors").unwrap().as_array().unwrap();
    assert_eq!(colors.len(), 3);
    assert_eq!(colors.get(0).unwrap().as_string().unwrap(), "red");
    assert_eq!(colors.get(1).unwrap().as_string().unwrap(), "yellow");
    assert_eq!(colors.get(2).unwrap().as_string().unwrap(), "green");
    let nested_arrays_of_ints = arrays
        .get("nested_arrays_of_ints")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(nested_arrays_of_ints.len(), 2);
    let first_ints = nested_arrays_of_ints.get(0).unwrap().as_array().unwrap();
    assert_eq!(first_ints.len(), 2);
    for (index, value) in first_ints.iter().enumerate() {
        assert_eq!(value.as_number().unwrap().into_usize().unwrap(), index + 1);
    }
    let second_ints = nested_arrays_of_ints.get(1).unwrap().as_array().unwrap();
    assert_eq!(second_ints.len(), 3);
    assert_eq!(second_ints, &Array::from(vec![3_u8, 4, 5]));

    let nested_mixed_array = arrays
        .get("nested_mixed_array")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(nested_mixed_array.len(), 2);
    let first_ints = nested_mixed_array.get(0).unwrap().as_array().unwrap();
    assert_eq!(first_ints.len(), 2);
    for (index, value) in first_ints.iter().enumerate() {
        assert_eq!(value.as_number().unwrap().into_usize().unwrap(), index + 1);
    }
    let second_ints = nested_mixed_array.get(1).unwrap().as_array().unwrap();
    assert_eq!(second_ints.len(), 3);
    assert_eq!(second_ints, &Array::from(vec!["a", "b", "c"]));

    let string_array = arrays.get("string_array").unwrap().as_array().unwrap();
    assert_eq!(string_array.len(), 4);
    assert_eq!(string_array.get(0).unwrap().as_string().unwrap(), "all");
    assert_eq!(string_array.get(1).unwrap().as_string().unwrap(), "strings");
    assert_eq!(
        string_array.get(2).unwrap().as_string().unwrap(),
        "are the same"
    );
    assert_eq!(string_array.get(3).unwrap().as_string().unwrap(), "type");
    let empty_array = arrays.get("empty_array").unwrap().as_array().unwrap();
    assert_eq!(empty_array.len(), 0);

    let numbers = arrays.get("numbers").unwrap().as_array().unwrap();
    assert_eq!(numbers.len(), 6);
    assert_eq!(
        numbers
            .get(0)
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        0.1
    );
    assert_eq!(
        numbers
            .get(1)
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        0.2
    );
    assert_eq!(
        numbers
            .get(2)
            .unwrap()
            .as_number()
            .unwrap()
            .into_f64()
            .unwrap(),
        0.5
    );
    assert_eq!(
        numbers
            .get(3)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );
    assert_eq!(
        numbers
            .get(4)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        2
    );
    assert_eq!(
        numbers
            .get(5)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        5
    );
    let contributors = arrays.get("contributors").unwrap().as_array().unwrap();
    assert_eq!(contributors.len(), 2);
    assert_eq!(
        contributors.get(0).unwrap().as_string().unwrap(),
        "Foo Bar <foo@example.com>"
    );
    let inner_obj = contributors.get(1).unwrap().as_object().unwrap();
    assert_eq!(inner_obj.len(), 3);
    assert_eq!(
        inner_obj.get("name").unwrap().as_string().unwrap(),
        "Baz Qux"
    );
    assert_eq!(
        inner_obj.get("email").unwrap().as_string().unwrap(),
        "bazqux@example.com"
    );
    assert_eq!(
        inner_obj.get("url").unwrap().as_string().unwrap(),
        "https://example.com/bazqux"
    );
    let integers2 = arrays.get("integers2").unwrap().as_array().unwrap();
    assert_eq!(integers2.len(), 3);
    assert_eq!(
        integers2
            .get(0)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );
    assert_eq!(
        integers2
            .get(1)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        2
    );
    assert_eq!(
        integers2
            .get(2)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        3
    );
    let integers3 = arrays.get("integers3").unwrap().as_array().unwrap();
    assert_eq!(integers3.len(), 2);
    assert_eq!(
        integers3
            .get(0)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );
    assert_eq!(
        integers3
            .get(1)
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        2
    );
    let nested = parsed.get("nested").unwrap().as_object().unwrap();
    assert_eq!(nested.len(), 1);
    let nested_table = nested.get("table").unwrap().as_object().unwrap();
    assert_eq!(nested_table.len(), 1);
    let nested_table_definition = nested_table.get("definition").unwrap().as_object().unwrap();
    assert_eq!(nested_table_definition.len(), 2);
    assert_eq!(
        nested_table_definition
            .get("some-val")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );

    let quoted_table = parsed
        .get("a quoted table name!")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(quoted_table.len(), 2);
    assert_eq!(
        quoted_table.get("unicode_2").unwrap().as_string().unwrap(),
        " "
    );
    assert_eq!(
        quoted_table.get("unicode_4").unwrap().as_string().unwrap(),
        "δ"
    );
    let ary_of_tables = parsed
        .get("an array of quoted tables")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(ary_of_tables.len(), 1);
    assert_eq!(
        ary_of_tables
            .get(0)
            .unwrap()
            .as_object()
            .unwrap()
            .get("unicode_8")
            .unwrap()
            .as_string()
            .unwrap(),
        "ϧ"
    );

    let nested_table_definition_another = nested_table_definition
        .get("another")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(nested_table_definition_another.len(), 1);
    let nested_table_definition_another_layer = nested_table_definition_another
        .get("layer")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(nested_table_definition_another_layer.len(), 5);
    let nested = nested_table_definition_another_layer
        .get("nested")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(nested.len(), 1);
    assert_eq!(
        nested
            .get("some-val")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );
    let inline_table = nested_table_definition_another_layer
        .get("inline-table")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(inline_table.len(), 1);
    assert_eq!(
        inline_table.get("inline").unwrap().as_string().unwrap(),
        "table"
    );
    let multi_line_inline_table = nested_table_definition_another_layer
        .get("multi-line-inline-table")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(multi_line_inline_table.len(), 1);
    let name = multi_line_inline_table
        .get("name")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(name.len(), 2);
    assert_eq!(name.get("first").unwrap().as_string().unwrap(), "Tom");
    assert_eq!(
        name.get("last").unwrap().as_string().unwrap(),
        "Preston-Werner"
    );
    let empty_multi_line_inline_table = nested_table_definition_another_layer
        .get("empty_inline_table")
        .unwrap()
        .as_object()
        .unwrap();
    assert_eq!(empty_multi_line_inline_table.len(), 0);

    let points = nested_table_definition_another_layer
        .get("points")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(points.len(), 3);
    // Could check the number values, that is covered by the others though
    for value in points.iter() {
        assert!(value.is_object());
        let obj = value.as_object().unwrap();
        assert_eq!(obj.len(), 3);
        assert!(obj.get("x").unwrap().is_number());
        assert!(obj.get("y").unwrap().is_number());
        assert!(obj.get("z").unwrap().is_number());
    }

    let array_of_tables = parsed.get("array-of-tables").unwrap().as_array().unwrap();
    assert_eq!(array_of_tables.len(), 4);
    assert!(array_of_tables.get(0).unwrap().is_object());
    let obj = array_of_tables.get(0).unwrap().as_object().unwrap();
    assert_eq!(obj.len(), 1);
    assert_eq!(
        obj.get("w")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );
    assert!(array_of_tables.get(1).unwrap().is_object());
    assert!(
        array_of_tables
            .get(1)
            .unwrap()
            .as_object()
            .unwrap()
            .is_empty()
    );
    assert!(array_of_tables.get(2).unwrap().is_object());
    assert_eq!(
        array_of_tables
            .get(2)
            .unwrap()
            .as_object()
            .unwrap()
            .get("y")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );
    assert!(array_of_tables.get(3).unwrap().is_object());
    assert_eq!(
        array_of_tables
            .get(3)
            .unwrap()
            .as_object()
            .unwrap()
            .get("z")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        1
    );

    let x = parsed.get("x").unwrap().as_object().unwrap();
    assert!(x.len() == 2);
    assert_eq!(
        x.get("super-table-key").unwrap().as_boolean().unwrap(),
        &true
    );
    let y = x.get("y").unwrap().as_object().unwrap();
    assert!(y.len() == 1);
    let z = y.get("z").unwrap().as_object().unwrap();
    assert!(z.len() == 1);
    let w = z.get("w").unwrap().as_object().unwrap();
    assert!(w.len() == 1);
    assert_eq!(
        w.get("key")
            .unwrap()
            .as_number()
            .unwrap()
            .into_usize()
            .unwrap(),
        2
    );
}

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
