use std::{collections::HashSet, fs};

use fixture_meta::{Meta, ResponseKind};
use libtest_mimic::{Arguments, Failed, Trial};
use rtr_iso20022::{
    Message,
    MessageType::{self},
};

/// Sandbox scenarios whose sample request is deliberately invalid, paired with
/// the error the parser is expected to report.
const EXPECTED_REJECTIONS: &[(&str, &str)] = &[
    (
        "Exchange Inbound - Send Payment/pacs.008_Credit Transfer.17",
        "unknown field `grheader`",
    ),
    (
        "Exchange Inbound - Payment Inqu/pacs.028_Payment Status Request.08",
        "longer than 35 characters",
    ),
    (
        "Exchange Inbound - Heartbeat/get_heartbeat.04",
        "unknown field `event_codes`",
    ),
];

/// Sandbox scenarios whose sample request is not tested, paired with the reason.
const SKIPPED: &[(&str, &str)] = &[(
    "Exchange Inbound - Payment Inqu/pacs.028_Payment Status Request.11",
    "malformed JSON (trailing comma); exercises serde_json, not this crate",
)];

fn main() {
    let args = Arguments::from_args();

    let mut tests_dir_entries = Vec::new();

    let path_fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for category_dir in fs::read_dir(&path_fixtures).unwrap() {
        for test_dir in fs::read_dir(category_dir.unwrap().path()).unwrap() {
            tests_dir_entries.push(test_dir.unwrap());
        }
    }

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

    let reject_case = |expected_error: &str, raw_message: &str| -> Result<(), Failed> {
        match serde_json::from_str::<Message>(raw_message) {
            Ok(_) => {
                Err(format!("expected error containing `{expected_error}`, but it parsed").into())
            }
            Err(err) if err.to_string().contains(expected_error) => Ok(()),
            Err(err) => {
                Err(format!("expected error containing `{expected_error}`, got `{err}`").into())
            }
        }
    };

    let mut tests = Vec::new();
    let mut case_ids = HashSet::new();

    for dir_entry in tests_dir_entries {
        let test_dir_path = dir_entry.path();
        let case_id = test_dir_path
            .strip_prefix(&path_fixtures)
            .unwrap()
            .display()
            .to_string();

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
        let rejection = EXPECTED_REJECTIONS
            .iter()
            .find(|(id, _)| *id == case_id)
            .map(|(_, error)| *error);
        let trial = match rejection {
            Some(expected_error) => {
                Trial::test(name, move || reject_case(expected_error, &request_str))
            }
            None => Trial::test(name, move || run_case(&expected_code, &request_str)),
        };
        let skipped = SKIPPED.iter().any(|(id, _)| *id == case_id);
        tests.push(trial.with_ignored_flag(skipped));
        case_ids.insert(case_id);

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

    for (id, _) in EXPECTED_REJECTIONS.iter().chain(SKIPPED) {
        assert!(
            case_ids.contains(*id),
            "`{id}` is listed in roundtrip.rs but has no fixture"
        );
    }

    libtest_mimic::run(&args, tests).exit();
}
