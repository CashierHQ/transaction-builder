# `ic-icrc-tx` Crate build and parse icrc transaction

## Builder lib

This library using for building

- Icrc1TransferArg
- _and more_

to struct

```rust
pub struct CanisterCall {
    pub canister_id: String,
    pub method: String,
    pub arg: String,
}
```

## Parser lib

- Parsing the arg back to rust struct

- Parsing response from canister call to rust struct
