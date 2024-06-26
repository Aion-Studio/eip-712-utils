mod encode;
mod error;
mod parser;

pub use crate::encode::hash_structured_data;
pub use crate::error::{Error, ErrorKind};
pub use crate::nft_helpers::*;

pub mod eip712;
pub mod nft_helpers;
pub mod signing;

// Re-export EIP712 and EIP712Domain
pub use eip712::{EIP712Domain, FieldType, MessageTypes, EIP712};

// Re-export functions from nft_helpers
pub use nft_helpers::hash_structured_data_string;

// Re-export functions from signing
pub use signing::sign_message;

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

    #[test]
    fn it_hashes_data_correctly() {
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

        let result = hash_structured_data(data).unwrap().to_hex::<String>();
        println!("Hash: {:?}", result);
        assert_eq!(
            result,
            "77915d20c811f39572463a234db9b776d518d07d9682a825be0d79752745a4c7"
        );
    }
}
