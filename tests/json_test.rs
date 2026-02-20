#[cfg(test)]
mod json_tests {
    use mawu::{read::json, MawuContents};
    use athena::XffValue;
    use pretty_assertions::assert_eq;

    #[test]
    fn nan_infinity_has_to_fail() {
        let nan = json("data/json/json-test-data/n_nan.json");
        assert!(nan.is_err());
        let infinity = json("data/json/json-test-data/n_inf.json");
        assert!(infinity.is_err());
    }

    #[test]
    fn write_weird_shit() {
        let weird = XffValue::from(vec![
            XffValue::from("\""),
            XffValue::from("\\"),
            XffValue::from("/"),
            XffValue::from("\u{0061}"),
            XffValue::from("\u{30af}"),
            XffValue::from("\u{30EA}"),
            XffValue::from("\u{30b9}"),
            XffValue::from("\u{A66D}"),
            XffValue::from("\u{002c}"),
            XffValue::from("\u{0123}"),
            XffValue::from("\u{0123}"),
            XffValue::from("new\u{000A}line"),
        ]);
        let write_succ = mawu::write("test_file_delete_me_weird_unicode.json", MawuContents::Json(weird.clone()));
        assert!(write_succ.is_ok());
        let read_succ = mawu::read::json("test_file_delete_me_weird_unicode.json");
        assert!(read_succ.is_ok());
        let read_val = read_succ.unwrap();
        
        let weird_arr = weird.into_array().unwrap();
        let read_arr = read_val.clone().into_array().unwrap();
        
        for (a_weird, b_read) in weird_arr.iter().zip(read_arr.iter()) {
            assert_eq!(a_weird, b_read);
        }
        assert_eq!(read_val, weird);

        let weird2 = XffValue::from(vec![
            XffValue::from(r"\b"),
            XffValue::from(r"\f"),
            XffValue::from(r"\n"),
            XffValue::from(r"\r"),
            XffValue::from(r"\t"),
        ]);
        let write_succ2 = mawu::write("test_file_delete_me_weird_unicode2.json", MawuContents::Json(weird2.clone()));
        assert!(write_succ2.is_ok());
        let read_succ2 = mawu::read::json("test_file_delete_me_weird_unicode2.json");
        assert!(read_succ2.is_ok());

        // clenup time!
        std::fs::remove_file("test_file_delete_me_weird_unicode.json").unwrap();
        std::fs::remove_file("test_file_delete_me_weird_unicode2.json").unwrap();
    }

    #[test]
    #[ignore]
    fn large_json_file_26mb() {
        let large_result =
            json("data/json/json-test-data/large-file-json/large-file.json").unwrap();
        assert_eq!(large_result.is_array(), true);
        assert_eq!(large_result.into_array().unwrap().len(), 11351);
    }

    #[test]
    #[ignore]
    fn my_own_random_data_26mb() {
        let mawu_result = json("data/json/json-test-data/test_data_160k.json").unwrap();
        assert_eq!(mawu_result.is_array(), true);
        assert_eq!(mawu_result.into_array().unwrap().len(), 160000);
    }

    #[test]
    #[ignore]
    fn speed_test() {
        let mawu_result = json("data/json/json-test-data/test_data_517k.json").unwrap();
        assert_eq!(mawu_result.is_array(), true);
        assert_eq!(mawu_result.into_array().unwrap().len(), 517000);
    }

    #[test]
    #[ignore]
    fn my_own_random_data_84mb() {
        let mawu_result = json("data/json/json-test-data/test_data_517k.json").unwrap();
        assert_eq!(mawu_result.is_array(), true);
        assert_eq!(mawu_result.into_array().unwrap().len(), 517000);
    }

