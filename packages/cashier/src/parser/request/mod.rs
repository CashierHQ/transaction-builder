use base64::{engine::general_purpose, Engine};
use candid::Decode;
use icrc_ledger_types::icrc1::transfer::TransferArg;

use crate::types::CanisterCall;

/// Parses an ICRC1 transfer canister call.
///
/// This function takes a `CanisterCall` input, decodes the base64-encoded argument,
/// and deserializes it into a `TransferArg` struct.
///
/// # Arguments
///
/// * `caniser_call_input` - The `CanisterCall` containing the base64-encoded transfer argument.
///
/// # Returns
///
/// A `TransferArg` struct containing the decoded transfer arguments.
///
/// # Panics
///
/// This function will panic if the base64 decoding or deserialization fails.
pub fn parse_icrc1_transfer_arg(caniser_call_input: CanisterCall) -> TransferArg {
    let transfer_arg_bytes = general_purpose::STANDARD
        .decode(caniser_call_input.arg)
        .unwrap();

    let transfer_arg = Decode!(
        &transfer_arg_bytes,
        icrc_ledger_types::icrc1::transfer::TransferArg
    )
    .unwrap();
    transfer_arg
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{Nat, Principal};
    use icrc_ledger_types::icrc1::account::Account;

    #[test]
    fn test_parse_icrc1_transfer_arg() {
        let canister_call_input = CanisterCall {
            canister_id: "etik7-oiaaa-aaaar-qagia-cai".to_string(),
            method: "icrc1_transfer".to_string(),
            arg: "RElETAZsBvvKAQHG/LYCBLqJ5cIEAqLelOsGAoLz85EMBdijjKgNfWwCs7DawwNorYbKgwUCbgNte259bngBAAEdXdZAg85gOc3s6DkTiv7FBn9RDHSPT6rgmlsBGgIAAAAAAICt4gQ=".to_string(),
        };

        let transfer_arg = parse_icrc1_transfer_arg(canister_call_input);

        println!("Decoded Arg: {:?}", transfer_arg);

        assert_eq!(
            transfer_arg.to,
            Account {
                owner: Principal::from_text(
                    "6pfju-rc52z-aihtt-ahhg6-z2bzc-ofp5r-igp5i-qy5ep-j6vob-gs3ae-nae"
                )
                .unwrap(),
                subaccount: None,
            }
        );
        assert_eq!(transfer_arg.amount, Nat::from(10000000u64));
        assert_eq!(transfer_arg.memo, None);
        assert_eq!(transfer_arg.fee, None);
        assert_eq!(transfer_arg.created_at_time, None);
        assert_eq!(transfer_arg.from_subaccount, None);
    }
}
