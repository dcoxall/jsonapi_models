#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use std::collections::HashMap;

pub type ID = String;
pub type Type = String;
pub type StatusCode = i16;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
}

pub type Meta = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Relationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResourceLinkage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

pub type Relationships = HashMap<String, Relationship>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LinkObject {
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Link {
    Value(String),
    Object(LinkObject),
}

pub type Links = HashMap<String, Link>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

pub type Errors = Vec<Error>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ResourceIdentifier {
    pub id: ID,

    #[serde(rename = "type")]
    pub _type: Type,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

pub type Attributes = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Resource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,

    #[serde(rename = "type")]
    pub _type: Type,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Attributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Relationships>,
}

pub type Resources = Vec<Resource>;
pub type ResourceIdentifiers = Vec<ResourceIdentifier>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum PrimaryData {
    Singular(Resource),
    Multiple(Resources),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum ResourceLinkage {
    None,
    Singular(ResourceIdentifier),
    Multiple(ResourceIdentifiers),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct JsonApiObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct SuccessDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PrimaryData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<JsonApiObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Resources>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ErrorDocument {
    pub errors: Errors,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonapi: Option<JsonApiObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included: Option<Resources>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Document {
    Error(ErrorDocument),
    Success(SuccessDocument),
}
