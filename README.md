
# EIP-712 Rust Utils


## Overview
The `eip_712_utils` crate is a Rust library providing utilities for EIP-712 message signing. This library is designed for simple creation, signing, and validation of EIP-712 structured data, particularly for NFTs (Non-Fungible Tokens).

## Features
- Generate EIP-712 compliant structured data
- Hash structured data as per EIP-712 specifications
- Sign EIP-712 structured data


## Installation
`cargo add eip_712_utils`

## Usage


### Generate and Sign Structured Data

```rust

#[cfg(test)]
mod tests {
    use rustc_hex::ToHex;
    use serde_json::json;

    use super::*;

    #[test]
    fn generate_hash_and_sign_it() {
        let name = "AionRisingNFTs";
        let version = "0.0.1";
        let chain_id = "0x7A69";
        let verifying_contract = "0x037eDa3aDB1198021A9b2e88C22B464fD38db3f3";
        let token_id = "0x1";
        let amount = "0x1";
        let to = "0x7FA9385bE102ac3EAc297483Dd6233D62b3e1496";
        let nonce = "0x1";

        let custom_fields = vec![
            FieldType {
                name: "tokenId".to_string(),
                type_: "uint256".to_string(),
            },
            FieldType {
                name: "amount".to_string(),
                type_: "uint256".to_string(),
            },
            FieldType {
                name: "to".to_string(),
                type_: "address".to_string(),
            },
            FieldType {
                name: "nonce".to_string(),
                type_: "uint256".to_string(),
            },
        ];

        let data: EIP712 = EIP712::builder()
            .domain(name, version, chain_id, verifying_contract)
            .custom_field(("NFTData".to_string(), custom_fields))
            .message(json!({
                "tokenId": token_id,
                "amount": amount,
                "to": to,
                "nonce": nonce
            }))
            .build();

        let hashed_structured_data = hash_structured_data(data).unwrap().to_hex::<String>();

        let private_key = 659918_u32;
        let signature = sign_message(&hashed_structured_data, private_key);
        let expected_signature = "b9c658f86d985ad0502584c70ea520cf68523e4013786f83f216de093ef9467e453d27fe627278ab0c8425906843a706f66a9c3120b37e88ac722aa217a04fcf1b";
        assert_eq!(signature, expected_signature);
    }
}

```

## API Reference

### Modules

#### `eip712`
This module provides the core functionality for EIP-712 encoding and hashing.

- **EIP712**
  - Represents the EIP-712 structured data.
  - Methods:
    - `builder() -> EIP712Builder`: Returns a new builder for EIP712.
    - `domain(name, version, chain_id, verifying_contract)`: Sets the domain parameters.
    - `custom_field((field_name, fields))`: Adds custom fields to the EIP-712 structure. Here you can use your NFT specific Data. 
    Imporant to note that this field will be used as the root node of the hashed data. If it's not provided, the root node used will be `EIP712Domain`.
    - `message(message)`: Sets the message data. Simplest way is to use `json!` from `serde` as shown in the example.
    - `build() -> EIP712`: Builds the EIP-712 structured data.

- **EIP712Domain**
  - Represents the EIP-712 domain.
  - Fields: `name`, `version`, `chain_id`, `verifying_contract`.

- **FieldType**
  - Represents a field type in the EIP-712 structure.
  - Fields: `name`, `type_`.

- **MessageTypes**
  - Represents the types of messages in the EIP-712 structure.

- **hash_structured_data(data: EIP712) -> Result<H256, Error>**
  - Hashes the EIP-712 structured data.
  - Parameters: `data` - The EIP-712 structured data.
  - Returns: The hash of the structured data.

#### `nft_helpers`
This module contains helper functions specifically for NFT-related operations.

- **hash_structured_data_string(data: String) -> Result<H256, Error>**
  - Hashes EIP-712 structured data provided as a JSON string.
  - Parameters: `data` - The JSON string representing the EIP-712 structured data.
  - Returns: The hash of the structured data.

#### `signing`
This module provides utilities for signing EIP-712 messages.

- **sign_message(message: &Message, private_key: &SecretKey) -> Signature**
  - Signs the given EIP-712 message.
  - Parameters: `message` - The EIP-712 message to sign, `private_key` - The private key used to sign the message.
  - Returns: The signature of the message.

## Running Tests
Run the tests with:

```sh
cargo test
```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License
This project is licensed under the MIT License.
