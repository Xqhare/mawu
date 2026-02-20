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
            ).unwrap();
            assert!(mawu_result.is_csv_object());
            let write_succ = mawu::write("test_file_delete_me.csv", MawuContents::Csv(mawu_result.clone()));
            let write_read = mawu::read::csv_headed("test_file_delete_me.csv");
            assert!(write_succ.is_ok());
            assert!(write_read.is_ok());
            
            let read_val = write_read.unwrap();
            for row in read_val.as_csv_object().unwrap() {
                assert_eq!(row.len(), 3);
                let id = row.get("Id").unwrap().clone().into_number().unwrap().into_usize().unwrap();
                let types = row.get("Type").unwrap();
                let content = row.get("Content").unwrap();
                
                if id == 1 {
                    assert_eq!(types.clone().into_string().unwrap(), "uint");
                    assert_eq!(content.clone().into_number().unwrap().into_usize().unwrap(), 0);
                } else if id == 8 {
                    assert_eq!(types.clone().into_string().unwrap(), "sint");
                    assert_eq!(content.clone().into_number().unwrap().into_isize().unwrap(), -1);
                } else if id == 18 {
                    assert_eq!(types.clone().into_string().unwrap(), "float");
                    assert_eq!(content.clone().into_number().unwrap().into_f64().unwrap(), 0.0);
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
                row0.get("Index").unwrap().clone().into_number().unwrap().into_usize().unwrap(),
                1
            );
            assert_eq!(
                row0.get("Customer Id").unwrap().clone().into_string().unwrap(),
                "DD37Cf93aecA6Dc"
            );
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
            let write_succ = mawu::write("test_file_delete_me2.csv", MawuContents::Csv(mawu.clone()));
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
