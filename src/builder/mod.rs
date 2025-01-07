use base64::engine::general_purpose;
use base64::Engine;
use candid::Encode;

use crate::types::CanisterCall;

pub fn build_icrc1_transfer(
    canister_id: String,
    transfer_arg: icrc_ledger_types::icrc1::transfer::TransferArg,
) -> CanisterCall {
    CanisterCall {
        canister_id,
        method: "icrc1_transfer".to_string(),
        arg: general_purpose::STANDARD.encode(Encode!(&transfer_arg).unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::{Nat, Principal};
    use icrc_ledger_types::icrc1::{account::Account, transfer::TransferArg};

    #[test]
    fn test_build_icrc1_transfer() {
        let canister_id = "test_canister_id".to_string();
        let transfer_arg = TransferArg {
            to: Account {
                owner: Principal::from_text(
                    "6pfju-rc52z-aihtt-ahhg6-z2bzc-ofp5r-igp5i-qy5ep-j6vob-gs3ae-nae",
                )
                .unwrap(),
                subaccount: None,
            },
            amount: Nat::from(10000000u64),
            memo: None,
            fee: None,
            created_at_time: None,
            from_subaccount: None,
        };

        let result = build_icrc1_transfer(canister_id.clone(), transfer_arg.clone());

        println!("Encoded Arg: {}", result.arg);

        assert_eq!(result.canister_id, canister_id);
        assert_eq!(result.method, "icrc1_transfer".to_string());
        assert_eq!(
            result.arg,
            general_purpose::STANDARD.encode(Encode!(&transfer_arg).unwrap())
        );
    }
}
