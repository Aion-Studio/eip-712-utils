//! EIP712 structs
use ethereum_types::{Address, H256, U256};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;
use validator::ValidationErrors;

use once_cell::sync::Lazy;

use crate::create_domain;

pub type MessageTypes = HashMap<String, Vec<FieldType>>;

static TYPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*(\[([1-9]\d*)*\])*$").unwrap());
static IDENT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_$][a-zA-Z_$0-9]*$").unwrap());

#[derive(Deserialize, Serialize, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EIP712Domain {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) chain_id: U256,
    pub(crate) verifying_contract: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) salt: Option<H256>,
}

impl EIP712Domain {
    pub fn new(name: &str, version: &str, chain_id: &str, verifying_contract: &str) -> Self {
        let json_data = create_domain(name, version, chain_id, verifying_contract);
        let domain: EIP712Domain = match serde_json::from_str(&json_data) {
            Ok(domain) => domain,
            Err(e) => {
                println!("Error parsing EIP712Domain {:?}", e);
                panic!()
            }
        };
        domain
    }
}

#[derive(Default)]
pub struct EIP712Builder {
    domain: Option<EIP712Domain>,
    message: Option<Value>,
    custom_field: Option<(String, Vec<FieldType>)>,
}

impl EIP712Builder {
    pub fn domain(
        mut self,
        name: &str,
        version: &str,
        chain_id: &str,
        verifying_contract: &str,
    ) -> Self {
        self.domain = Some(EIP712Domain::new(
            name,
            version,
            chain_id,
            verifying_contract,
        ));
        self
    }

    pub fn custom_field(mut self, field_types: (String, Vec<FieldType>)) -> Self {
        self.custom_field = Some(field_types);
        self
    }

    pub fn message(mut self, message: Value) -> Self {
        self.message = Some(message);
        self
    }

    pub fn build(self) -> EIP712 {
        EIP712::new(
            self.domain.expect("Domain must be set"),
            self.message.expect("Message must be set"),
            self.custom_field,
        )
    }
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

impl EIP712 {
    pub fn builder() -> EIP712Builder {
        EIP712Builder::default()
    }
    pub(crate) fn new(
        domain: EIP712Domain,
        message: Value,
        custom_field: Option<(String, Vec<FieldType>)>,
    ) -> Self {
        let mut types = HashMap::new();
        let field_types_eip_domain = vec![
            FieldType {
                name: "name".to_string(),
                type_: "string".to_string(),
            },
            FieldType {
                name: "version".to_string(),
                type_: "string".to_string(),
            },
            FieldType {
                name: "chainId".to_string(),
                type_: "uint256".to_string(),
            },
            FieldType {
                name: "verifyingContract".to_string(),
                type_: "address".to_string(),
            },
        ];

        types.insert("EIP712Domain".to_string(), field_types_eip_domain);

        let mut primary_type = "EI712Domain".to_string();
        if let Some((type_name, fields)) = custom_field {
            primary_type = type_name.clone();
            types.insert(type_name, fields);
        }

        Self {
            primary_type,
            types,
            message,
            domain,
        }
    }
    pub fn add_type(&mut self, type_name: String, fields: Vec<FieldType>) {
        self.types.insert(type_name, fields);
    }
}

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
pub struct FieldType {
    #[validate(regex(path = *IDENT_REGEX))]
    pub name: String,
    #[serde(rename = "type")]
    #[validate(regex(path = *TYPE_REGEX))]
    pub type_: String,
}
