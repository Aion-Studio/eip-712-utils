use hex::{self, encode};
use secp256k1::{Message, Secp256k1, SecretKey};

pub fn sign_message(message_hex: &str, private_key_uint: u32) -> String {
    let message = hex::decode(message_hex).expect("Decoding failed");

    // private key to [u8]
    let mut private_key_bytes = [0u8; 32];
    private_key_bytes[28..].copy_from_slice(&private_key_uint.to_be_bytes());

    let secret_key =
        SecretKey::from_slice(&private_key_bytes).expect("32 bytes, within curve order");
    let secp = Secp256k1::new();

    let message = Message::from_digest_slice(&message).expect("32 bytes");

    let sig = secp.sign_ecdsa_recoverable(&message, &secret_key);

    let (rec_id, signature_bytes) = sig.serialize_compact();

    // Convert to hex
    let signature_hex = encode(signature_bytes); // Use 'encode' from the 'hex' crate

    if rec_id.to_i32() == 0 {
        // append 27 in hex to the signature_hex
        let signature_hex = format!("{}{:x}", signature_hex, 27);
        return signature_hex;
    } else {
        let signature_hex = format!("{}{:x}", signature_hex, 29);
        return signature_hex;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
	use serde_json::from_str;
	use rustc_hex::ToHex;

    #[test]
    fn it_signs_correctly() {
        let message = "77915d20c811f39572463a234db9b776d518d07d9682a825be0d79752745a4c7";

    }
}
