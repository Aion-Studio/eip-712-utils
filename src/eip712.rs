//! EIP712 structs
use serde_json::{Value};
use std::collections::HashMap;
use ethereum_types::{U256, H256, Address};
use regex::Regex;
use validator::{Validate,ValidationErrors};
use lazy_static::lazy_static;

pub(crate) type MessageTypes = HashMap<String, Vec<FieldType>>;

lazy_static! {
	// match solidity identifier with the addition of '[(\d)*]*'
	static ref TYPE_REGEX: Regex = Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*(\[([1-9]\d*)*\])*$").unwrap();
	static ref IDENT_REGEX: Regex = Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$").unwrap();
}

// #[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
// #[derive(Deserialize, Serialize, Validate, Debug, Clone)]
// pub(crate) struct EIP712Domain {
// 	pub(crate) name: String,
// 	pub(crate) version: String,
// 	pub(crate) chain_id: U256,
// 	pub(crate) verifying_contract: Address,
// 	#[serde(skip_serializing_if="Option::is_none")]
// 	pub(crate) salt: Option<H256>,
// }
// /// EIP-712 struct
// #[serde(rename_all = "camelCase")]
// #[serde(deny_unknown_fields)]
// #[derive(Deserialize, Debug, Clone)]
pub struct EIP712 {
	// pub(crate) types: MessageTypes,
	// pub(crate) primary_type: String,
	// pub(crate) message: Value,
	// pub(crate) domain: EIP712Domain,
}

// impl Validate for EIP712 {
// 	fn validate(&self) -> Result<(), ValidationErrors> {
// 		for field_types in self.types.values() {
// 			for field_type in field_types {
// 				field_type.validate()?;
// 			}
// 		}
// 		Ok(())
// 	}
// }

// #[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub(crate) struct FieldType {
	// #[validate(regex = "IDENT_REGEX")]
	// pub name: String,
	// #[serde(rename = "type")]
	// #[validate(regex = "TYPE_REGEX")]
	// pub type_: String,
}
