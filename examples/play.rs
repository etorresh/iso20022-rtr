use rtr_iso20022;
use std::{fs, io::BufReader};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let file =
        std::fs::File::open("samples_rtr_iso_20022/incoming_pacs.008_happy_path/request.json")?;
    let reader = BufReader::new(file);
    // let message: rtr_iso20022::Pacs008 = serde_json::from_reader(reader)?;
    Ok(())
}
