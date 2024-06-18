mod eip712;
mod encode;
mod error;
mod parser;
mod nft_helpers;
mod signing;

pub use crate::eip712::EIP712;
pub use crate::encode::hash_structured_data;
pub use crate::error::{Error, ErrorKind};
pub use crate::nft_helpers::*;
pub use crate::signing::sign_message;

#[cfg(test)]
mod tests {
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

        let domain_string = create_domain(name, version, chain_id, verifying_contract);
        let message_string = create_message(token_id, amount, to, nonce);
        let json = generate_eip712_json_string(&domain_string, &message_string);
        let hashed_structured_data = hash_structured_data_string(json);

        let private_key = 659918_u32;
        let signature = sign_message(&hashed_structured_data, private_key);
        let expected_signature = "b9c658f86d985ad0502584c70ea520cf68523e4013786f83f216de093ef9467e453d27fe627278ab0c8425906843a706f66a9c3120b37e88ac722aa217a04fcf1b";
        assert_eq!(signature, expected_signature);
    }
}
