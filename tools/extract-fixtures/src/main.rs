// cargo r -p extract-fixtures
use std::fs;

use calamine::{DataType, Reader, Xlsx, open_workbook};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum ResponseKind {
    Iso(String),
    Http(u16),
}

/// Written next to the fixtures as `meta.json`.
#[derive(Debug, PartialEq, Serialize)]
struct Meta {
    request: String,
    response: ResponseKind,
}

/// One row of a test-scenario sheet.
struct Case {
    id: String,
    request: String,
    response: Option<String>,
    meta: Meta,
}

/// Parses a "Message Type" cell such as
/// "Participant -> Sandbox: pacs.008\nSandbox -> Participant: pacs.002 ".
fn parse_message_type(cell: &str) -> Result<Meta, String> {
    let lines: Vec<&str> = cell
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let [request_line, response_line] = lines[..] else {
        return Err(format!("expected 2 lines, found {}", lines.len()));
    };
    let after_colon = |line: &str| -> Result<String, String> {
        line.rsplit_once(':')
            .map(|(_, rest)| rest.trim().to_string())
            .ok_or_else(|| format!("no `:` in `{line}`"))
    };

    let request = after_colon(request_line)?;
    if !is_iso_code(&request) {
        return Err(format!("request `{request}` is not an ISO message code"));
    }

    let response = after_colon(response_line)?;
    let response = if let Some(status) = response.strip_prefix("HTTP ") {
        let status = status
            .trim()
            .parse::<u16>()
            .map_err(|_| format!("bad HTTP status in `{response}`"))?;
        ResponseKind::Http(status)
    } else if is_iso_code(&response) {
        ResponseKind::Iso(response)
    } else {
        return Err(format!("unrecognised response `{response}`"));
    };

    Ok(Meta { request, response })
}

/// Matches codes like "pacs.008": four lowercase letters, a dot, three digits.
fn is_iso_code(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 8
        && b[..4].iter().all(u8::is_ascii_lowercase)
        && b[4] == b'.'
        && b[5..].iter().all(u8::is_ascii_digit)
}

fn main() {
    let path_sheet = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data/RTR_Sandbox_API_Test_Scenarios_v3.0_1.xlsx");
    let mut workbook: Xlsx<_> = open_workbook(path_sheet).unwrap();
    let sheet_names = workbook.sheet_names();
    // the first two sheets are "Version History", and "Guidelines" so we skip them, and only work with the sheets with sample JSON
    let sheet_targets = &sheet_names[2..];

    let mut test_dataset = Vec::new();
    for sheet in sheet_targets {
        let range = workbook.worksheet_range(sheet).unwrap();
        let mut rows = range.rows();
        let header_row = rows.next().unwrap();

        let column = |header: &str| -> usize {
            header_row
                .iter()
                .position(|cell| cell.get_string() == Some(header))
                .unwrap_or_else(|| panic!("Column `{header}` not found in sheet `{sheet}`"))
        };
        let col_id = column("Test ID");
        let col_message_type = column("Message Type");
        let col_request = column("Sample Request");
        let col_response = column("Sample Response");

        let cell = |row: &[calamine::Data], idx: usize, header: &str| -> String {
            row.get(idx)
                .and_then(|c| c.as_string())
                .unwrap_or_else(|| panic!("Missing `{header}` cell in sheet `{sheet}`"))
        };

        let mut cases = Vec::new();
        for row in rows {
            let test_id = cell(row, col_id, "Test ID");
            let meta = parse_message_type(&cell(row, col_message_type, "Message Type"))
                .unwrap_or_else(|err| {
                    panic!("Bad `Message Type` for `{test_id}` in sheet `{sheet}`: {err}")
                });
            let request = cell(row, col_request, "Sample Request");
            let response = {
                let res = cell(row, col_response, "Sample Response");
                res.starts_with("{").then_some(res)
            };

            // The Message Type column and the Sample Response cell are independent signals; they must agree.
            match (&meta.response, &response) {
                (ResponseKind::Iso(_), Some(_)) | (ResponseKind::Http(_), None) => {}
                (ResponseKind::Iso(code), None) => panic!(
                    "`{test_id}` in sheet `{sheet}` declares response `{code}` but has no JSON body"
                ),
                (ResponseKind::Http(status), Some(_)) => panic!(
                    "`{test_id}` in sheet `{sheet}` declares HTTP {status} but has a JSON body"
                ),
            }

            cases.push(Case {
                id: sanitize_filename::sanitize(test_id),
                request,
                response,
                meta,
            });
        }
        test_dataset.push((sanitize_filename::sanitize(sheet), cases));
    }

    let path_fixtures =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures");
    for (name, cases) in test_dataset {
        let path_sheet = path_fixtures.join(name);
        fs::create_dir_all(&path_sheet).expect("Failed to create sheet directory");

        for case in cases {
            let path_case = path_sheet.join(&case.id);
            fs::create_dir_all(&path_case).expect("Failed to create case directory");
            fs::write(path_case.join("request.json"), case.request).unwrap();
            if let Some(response) = case.response {
                fs::write(path_case.join("response.json"), response).unwrap();
            }
            let meta = serde_json::to_string_pretty(&case.meta).unwrap();
            fs::write(path_case.join("meta.json"), meta + "\n").unwrap();
        }
    }
}
