//! EIP712 structs
use ethereum_types::{Address, H256, U256};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;
use validator::ValidationErrors;

use once_cell::sync::Lazy;

pub(crate) type MessageTypes = HashMap<String, Vec<FieldType>>;

static TYPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*(\[([1-9]\d*)*\])*$").unwrap());
static IDENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$").unwrap());

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct EIP712Domain {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) chain_id: U256,
    pub(crate) verifying_contract: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) salt: Option<H256>,
}
/// EIP-712 struct
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EIP712 {
    pub(crate) types: MessageTypes,
    pub(crate) primary_type: String,
    pub(crate) message: Value,
    pub(crate) domain: EIP712Domain,
}

impl Validate for EIP712 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        for field_types in self.types.values() {
            for field_type in field_types {
                // Borrow the `FieldType` to use the validate method
                field_type.validate()?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub(crate) struct FieldType {
    #[validate(regex(path = *IDENT_REGEX))]
    pub name: String,
    #[serde(rename = "type")]
    #[validate(regex(path = *TYPE_REGEX))]
    pub type_: String,
}
