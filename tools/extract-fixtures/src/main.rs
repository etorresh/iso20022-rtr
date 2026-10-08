use std::fs;

use calamine::{Data, DataType, Reader, Xlsx, open_workbook};

// cargo r -p extract-fixtures
fn main() {
    let path_sheet = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data/RTR_Sandbox_API_Test_Scenarios_v3.0_1.xlsx");
    let mut workbook: Xlsx<_> = open_workbook(path_sheet).unwrap();
    let sheet_names = workbook.sheet_names();
    // the first two sheets are "Version History", and "Guidelines" so we skip them, and only work with the sheets with sample JSON
    let sheet_targets = &sheet_names[2..];
    let header_targets = ["Test ID", "Sample Request", "Sample Response"];
    let ranges: Vec<_> = sheet_targets
        .iter()
        .map(|sheet| (sheet, workbook.worksheet_range(sheet).unwrap()))
        .collect();

    let mut all_sheets = Vec::new();
    for (sheet, range) in &ranges {
        let mut rows = range.rows();
        let header_row = rows.next().unwrap();

        let target_indices: Vec<usize> = header_targets
            .iter()
            .filter_map(|target| {
                header_row
                    .iter()
                    .position(|cell| cell.get_string() == Some(target))
            })
            .collect();

        let mut all_rows = Vec::new();
        for row in rows {
            let selected_cells: Vec<&Data> = target_indices
                .iter()
                .map(|&idx| row.get(idx).unwrap())
                .collect();
            all_rows.push(selected_cells);
        }
        all_sheets.push((sheet, all_rows));
    }

    let mut test_dataset = Vec::new();
    for sheet in all_sheets {
        let (name, rows) = sheet;
        let mut tests = Vec::new();
        for row in rows {
            let test_id = sanitize_filename::sanitize(row[0].as_string().unwrap());
            let sample_request = row[1].as_string().unwrap();
            let sample_response = {
                let res = row[2].as_string().unwrap();
                if res.starts_with("{") {
                    Some(res)
                } else {
                    None
                }
            };
            tests.push((test_id, sample_request, sample_response));
        }
        let name = sanitize_filename::sanitize(name);
        test_dataset.push((name, tests));
    }

    let path_fixtures =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures");
    for (name, sheet) in test_dataset {
        let path_sheet = path_fixtures.join(name);
        fs::create_dir_all(&path_sheet).expect("Failed to create sheet directory");

        for row in sheet {
            let (test_id, request_sample, response_sample_opt) = row;
            let path_test_id = path_sheet.join(test_id);
            fs::create_dir_all(&path_test_id).expect("Failed to create sheet directory");
            fs::write(path_test_id.join("request.json"), request_sample).unwrap();
            if let Some(response_sample) = response_sample_opt {
                fs::write(path_test_id.join("response.json"), response_sample).unwrap();
            }
        }
    }
}
