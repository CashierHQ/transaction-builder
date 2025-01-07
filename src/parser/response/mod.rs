use base64::{engine::general_purpose, Engine};
use candid::Decode;
use icrc_ledger_types::icrc1::transfer::{BlockIndex, TransferError};
pub struct Response {
    pub content_map: String,
}

pub type TransferResult = Result<BlockIndex, TransferError>;

pub fn parse_icrc1_response(response: Response) -> TransferResult {
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

        let result = parse_icrc1_response(response);

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_parser_icrc1_success_response() {
        let content_map = "RElETAhrAryKAX3F/tIBAWsI0cSYfALCkey5An+UwceJBAPrgqiXBAShw+v9BwXwh+bbCQaT5b7IDH/rnNvVDwdsAsfrxNAJccSYsbUNfWwBm7O+pgp9bAGLvfKbAX1sAb+bt/ANfWwBo7uRjAp4bAGcuracAn0BAACaAg==";
        let response = Response {
            content_map: content_map.to_string(),
        };

        let result = parse_icrc1_response(response);

        assert_eq!(result.is_ok(), true);
    }
}
