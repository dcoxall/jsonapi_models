#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use std::collections::HashMap;

pub type ID = String;
pub type Type = String;
pub type StatusCode = i16;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Source {
    pub pointer: Option<String>,
    pub parameter: Option<String>,
}

pub type Meta = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Relationship {
    pub links: Option<Links>,
    pub data: Option<ResourceLinkage>,
    pub meta: Option<Meta>,
}

pub type Relationships = HashMap<String, Relationship>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LinkObject {
    pub href: String,
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
    pub id: Option<ID>,
    pub links: Option<Links>,
    pub status: Option<StatusCode>,
    pub code: Option<String>,
    pub title: Option<String>,
    pub detail: Option<String>,
    pub source: Option<Source>,
    pub meta: Option<Meta>,
}

pub type Errors = Vec<Error>;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ResourceIdentifier {
    pub id: ID,

    #[serde(rename = "type")]
    pub _type: Type,

    pub meta: Option<Meta>,
}

pub type Attributes = HashMap<String, serde_json::Value>;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Resource {
    pub id: Option<ID>,

    #[serde(rename = "type")]
    pub _type: Type,

    pub meta: Option<Meta>,
    pub links: Option<Links>,
    pub attributes: Option<Attributes>,
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
    pub version: Option<String>,
    pub meta: Option<Meta>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SuccessDocument {
    pub data: Option<PrimaryData>,
    pub meta: Option<Meta>,
    pub jsonapi: Option<JsonApiObject>,
    pub links: Option<Links>,
    pub included: Option<Resources>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ErrorDocument {
    pub errors: Errors,
    pub meta: Option<Meta>,
    pub jsonapi: Option<JsonApiObject>,
    pub links: Option<Links>,
    pub included: Option<Resources>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Document {
    Error(ErrorDocument),
    Success(SuccessDocument),
}
