# Parser for Payments Canada Real-Time Rail (RTR) ISO 20022

## Schemas
The schemas in `schemas/` were downloaded from the Payments Canada RTR Usage Guidelines hosted on SWIFT MyStandards (retrieved August 10th 2026)

## Notes
- `pacs.008` Amount Type: Both SWIFT MyStandards and the [pacs.008 PDF spec](https://www.payments.ca/sites/default/files/RTR_FItoFI_CustomerCreditTransfer_pacs.008.pdf) define `ActiveCurrencyAndAmount_2decimals---copy.amount` as a string (e.g. `"100.00"`). However, the [sandbox test spreadsheet](https://developer.payments.ca/sites/default/files/2025-09/RTR_Sandbox_API_Test_Scenarios_v3.0_1.xlsx) passes it as a raw integer (e.g. `"amount": 100`). I've emailed Payments Canada for clarification. For now, the local test scenarios are updated to use integers so they match the sandbox data.

## Licence
Dual-licensed under either:
 * Apache Licence, Version 2.0 ([LICENCE-APACHE](LICENCE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT Licence ([LICENCE-MIT](LICENCE-MIT) or http://opensource.org/licenses/MIT)

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 licence, shall be dual licensed as above, without any additional terms or conditions.
