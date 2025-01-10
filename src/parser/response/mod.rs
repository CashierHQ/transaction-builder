use base64::{engine::general_purpose, Engine};
use candid::Decode;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferError};
/// A struct representing the response containing a base64-encoded content map.
pub struct Response {
    pub content_map: String,
}

/// The result type for a transfer, which can either be a `BlockIndex` or a `TransferError`.
pub type TransferResult = Result<BlockIndex, TransferError>;

/// Parses an ICRC1 transfer response.
///
/// This function takes a `Response` input, decodes the base64-encoded content map,
/// and deserializes it into a `TransferResult`.
///
/// # Arguments
///
/// * `response` - The `Response` containing the base64-encoded content map.
///
/// # Returns
///
/// A `TransferResult` which is either a `BlockIndex` or a `TransferError`.
///
/// # Panics
///
/// This function will panic if the base64 decoding or deserialization fails.
pub fn parse_icrc1_transfer_response(response: Response) -> TransferResult {
    // Decode the base64 string to bytes
    let result_bytes = general_purpose::STANDARD
        .decode(response.content_map)
        .unwrap();
    let result = Decode!(&result_bytes, TransferResult).unwrap();

    println!("Result: {:?}", result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_icrc1_err_response() {
        let content_map = "RElETAhrAryKAX3F/tIBAWsI0cSYfALCkey5An+UwceJBAPrgqiXBAShw+v9BwXwh+bbCQaT5b7IDH/rnNvVDwdsAsfrxNAJccSYsbUNfWwBm7O+pgp9bAGLvfKbAX1sAb+bt/ANfWwBo7uRjAp4bAGcuracAn0BAAEHAA==";

        let response = Response {
            content_map: content_map.to_string(),
        };

        let result = parse_icrc1_transfer_response(response);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_parser_icrc1_success_response() {
        let content_map = "RElETAhrAryKAX3F/tIBAWsI0cSYfALCkey5An+UwceJBAPrgqiXBAShw+v9BwXwh+bbCQaT5b7IDH/rnNvVDwdsAsfrxNAJccSYsbUNfWwBm7O+pgp9bAGLvfKbAX1sAb+bt/ANfWwBo7uRjAp4bAGcuracAn0BAACaAg==";
        let response = Response {
            content_map: content_map.to_string(),
        };

        let result = parse_icrc1_transfer_response(response);

        assert_eq!(result.is_ok(), true);
    }
}
