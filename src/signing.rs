use hex::{self, encode};

pub fn sign_message(message_hex: &str, private_key_uint: u32) -> String {
    let message = hex::decode(message_hex).expect("Decoding failed");

    // private key to [u8]
    let mut private_key_bytes = [0u8; 32];
    private_key_bytes[28..].copy_from_slice(&private_key_uint.to_be_bytes());

    // let secret_key =
    //     SecretKey::from_slice(&private_key_bytes).expect("32 bytes, within curve order");
    // let secp = Secp256k1::new();

    // let message = Message::from_digest_slice(&message).expect("32 bytes");

    // let sig = secp.sign_ecdsa_recoverable(&message, &secret_key);

    // let (rec_id, signature_bytes) = sig.serialize_compact();

    // println!("rec_id: {:?}", rec_id);

    // // println!("Signature: {:?}", sig);
    // // let signature_bytes = sig.serialize_compact();

    // // Convert to hex
    // let signature_hex = encode(signature_bytes); // Use 'encode' from the 'hex' crate

    // // goal: 0xb9c658f86d985ad0502584c70ea520cf68523e4013786f83f216de093ef9467e
    // // result: b9c658f86d985ad0502584c70ea520cf68523e4013786f83f216de093ef9467e453d27fe627278ab0c8425906843a706f66a9c3120b37e88ac722aa217a04fcf00

    // println!("Signature (hex): {}", signature_hex);
    // if rec_id.to_i32() == 0 {
    //     // append 27 in hex to the signature_hex
    //     let signature_hex = format!("{}{:x}", signature_hex, 27);
    //     println!("{}", signature_hex);
    // } else {
    //     let signature_hex = format!("{}{:x}", signature_hex, 29);
    //     println!("{}", signature_hex);
    // }
    "yo".to_string()
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
