use parse_display::{Display, FromStr};
use serde_derive::{Deserialize, Serialize};
use serde_plain::*;
use std::fmt::{Display as StdDisplay, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub versions: Versions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<Field>>,
    pub r#type: FieldType,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub map_key: bool,
    #[serde(default, skip_serializing_if = "Versions::is_none")]
    pub nullable_versions: Versions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignorable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagged_versions: Option<Versions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zero_copy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Struct {
    pub name: String,
    pub versions: Versions,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Schema {
    pub name: String,
    pub valid_versions: Versions,
    pub fields: Vec<Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<i16>,
    pub r#type: SchemaType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub common_structs: Vec<Struct>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flexible_versions: Option<Versions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) enum SchemaType {
    Request,
    Response,
    Data,
    Header,
    Metadata,
}

impl ToString for SchemaType {
    fn to_string(&self) -> String {
        match self {
            SchemaType::Request => String::from("Request"),
            SchemaType::Response => String::from("Response"),
            SchemaType::Data => String::from("Data"),
            SchemaType::Header => String::from("Header"),
            SchemaType::Metadata => String::from("Metadata"),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Listener {
    ZkBroker,
    Broker,
    Controller,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveType {
    Bool,
    Int8,
    Int16,
    Uint16,
    Int32,
    Int64,
    Float64,
    String,
    Bytes,
    Records,
    Uuid,
}

derive_display_from_serialize!(PrimitiveType);
derive_fromstr_from_deserialize!(PrimitiveType);

#[derive(Debug, Clone)]
pub enum FieldType {
    Primitive(PrimitiveType),
    Struct(String),
    Array(Box<FieldType>),
}

impl StdDisplay for FieldType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Primitive(p) => write!(f, "{}", p),
            FieldType::Struct(s) => write!(f, "{}", s),
            FieldType::Array(a) => write!(f, "[]{}", a),
        }
    }
}

impl FieldType {
    pub fn is_array(&self) -> bool {
        matches!(self, FieldType::Array(_))
    }
}

impl FromStr for FieldType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(if let Some(prefix) = s.strip_prefix("[]") {
            Self::Array(Box::new(Self::from_str(prefix)?))
        } else if let Ok(prim) = PrimitiveType::from_str(s) {
            Self::Primitive(prim)
        } else {
            Self::Struct(s.into())
        })
    }
}

derive_serialize_from_display!(FieldType);
derive_deserialize_from_fromstr!(FieldType, "valid type specification");

#[derive(Debug, Copy, Clone, Display, FromStr, Eq, PartialEq)]
pub enum Versions {
    #[display("none")]
    #[from_str(regex = r"none")]
    None,
    #[display("{0}")]
    #[from_str(regex = r"(?P<0>\d+)")]
    Exact(i16),
    #[display("{0}+")]
    #[from_str(regex = r"(?P<0>\d+)\+")]
    Since(i16),
    #[display("{0}-{1}")]
    #[from_str(regex = r"(?P<0>\d+)-(?P<1>\d+)")]
    Range(i16, i16),
}

derive_serialize_from_display!(Versions);
derive_deserialize_from_fromstr!(Versions, "valid version specification");

impl Versions {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}

impl Default for Versions {
    fn default() -> Self {
        Versions::None
    }
}
