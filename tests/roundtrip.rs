use rtr_iso20022::{Pacs002, Pacs008};
use serde::{Serialize, de::DeserializeOwned};

pub fn assert_roundtrip<T: Serialize + DeserializeOwned>(raw_json: &str) {
    println!("Parsing the raw string to JSON AST");
    let original_ast: serde_json::Value = serde_json::from_str(raw_json).unwrap();
    println!("Parsing the raw string to my custom struct");
    let parsed_struct: T = serde_json::from_str(raw_json).unwrap();
    println!("Parsing my custom struct to JSON AST");
    let new_ast: serde_json::Value = serde_json::to_value(&parsed_struct).unwrap();
    assert_eq!(original_ast, new_ast);
}

macro_rules! test_roundtrip {
    // 3 args: request and response
    ($folder_name:ident, $req_ty:ty, $res_ty:ty) => {
        mod $folder_name {
            use super::*;
            #[test]
            fn request() {
                assert_roundtrip::<$req_ty>(include_str!(concat!(
                    "./data/",
                    stringify!($folder_name),
                    "/request.json"
                )));
            }
            #[test]
            fn response() {
                assert_roundtrip::<$res_ty>(include_str!(concat!(
                    "./data/",
                    stringify!($folder_name),
                    "/response.json"
                )));
            }
        }
    };

    // 2 args: request only
    ($folder_name:ident, $req_ty:ty) => {
        mod $folder_name {
            use super::*;
            #[test]
            fn request() {
                assert_roundtrip::<$req_ty>(include_str!(concat!(
                    "./data/",
                    stringify!($folder_name),
                    "/request.json"
                )));
            }
        }
    };
}

test_roundtrip!(incoming_pacs_008_happy_path, Pacs008);
