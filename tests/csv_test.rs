#[cfg(feature = "csv")]
#[cfg(test)]
mod csv_tests {

    mod headed {
        use mawu::MawuContents;
        use pretty_assertions::assert_eq;

        #[test]
        #[ignore]
        fn my_own_random_large_data_set_84mb_1mil_rows() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/my-own-random-data/test_data_1mil.csv",
            );
            assert!(mawu_result.is_ok());
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_object().unwrap().len(), 1_000_000);
        }

        #[test]
        fn write_and_read_data() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/my-own-random-data/all-types.csv",
            )
            .unwrap();
            assert!(mawu_result.is_csv_object());
            let write_succ = mawu::write(
                "test_file_delete_me.csv",
                MawuContents::Csv(mawu_result.clone()),
            );
            let write_read = mawu::read::csv_headed("test_file_delete_me.csv");
            assert!(write_succ.is_ok());
            assert!(write_read.is_ok());

            let read_val = write_read.unwrap();
            for row in read_val.as_csv_object().unwrap() {
                assert_eq!(row.len(), 3);
                let id = row
                    .get("Id")
                    .unwrap()
                    .clone()
                    .into_number()
                    .unwrap()
                    .into_usize()
                    .unwrap();
                let types = row.get("Type").unwrap();
                let content = row.get("Content").unwrap();

                if id == 1 {
                    assert_eq!(types.clone().into_string().unwrap(), "uint");
                    assert_eq!(
                        content.clone().into_number().unwrap().into_usize().unwrap(),
                        0
                    );
                } else if id == 8 {
                    assert_eq!(types.clone().into_string().unwrap(), "sint");
                    assert_eq!(
                        content.clone().into_number().unwrap().into_isize().unwrap(),
                        -1
                    );
                } else if id == 18 {
                    assert_eq!(types.clone().into_string().unwrap(), "float");
                    assert_eq!(
                        content.clone().into_number().unwrap().into_f64().unwrap(),
                        0.0
                    );
                } else if id == 25 {
                    assert_eq!(types.clone().into_string().unwrap(), "bool");
                    assert_eq!(content.clone().into_boolean().unwrap(), true);
                } else if id == 27 {
                    assert_eq!(types.clone().into_string().unwrap(), "none");
                    assert!(content.is_null());
                }
            }

            std::fs::remove_file("test_file_delete_me.csv").unwrap();
        }

        #[test]
        fn random_data_no_license_customers100() {
            let mawu_result = mawu::read::csv_headed(
                "data/csv/csv-test-data/headed/random-data-no-license/customers-100.csv",
            );
            assert_eq!(mawu_result.is_ok(), true);
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_object().unwrap().len(), 100);
            assert_eq!(mawu.as_csv_object().unwrap()[0].len(), 12);

            let row0 = &mawu.as_csv_object().unwrap()[0];
            assert_eq!(
                row0.get("Index")
                    .unwrap()
                    .clone()
                    .into_number()
                    .unwrap()
                    .into_usize()
                    .unwrap(),
                1
            );
            assert_eq!(
                row0.get("Customer Id")
                    .unwrap()
                    .clone()
                    .into_string()
                    .unwrap(),
                "DD37Cf93aecA6Dc"
            );
        }

        #[test]
        fn serialize_table_to_csv() {
            use athena::{Table, XffValue};
            let path = "table_output.csv";
            let mut table = Table::with_columns(vec!["Name".to_string(), "Age".to_string()]);
            table
                .add_row(vec![XffValue::from("Alice"), XffValue::from(30)])
                .unwrap();
            table
                .add_row(vec![XffValue::from("Bob"), XffValue::from(25)])
                .unwrap();

            mawu::write_pretty(path, mawu::mawu_value::MawuValue::Table(table), 0).unwrap();

            let read_csv = mawu::read::csv_headed(path).unwrap();
            assert!(read_csv.is_csv_object());
            let rows = read_csv.as_csv_object().unwrap();
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0].get("Name").unwrap().as_string().unwrap(), "Alice");
            assert_eq!(
                rows[1]
                    .get("Age")
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .into_usize()
                    .unwrap(),
                25
            );

            std::fs::remove_file(path).unwrap();
        }

        #[test]
        fn serialize_object_to_csv() {
            use athena::Object;
            let path = "object_output.csv";
            let mut obj = Object::new();
            obj.insert("Key1", "Val1");
            obj.insert("Key2", 42);

            mawu::write_pretty(path, mawu::mawu_value::MawuValue::Object(obj), 0).unwrap();

            let read_csv = mawu::read::csv_headed(path).unwrap();
            assert!(read_csv.is_csv_object());
            let rows = read_csv.as_csv_object().unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].get("Key1").unwrap().as_string().unwrap(), "Val1");

            std::fs::remove_file(path).unwrap();
        }

        #[test]
        fn serialize_ordered_object_to_csv() {
            use athena::OrderedObject;
            let path = "ordered_object_output.csv";
            let mut obj = OrderedObject::new();
            obj.push("A", 1);
            obj.push("B", 2);

            mawu::write_pretty(path, mawu::mawu_value::MawuValue::OrderedObject(obj), 0).unwrap();

            let read_csv = mawu::read::csv_headed(path).unwrap();
            assert!(read_csv.is_csv_object());
            let rows = read_csv.as_csv_object().unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(
                rows[0]
                    .get("A")
                    .unwrap()
                    .as_number()
                    .unwrap()
                    .into_usize()
                    .unwrap(),
                1
            );

            std::fs::remove_file(path).unwrap();
        }

        #[test]
        fn nested_structures_in_csv() {
            use athena::{Object, XffValue};
            use std::collections::HashMap;
            let path = "nested_output.csv";
            let mut row = HashMap::new();
            row.insert("simple".to_string(), XffValue::from("val"));
            row.insert("array".to_string(), XffValue::from(vec![1, 2, 3]));

            let mut nested_obj = Object::new();
            nested_obj.insert("inner", "value");
            row.insert("object".to_string(), XffValue::Object(nested_obj));

            let csv = mawu::mawu_value::MawuValue::CSVObject(vec![row]);
            mawu::write_pretty(path, csv, 0).unwrap();

            let content = std::fs::read_to_string(path).unwrap();
            assert!(content.contains("simple"));
            assert!(content.contains("array"));
            assert!(content.contains("object"));
            assert!(content.contains("[1,2,3]"));
            assert!(content.contains("{inner:\"value\"}"));

            std::fs::remove_file(path).unwrap();
        }

        #[test]
        fn command_character_in_csv() {
            use athena::{CommandCharacter, XffValue};
            use std::collections::HashMap;
            let path = "cmd_char_output.csv";
            let mut row = HashMap::new();
            row.insert(
                "cmd".to_string(),
                XffValue::CommandCharacter(CommandCharacter::Bell),
            );
            row.insert(
                "array_cmd".to_string(),
                XffValue::ArrayCmdChar(vec![
                    CommandCharacter::StartOfHeading,
                    CommandCharacter::EndOfText,
                ]),
            );

            let csv = mawu::mawu_value::MawuValue::CSVObject(vec![row]);
            mawu::write_pretty(path, csv, 0).unwrap();

            let content = std::fs::read_to_string(path).unwrap();
            assert!(content.contains("cmd"));
            assert!(content.contains("array_cmd"));
            assert!(content.contains("\"0x07\"")); // Bell
            assert!(content.contains("[\"0x01\",\"0x03\"]")); // SOH, ETX

            std::fs::remove_file(path).unwrap();
        }
    }

    mod headless {
        use mawu::MawuContents;
        use pretty_assertions::assert_eq;

        #[test]
        fn read_and_write_data() {
            let mawu_result = mawu::read::csv_headless(
                "data/csv/csv-test-data/headless/my-own-random-data/all-types.csv",
            );
            assert_eq!(mawu_result.is_ok(), true);
            let mawu = mawu_result.unwrap();
            assert_eq!(mawu.as_csv_array().unwrap().len(), 50);
            let write_succ =
                mawu::write("test_file_delete_me2.csv", MawuContents::Csv(mawu.clone()));
            assert!(write_succ.is_ok());
            let read_write = mawu::read::csv_headless("test_file_delete_me2.csv");
            assert!(read_write.is_ok());
            assert_eq!(read_write.as_ref().unwrap(), &mawu);

            let read_val = read_write.unwrap();
            let row1 = &read_val.as_csv_array().unwrap()[1];

            assert_eq!(row1.len(), 3);

            std::fs::remove_file("test_file_delete_me2.csv").unwrap();
        }
    }
}
