mod eip712;
mod encode;
mod error;
mod parser;
mod nft_helpers;

pub use crate::eip712::EIP712;
pub use crate::encode::hash_structured_data;
pub use crate::error::{Error, ErrorKind};
pub use crate::nft_helpers::*;
