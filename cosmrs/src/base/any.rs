use std::fmt;

use crate::{ErrorReport, Result};
use serde::{Deserialize, Serialize};

/// This is a wrapper for proto any, with serialize and deserialize implementations
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Any {
    /// type url for any type
    pub type_url: String,
    /// proto encoded value for type_url type
    pub value: Vec<u8>,
}

impl Any {
    /// constructor
    pub fn new(type_url: String, value: Vec<u8>) -> Self {
        Self { type_url, value }
    }
}

impl TryFrom<prost_types::Any> for Any {
    type Error = ErrorReport;

    fn try_from(any: prost_types::Any) -> Result<Any> {
        Any::try_from(&any)
    }
}

impl TryFrom<&prost_types::Any> for Any {
    type Error = ErrorReport;

    fn try_from(any: &prost_types::Any) -> Result<Any> {
        Ok(Any {
            type_url: any.type_url.clone(),
            value: any.value.clone(),
        })
    }
}

impl From<Any> for prost_types::Any {
    fn from(any: Any) -> prost_types::Any {
        prost_types::Any::from(&any)
    }
}

impl From<&Any> for prost_types::Any {
    fn from(any: &Any) -> prost_types::Any {
        prost_types::Any {
            type_url: any.type_url.clone(),
            value: any.value.clone(),
        }
    }
}


impl fmt::Display for Any {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type_url: {} \n value: {:?}",self.type_url,self.value)
    }
}
