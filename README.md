# Parser for Payments Canada Real-Time Rail (RTR) ISO 20022

## Schemas
The schemas in `schemas/` were downloaded from the Payments Canada RTR Usage Guidelines hosted on SWIFT MyStandards (retrieved August 10th 2026)

## Notes
- `pacs.008` Amount Type: Both SWIFT MyStandards and the [pacs.008 PDF spec](https://www.payments.ca/sites/default/files/RTR_FItoFI_CustomerCreditTransfer_pacs.008.pdf) define `ActiveCurrencyAndAmount_2decimals---copy.amount` as a string (e.g. `"100.00"`), but the [sandbox test spreadsheet](https://developer.payments.ca/sites/default/files/2025-09/RTR_Sandbox_API_Test_Scenarios_v3.0_1.xlsx) passes it as a JSON number (e.g. `"amount": 100`). I emailed Payments Canada, and the ISO 20022 team recommended following the JSON Schema on MyStandards as the most up-to-date specification. Their reply focused on decimal places (`100` and `100.00` are both accepted) rather than the string-versus-number question, but since the schema types the field as a string, this parser follows the schema and the fixtures are updated to use strings. The same applies to `exchange_rate` (`BaseOneRate`, also a string in the schema).

## Manual changes to the test fixtures
The fixtures in `tests/fixtures/` are extracted from the [sandbox test spreadsheet](https://developer.payments.ca/sites/default/files/2025-09/RTR_Sandbox_API_Test_Scenarios_v3.0_1.xlsx) (v3.0_1) with `cargo r -p extract-fixtures`. The extractor does not modify the samples, so the changes below must be applied by hand after every run.

| Category | Test ID | Issue | Workaround |
|---|---|---|---|
| Exchange Inbound - Send Payment | `pacs.008_Credit Transfer.01`–`.21` (requests) | `amount` is a JSON number, but the schema defines it as a string. | Quote every `amount` value, e.g. `100` → `"100"`. |
| Exchange Inbound - Send Payment | `pacs.008_Credit Transfer.02`, `.03` (requests) | `exchange_rate` is a JSON number, but the schema defines it as a string. | Quote the value, e.g. `1.59` → `"1.59"`. |
| Exchange Inbound - Send Payment | `pacs.008_Credit Transfer.05` (response) | `reason_description` contains raw line breaks, so it is not valid JSON. | Join the lines with single spaces, matching the other `admi.002` responses. |

## Licence
Dual-licensed under either:
 * Apache Licence, Version 2.0 ([LICENCE-APACHE](LICENCE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT Licence ([LICENCE-MIT](LICENCE-MIT) or http://opensource.org/licenses/MIT)

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 licence, shall be dual licensed as above, without any additional terms or conditions.
