pub fn create_domain(
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

pub fn create_message(token_id: &str, amount: &str, to: &str, nonce: &str) -> String {
    format!(
        r#"{{
        "tokenId": "{}",
        "amount": "{}",
        "to": "{}",
        "nonce": "{}"
    }}"#,
        token_id, amount, to, nonce
    )
}

pub fn generate_eip712_json_string(domain: &str, message: &str) -> String {
    let json = format!(
        r#"{{
        "primaryType": "NFTData",
        "domain": {},
        "message": {},
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
        domain, message
    );

    json
}