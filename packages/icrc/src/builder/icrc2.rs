use base64::engine::general_purpose;
use base64::Engine;
use candid::Encode;

use crate::types::CanisterCall;

pub fn build_icrc2_transfer(
    canister_id: String,
    transfer_arg: icrc_ledger_types::icrc2::approve::ApproveArgs,
) -> CanisterCall {
    CanisterCall {
        canister_id,
        method: "icrc2_approve".to_string(),
        arg: general_purpose::STANDARD.encode(Encode!(&transfer_arg).unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Nat;
    use icrc_ledger_types::{icrc1::account::Account, icrc2::approve::ApproveArgs};

    #[test]
    fn test_build_icrc2_transfer() {
        let canister_id = "rh2pm-ryaaa-aaaan-qeniq-cai".to_string();
        let transfer_arg = ApproveArgs {
            from_subaccount: None,
            spender: Account {
                owner: "6pfju-rc52z-aihtt-ahhg6-z2bzc-ofp5r-igp5i-qy5ep-j6vob-gs3ae-nae"
                    .parse()
                    .unwrap(),
                subaccount: None,
            },
            amount: Nat::from(10000000u64),
            expected_allowance: None,
            expires_at: None,
            fee: None,
            memo: None,
            created_at_time: None,
        };

        let result = build_icrc2_transfer(canister_id.clone(), transfer_arg.clone());

        println!("Encoded Arg: {}", result.arg);

        assert_eq!(result.canister_id, canister_id);
        assert_eq!(result.method, "icrc2_approve".to_string());
        assert_eq!(
            result.arg,
            general_purpose::STANDARD.encode(Encode!(&transfer_arg).unwrap())
        );
    }
}
