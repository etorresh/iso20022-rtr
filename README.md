# rtr-iso20022

Serde types for the ISO 20022 JSON messages used by Payments Canada's Real-Time Rail (RTR). The types are generated from the RTR JSON schemas on SWIFT MyStandards and tested against the Exchange scenarios from the Payments Canada sandbox.

Supported messages, each wrapped in a `head.001` Business Application Header:

| Message | Name |
|---|---|
| `pacs.008` | FI to FI Customer Credit Transfer |
| `pacs.002` | FI to FI Payment Status Report |
| `pacs.028` | FI to FI Payment Status Request |
| `admi.002` | Message Reject |
| `admi.004` | System Event Notification |
| `admi.011` | System Event Acknowledgement |

## Usage
```rust
use rtr_iso20022::{Message, MessageType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(
        "tests/fixtures/Exchange Inbound - Send Payment/pacs.008_Credit Transfer.01/request.json",
    )?;
    let message: Message = serde_json::from_str(&json)?;

    match &message.payload {
        MessageType::Pacs008(transfer) => {
            println!("credit transfer {}", *transfer.group_header.message_identification);
        }
        MessageType::Admi002(reject) => {
            println!("rejected: {}", *reject.reason.rejecting_party_reason);
        }
        _ => {}
    }

    let round_tripped = serde_json::to_string_pretty(&message)?;
    println!("{round_tripped}");
    Ok(())
}
```

Deserialization enforces the schema's constraints, such as maximum lengths, patterns and enumerations, and rejects unknown fields inside the header and the message body.

## How the types are generated
The schemas in `schemas/` were downloaded from the Payments Canada RTR Usage Guidelines hosted on SWIFT MyStandards (retrieved August 10th 2026). `tools/schema-to-rust` turns each schema into a Rust module under `src/generated/` with [typify](https://github.com/oxidecomputer/typify). To regenerate them:

```sh
cargo r -p schema-to-rust
```

`src/lib.rs` ties the generated modules together into a single `Message` type.

## Testing
```sh
cargo test
```

The fixtures cover all 36 scenarios from the Exchange Inbound sheets of the sandbox test spreadsheet (Send Payment, Payment Inquiry and Heartbeat), and every sample request and response runs as its own test. Each one is deserialized, checked against the expected message type, serialized again, and compared with the original JSON. A few sample requests are invalid on purpose; those are listed at the top of `tests/roundtrip.rs`.

## Notes
- `pacs.008` Amount Type: Both SWIFT MyStandards and the [pacs.008 PDF spec](https://www.payments.ca/sites/default/files/RTR_FItoFI_CustomerCreditTransfer_pacs.008.pdf) define `ActiveCurrencyAndAmount_2decimals---copy.amount` as a string (e.g. `"100.00"`), but the [sandbox test spreadsheet](https://developer.payments.ca/sites/default/files/2025-09/RTR_Sandbox_API_Test_Scenarios_v3.0_1.xlsx) passes it as a JSON number (e.g. `"amount": 100`). I emailed Payments Canada, and the ISO 20022 team recommended following the JSON Schema on MyStandards as the most up-to-date specification. Since the schema types the field as a string, this parser follows the schema and the fixtures are updated to use strings. `exchange_rate` (`BaseOneRate`) is also a string in the schema, so the fixtures use a string for it too.

## Manual changes to the test fixtures
The fixtures in `tests/fixtures/` are extracted from the [sandbox test spreadsheet](https://developer.payments.ca/sites/default/files/2025-09/RTR_Sandbox_API_Test_Scenarios_v3.0_1.xlsx) (v3.0_1) with `cargo r -p extract-fixtures`. The extractor copies the samples as they are, so the fixes below were applied by hand. They are already included in the fixtures in this repo. You only need to repeat them if you run the extractor yourself.

| Category | Test ID | Issue | Workaround |
|---|---|---|---|
| Exchange Inbound - Send Payment | `pacs.008_Credit Transfer.01` to `.21` (requests) | `amount` is a JSON number, but the schema defines it as a string. | Quote every `amount` value, e.g. `100` to `"100"`. |
| Exchange Inbound - Send Payment | `pacs.008_Credit Transfer.02`, `.03` (requests) | `exchange_rate` is a JSON number, but the schema defines it as a string. | Quote the value, e.g. `1.59` to `"1.59"`. |
| Exchange Inbound - Send Payment | `pacs.008_Credit Transfer.05` (response) | `reason_description` contains raw line breaks, so it is not valid JSON. | Join the lines with single spaces. |

## Licence
Dual-licensed under either:
 * Apache Licence, Version 2.0 ([LICENCE-APACHE](LICENCE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT Licence ([LICENCE-MIT](LICENCE-MIT) or <http://opensource.org/licenses/MIT>)

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 licence, shall be dual licensed as above, without any additional terms or conditions.