    #[test]
    fn simple_valid_json() {
        let simple_result = json("data/json/json-test-data/simple-json.json").unwrap();
        let tmp_simple_bind = simple_result.into_object().unwrap();
        assert_eq!(tmp_simple_bind.len(), 1);
        let tmp_quiz = tmp_simple_bind.get("quiz").unwrap().clone().into_object().unwrap();
        // Sports
        let sports = tmp_quiz.get("sport").unwrap().clone().into_object().unwrap();
        let sport_q1 = sports.get("q1").unwrap().clone().into_object().unwrap();
        let s_q1_question = sport_q1.get("question").unwrap().clone().into_string().unwrap();
        assert_eq!(
            s_q1_question,
            "Which one is correct team name in 1. Bundesliga?"
        );
        let s_q1_options = sport_q1.get("options").unwrap().clone().into_array().unwrap();
        assert_eq!(s_q1_options.len(), 4);
        assert_eq!(
            s_q1_options.get(0).unwrap().clone().into_string().unwrap(),
            "2. Fc Bayern"
        );
        let s_q1_answer = sport_q1.get("answer").unwrap().clone().into_string().unwrap();
        assert_eq!(s_q1_answer, "VfB Stuttgart");
        // Maths
        let maths = tmp_quiz.get("maths").unwrap().clone().into_object().unwrap();
        let m_q1 = maths.get("q1").unwrap().clone().into_object().unwrap();
        let m_q1_question = m_q1.get("question").unwrap().clone().into_string().unwrap();
        assert_eq!(m_q1_question, "5 + 8 = ?");
        let m_q1_options = m_q1.get("options").unwrap().clone().into_array().unwrap();
        assert_eq!(m_q1_options.len(), 4);
        assert_eq!(m_q1_options.get(0).unwrap().clone().into_number().unwrap().into_usize().unwrap(), 10);
        let m_q1_answer = m_q1.get("answer").unwrap().clone().into_number().unwrap().into_usize().unwrap();
        assert_eq!(m_q1_answer, 13);
        let m_q2 = maths.get("q2").unwrap().clone().into_object().unwrap();
        let m_q2_question = m_q2.get("question").unwrap().clone().into_string().unwrap();
        assert_eq!(m_q2_question, "12 - 10 = ?");
        let m_q2_options = m_q2.get("options").unwrap().clone().into_array().unwrap();
        assert_eq!(m_q2_options.len(), 4);
        assert_eq!(m_q2_options.get(0).unwrap().clone().into_number().unwrap().into_usize().unwrap(), 1);
        let m_q2_answer = m_q2.get("answer").unwrap().clone().into_number().unwrap().into_usize().unwrap();
        assert_eq!(m_q2_answer, 2);

        let very_simple_result = json("data/json/json-test-data/very-simple-json.json").unwrap();
        let vs_obj = very_simple_result.into_object().unwrap();
        assert_eq!(vs_obj.len(), 3);
        let vs_key1 = vs_obj.get("fruit").unwrap();
        assert_eq!(vs_key1.clone().into_string().unwrap(), "Banana");
        let vs_key2 = vs_obj.get("size").unwrap();
        assert_eq!(vs_key2.clone().into_string().unwrap(), "Medium");
        let vs_key3 = vs_obj.get("colour").unwrap();
        assert_eq!(vs_key3.clone().into_string().unwrap(), "Blue");
    }

    #[test]
    fn rfc8259_valid_json() {
        let rfc8259_array = json("data/json/json-test-data/rfc8259-test-data/array.json").unwrap();
        let rfc8259_object =
            json("data/json/json-test-data/rfc8259-test-data/object.json").unwrap();
        let rfc8259_string =
            json("data/json/json-test-data/rfc8259-test-data/small-text1.json").unwrap();
        let rfc8259_num =
            json("data/json/json-test-data/rfc8259-test-data/small-text2.json").unwrap();
        let rfc8259_bool =
            json("data/json/json-test-data/rfc8259-test-data/small-text3.json").unwrap();
        assert_eq!(rfc8259_array.is_array(), true);
        assert_eq!(rfc8259_array.into_array().unwrap().len(), 2);
        assert_eq!(rfc8259_object.is_object(), true);
        assert_eq!(rfc8259_object.clone().into_object().unwrap().len(), 1);
        assert_eq!(
            rfc8259_object
                .into_object()
                .unwrap()
                .get("Image")
                .unwrap()
                .clone()
                .into_object()
                .unwrap()
                .len(),
            6
        );
        assert_eq!(rfc8259_string.is_string(), true);
        assert_eq!(
            rfc8259_string.into_string().unwrap(),
            "Hello world!".to_string()
        );
        assert!(rfc8259_num.is_number());
        assert_eq!(rfc8259_num.into_number().unwrap().into_usize().unwrap(), 42);
        assert_eq!(rfc8259_bool.is_boolean(), true);
        assert_eq!(rfc8259_bool.into_boolean().unwrap(), true);
    }

