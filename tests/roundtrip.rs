use rtr_iso20022::{Message, MessageType, MessageType::*};

pub fn assert_roundtrip(file_path: &str, is_expected_variant: impl FnOnce(&MessageType) -> bool) {
    let raw_json = std::fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Failed to load fixture file at: {}", file_path));
    println!("Parsing the raw string to JSON AST");
    let original_ast: serde_json::Value = serde_json::from_str(&raw_json).unwrap();
    println!("Parsing the raw string to my custom struct");
    let parsed_struct: Message = serde_json::from_str(&raw_json).unwrap();
    println!("Parsing my custom struct to JSON AST");
    let new_ast: serde_json::Value = serde_json::to_value(&parsed_struct).unwrap();
    assert_eq!(original_ast, new_ast);
    assert!(is_expected_variant(&parsed_struct.payload));
}

// rust-analyzer hates include_str! https://github.com/rust-lang/rust-analyzer/issues/10647 so I'm using format! and reading the file at run-time
macro_rules! test_roundtrip {
    ($folder_name:ident, $req_ty:path, $res_ty:path) => {
        mod $folder_name {
            use super::*;
            #[test]
            fn request() {
                let path = format!(
                    "{}/tests/fixtures/{}/request.json",
                    env!("CARGO_MANIFEST_DIR"),
                    stringify!($folder_name)
                );
                assert_roundtrip(&path, |message| matches!(message, $req_ty(..)));
            }
            #[test]
            fn response() {
                let path = format!(
                    "{}/tests/fixtures/{}/response.json",
                    env!("CARGO_MANIFEST_DIR"),
                    stringify!($folder_name)
                );
                assert_roundtrip(&path, |message| matches!(message, $res_ty(..)));
            }
        }
    };

    // 2 args: request only
    ($folder_name:ident, $req_ty:path) => {
        mod $folder_name {
            use super::*;
            #[test]
            fn request() {
                let path = format!(
                    "{}/tests/fixtures/{}/request.json",
                    env!("CARGO_MANIFEST_DIR"),
                    stringify!($folder_name)
                );
                assert_roundtrip(&path, |message| matches!(message, $req_ty(..)));
            }
        }
    };
}
test_roundtrip!(incoming_pacs_008_happy_path, Pacs008);
