// pub fn generate_hash(data: &str) -> String {
//     let json = r#"{
//         "primaryType": "NFTData",
//         "domain": {
//             "name": "AionRisingNFTs",
//             "version": "0.0.1",
//             "chainId": "0x7A69",
//             "verifyingContract": "0x037eDa3aDB1198021A9b2e88C22B464fD38db3f3"
//         },
//         "message": {
//             "tokenId": "0x1",
//             "amount": "0x1",
//             "to": "0x7FA9385bE102ac3EAc297483Dd6233D62b3e1496",
//             "nonce": "0x1"
//         },
//         "types": {
//             "EIP712Domain": [
//                 { "name": "name", "type": "string" },
//                 { "name": "version", "type": "string" },
//                 { "name": "chainId", "type": "uint256" },
//                 { "name": "verifyingContract", "type": "address" }
//             ],
//             "NFTData": [
//                 { "name": "tokenId", "type": "uint256" },
//                 { "name": "amount", "type": "uint256" },
//                 { "name": "to", "type": "address" },
//                 { "name": "nonce", "type": "uint256" }
//             ]
//         }
//     }"#;
//     let typed_data = from_str::<EIP712>(json).unwrap();
//      let result = hash_structured_data(typed_data).unwrap().to_hex::<String>();
//      assert_eq!(result, "77915d20c811f39572463a234db9b776d518d07d9682a825be0d79752745a4c7");
// }

pub fn create_domain_string(
    name: &str,
    version: &str,
    chain_id: &str,
    verifying_contract: &str,
) -> String {
    format!(
        r#"{{
        "name": "{}",
        "version": "{}",
        "chainId": "{}",
        "verifyingContract": "{}"
    }}"#,
        name, version, chain_id, verifying_contract
    )
}

pub fn generate_eip712_json_string(
    name: &str,
    version: &str,
    chain_id: &str,
    verifying_contract: &str,
) -> String {
    let domain = create_domain_string(name, version, chain_id, verifying_contract);
    // TODO: reuse this part
    let json = format!(
        r#"{{
        "primaryType": "NFTData",
        "domain": {},
        "message": {{
            "tokenId": "0x1",
            "amount": "0x1",
            "to": "0x7FA9385bE102ac3EAc297483Dd6233D62b3e1496",
            "nonce": "0x1"
        }},
        "types": {{
            "EIP712Domain": [
                {{ "name": "name", "type": "string" }},
                {{ "name": "version", "type": "string" }},
                {{ "name": "chainId", "type": "uint256" }},
                {{ "name": "verifyingContract", "type": "address" }}
            ],
            "NFTData": [
                {{ "name": "tokenId", "type": "uint256" }},
                {{ "name": "amount", "type": "uint256" }},
                {{ "name": "to", "type": "address" }},
                {{ "name": "nonce", "type": "uint256" }}
            ]
        }}
    }}"#,
        domain
    );

    json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_json() {
        let name = "AionRisingNFTs";
        let version = "0.0.1";
        let chain_id = "0x7A69";
        let verifying_contract = "0x037eDa3aDB1198021A9b2e88C22B464fD38db3f3";

        let json = generate_eip712_json_string(name, version, chain_id, verifying_contract);
        let expected_json = r#"{
            "primaryType": "NFTData",
            "domain": {
                "name": "AionRisingNFTs",
                "version": "0.0.1",
                "chainId": "0x7A69",
                "verifyingContract": "0x037eDa3aDB1198021A9b2e88C22B464fD38db3f3"
            },
            "message": {
                "tokenId": "0x1",
                "amount": "0x1",
                "to": "0x7FA9385bE102ac3EAc297483Dd6233D62b3e1496",
                "nonce": "0x1"
            },
            "types": {
                "EIP712Domain": [
                    { "name": "name", "type": "string" },
                    { "name": "version", "type": "string" },
                    { "name": "chainId", "type": "uint256" },
                    { "name": "verifyingContract", "type": "address" }
                ],
                "NFTData": [
                    { "name": "tokenId", "type": "uint256" },
                    { "name": "amount", "type": "uint256" },
                    { "name": "to", "type": "address" },
                    { "name": "nonce", "type": "uint256" }
                ]
            }
        }"#;

        let v1: serde_json::Value = serde_json::from_str(&json).unwrap();
        let v2: serde_json::Value = serde_json::from_str(&expected_json).unwrap();
        assert_eq!(v1, v2);
    }
}