    #[test]
    fn json_org_valid_json() {
        let small_weird_json =
            json("data/json/json-test-data/jsonOrg-json-examples/small-weird-json.json").unwrap();
        let small_simple_json =
            json("data/json/json-test-data/jsonOrg-json-examples/small-simple-json.json").unwrap();
        let small_complex_json =
            json("data/json/json-test-data/jsonOrg-json-examples/small-complex-json.json").unwrap();
        let medium_complex_json =
            json("data/json/json-test-data/jsonOrg-json-examples/medium-complex-json.json")
                .unwrap();
        let large_complex_json =
            json("data/json/json-test-data/jsonOrg-json-examples/large-complex-json.json").unwrap();
        assert_eq!(small_weird_json.is_object(), true);
        assert_eq!(small_simple_json.is_object(), true);
        assert_eq!(small_complex_json.is_object(), true);
        assert_eq!(medium_complex_json.is_object(), true);
        assert_eq!(large_complex_json.is_object(), true);
    }

    #[test]
    fn microsoft_edge_valid_dummy_json_small() {
        let micro_64kb =
            json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/64KB.json")
                .unwrap();
        assert_eq!(micro_64kb.is_array(), true);
        let micro_64kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/64KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_64kb_mini.is_array(), true);

        let micro_128kb = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/128KB.json",
        )
        .unwrap();
        assert_eq!(micro_128kb.is_array(), true);
        let micro_128kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/128KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_128kb_mini.is_array(), true);

        let micro_256kb = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/256KB.json",
        )
        .unwrap();
        assert_eq!(micro_256kb.is_array(), true);
        let micro_256kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/256KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_256kb_mini.is_array(), true);

        let micro_512kb = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/512KB.json",
        )
        .unwrap();
        assert_eq!(micro_512kb.is_array(), true);
        let micro_512kb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/512KB-min.json",
        )
        .unwrap();
        assert_eq!(micro_512kb_mini.is_array(), true);
    }

    #[test]
    #[ignore]
    fn microsoft_edge_valid_dummy_json_large() {
        let micro_1mb =
            json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/1MB.json")
                .unwrap();
        assert_eq!(micro_1mb.is_array(), true);
        let micro_1mb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/1MB-min.json",
        )
        .unwrap();
        assert_eq!(micro_1mb_mini.is_array(), true);

        let micro_5mb =
            json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/5MB.json")
                .unwrap();
        assert_eq!(micro_5mb.is_array(), true);
        let micro_5mb_mini = json(
            "data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/5MB-min.json",
        )
        .unwrap();
        assert_eq!(micro_5mb_mini.is_array(), true);
    }

    #[test]
    fn microsoft_edge_invalid_dummy_json() {
        let invalid_binary = json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/binary-data.json");
        assert_eq!(invalid_binary.is_err(), true);

        let invalid_missing_colon = json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/missing-colon.json");
        assert_eq!(invalid_missing_colon.is_err(), true);

        let unterminated = json("data/json/json-test-data/microsoftEdge-json-test-data/json-dummy-data/invalid-json/unterminated.json");
        assert_eq!(unterminated.is_err(), true);
    }

    #[test]
    fn transformation_numbers() {
        let number_10 = json(
            "data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1.0.json",
        )
        .unwrap();
        assert_eq!(number_10.clone().into_array().unwrap().len(), 1);
        assert!(number_10.into_array().unwrap().get(0).unwrap().clone().into_number().unwrap().into_f64().unwrap() == 1.0);
        
        let number_1000000000000000005 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1.000000000000000005.json").unwrap();
        assert_eq!(
            number_1000000000000000005.into_array().unwrap().get(0).unwrap()
                .clone().into_number()
                .unwrap().into_f64().unwrap(),
            1.0
        );
        let number_1e6 = json(
            "data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1e6.json",
        )
        .unwrap();
        assert_eq!(
            number_1e6.into_array().unwrap().get(0).unwrap().clone().into_number().unwrap().into_f64().unwrap(),
            1000000.0
        );
        let number_1e_999 = json(
            "data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1e-999.json",
        )
        .unwrap();
        assert_eq!(
            number_1e_999.into_array().unwrap().get(0).unwrap().clone().into_number().unwrap().into_f64().unwrap(),
            0.0
        );
        let number_1000000000000000 = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/number_1000000000000000.json").unwrap();
        assert_eq!(
            number_1000000000000000.into_array().unwrap().get(0).unwrap()
                .clone().into_number()
                .unwrap().into_usize().unwrap(),
            1000000000000000
        );
    }

    #[test]
    fn transformation_objects() {
        let object_key_nfc_nfd = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_key_nfc_nfd.json").unwrap();
        assert_eq!(object_key_nfc_nfd.into_object().unwrap().len(), 2);
        let object_key_nfd_nfc = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_key_nfd_nfc.json").unwrap();
        assert_eq!(object_key_nfd_nfc.into_object().unwrap().len(), 2);
        // overwrites as expected
        let object_same_key_different_values = json("data/json/json-test-data/jsonTestSuite-data/i_test_transform/object_same_key_different_values.json").unwrap();
        assert_eq!(
            object_same_key_different_values.clone().into_object().unwrap().len(),
            1
        );
        assert_eq!(
            object_same_key_different_values
                .into_object()
                .unwrap()
                .get("a")
                .unwrap()
                .clone()
                .into_number()
                .unwrap()
                .into_usize()
                .unwrap(),
            2
        );
    }

    #[test]
    fn implementor_dependent_numbers() {
        // I accept underflow to 0.0 - documented
        let number_double_huge_neg_exp = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_double_huge_neg_exp.json").unwrap();
        assert_eq!(
            number_double_huge_neg_exp.clone().into_array().unwrap().get(0).unwrap().is_number(),
            true
        );
        assert!(
            number_double_huge_neg_exp.into_array().unwrap().get(0).unwrap().clone()
                .into_number()
                .unwrap()
                .into_f64()
                .unwrap()
                == 0.0
        );
        let number_real_underflow = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_real_underflow.json").unwrap();
        assert_eq!(number_real_underflow.clone().into_array().unwrap().len(), 1);
        assert!(
            number_real_underflow.into_array().unwrap().get(0).unwrap().clone()
                .into_number()
                .unwrap()
                .into_f64()
                .unwrap()
                == 0.0
        );
        // I don't accept overflow to infinity - documented
        let number_huge_exp = json(
            "data/json/json-test-data/jsonTestSuite-data/test_parsing/i_number_huge_exp.json",
        )
        .unwrap();
        assert_eq!(number_huge_exp.clone().into_array().unwrap().len(), 1);
        assert!(number_huge_exp.into_array().unwrap().get(0).unwrap().is_null());
    }

    #[test]
    fn valid_arrays() {
        let arrays_with_spaces = json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_arraysWithSpaces.json").unwrap();
        assert_eq!(arrays_with_spaces.is_array(), true);
        let array_empty =
            json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_empty.json")
                .unwrap();
        assert_eq!(array_empty.is_array(), true);
        
        let array_null =
            json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_array_null.json")
                .unwrap();
        assert_eq!(array_null.is_array(), true);
        assert_eq!(array_null.into_array().unwrap().get(0).unwrap().is_null(), true);
    }

    #[test]
    fn valid_objects() {
        let object =
            json("data/json/json-test-data/jsonTestSuite-data/test_parsing/y_object.json")
                .unwrap();
        assert_eq!(object.is_object(), true);
        let obj = object.into_object().unwrap();
        assert_eq!(
            obj.get("asd")
                .unwrap()
                .clone()
                .into_string()
                .unwrap(),
            "sdf"
        );
        assert_eq!(
            obj.get("dfg")
                .unwrap()
                .clone()
                .into_string()
                .unwrap(),
            "fgh"
        );
    }
}
