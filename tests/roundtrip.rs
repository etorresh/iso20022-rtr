use std::fs;

use fixture_meta::{Meta, ResponseKind};
use libtest_mimic::{Arguments, Failed, Trial};
use rtr_iso20022::{
    Message,
    MessageType::{self},
};

fn main() {
    let args = Arguments::from_args();

    let mut tests_dir_entries = Vec::new();

    let path_fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for category_dir in fs::read_dir(&path_fixtures).unwrap() {
        for test_dir in fs::read_dir(category_dir.unwrap().path()).unwrap() {
            tests_dir_entries.push(test_dir.unwrap());
        }
    }

    tests_dir_entries.sort_by_key(|a| a.file_name());

    let code_of = |payload: &MessageType| match payload {
        MessageType::Admi002(_) => "admi.002",
        MessageType::Admi004(_) => "admi.004",
        MessageType::Admi011(_) => "admi.011",
        MessageType::Pacs002(_) => "pacs.002",
        MessageType::Pacs008(_) => "pacs.008",
        MessageType::Pacs028(_) => "pacs.028",
    };

    let run_case = move |expected_code: &str, raw_message: &str| -> Result<(), Failed> {
        let message: Message = serde_json::from_str(raw_message)?;
        let code = code_of(&message.payload);
        if code != expected_code {
            return Err(format!("expected {expected_code}, got {code}").into());
        }
        Ok(())
    };

    let mut tests = Vec::new();

    for dir_entry in tests_dir_entries {
        let test_dir_path = dir_entry.path();

        let meta_str = fs::read_to_string(test_dir_path.join("meta.json")).unwrap();
        let meta: Meta = serde_json::from_str(&meta_str).unwrap();

        let expected_code = meta.request;
        let request_path = test_dir_path.join("request.json");
        let request_str = fs::read_to_string(&request_path).unwrap();
        let name = request_path
            .strip_prefix(&path_fixtures)
            .unwrap()
            .display()
            .to_string();
        tests.push(Trial::test(name, move || {
            run_case(&expected_code, &request_str)
        }));

        if let ResponseKind::Iso(expected_code) = meta.response {
            let response_path = test_dir_path.join("response.json");
            if let Ok(response_str) = fs::read_to_string(&response_path) {
                let name = response_path
                    .strip_prefix(&path_fixtures)
                    .unwrap()
                    .display()
                    .to_string();
                tests.push(Trial::test(name, move || {
                    run_case(&expected_code, &response_str)
                }));
            }
        }
    }

    libtest_mimic::run(&args, tests).exit();
}
