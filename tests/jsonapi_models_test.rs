extern crate jsonapi_models;
#[macro_use]
extern crate serde_json;

use jsonapi_models::*;
use std::collections::HashMap;

#[test]
fn deserialize_success_document_simple() {
    let serialized = r#"{
        "data": {
            "type": "example",
            "id": "123"
        }
    }"#;
    let expected = Resource {
        id: Some(String::from("123")),
        _type: String::from("example"),
        ..Default::default()
    };
    let document: Document =
        serde_json::from_str(serialized).expect("Failed to deserialize");

    match document {
        Document::Success(doc) => {
            match doc.data.expect("Unable to locate data") {
                PrimaryData::Singular(resource) => assert_eq!(expected, resource),
                _ => assert!(false),
            }
        },
        Document::Error(_) => assert!(false),
    }
}

#[test]
fn deserialize_success_document_advanced() {
    let serialized = r#"{
        "data": {
            "type": "example",
            "id": "123",
            "attributes": {
                "name": "advanced",
                "duration": 987
            },
            "links": {
                "self": {
                    "href": "https://example.org/examples/123"
                }
            },
            "relationships": {
                "connections": {
                    "data": [
                        {
                            "type": "foo",
                            "id": "foo-123"
                        },
                        {
                            "type": "foo",
                            "id": "foo-234"
                        }
                    ]
                }
            }
        }
    }"#;

    let mut attributes: Attributes = HashMap::new();
    attributes.insert(String::from("name"), serde_json::Value::String(String::from("advanced")));
    attributes.insert(String::from("duration"), serde_json::Value::Number(serde_json::Number::from(987)));

    let mut links: Links = HashMap::new();
    links.insert(String::from("self"),
        Link::Object(
            LinkObject {
                href: String::from("https://example.org/examples/123"),
                meta: None,
            }));

    let mut relationships: Relationships = HashMap::new();
    relationships.insert(String::from("connections"),
        Relationship {
            data: Some(ResourceLinkage::Multiple(
                vec![
                    ResourceIdentifier { id: String::from("foo-123"), _type: String::from("foo"), meta: None },
                    ResourceIdentifier { id: String::from("foo-234"), _type: String::from("foo"), meta: None },
                ]
            )),
            ..Default::default()
        });

    let expected = Resource {
        id: Some(String::from("123")),
        _type: String::from("example"),
        attributes: Some(attributes),
        links: Some(links),
        relationships: Some(relationships),
        ..Default::default()
    };
    let document: SuccessDocument =
        serde_json::from_str(serialized).expect("Failed to deserialize");

    match document.data.expect("Unable to locate document data") {
        PrimaryData::Singular(resource) => assert_eq!(expected, resource),
        PrimaryData::Multiple(_) => assert!(false),
    }
}

#[test]
fn deserialize_error_document_simple() {
    let serialized = r#"{
        "errors": [
            {
                "id": "broken",
                "code": "your.code.broken",
                "source": {
                    "pointer": "data[0].foobar"
                }
            }
        ]
    }"#;
    let expected = Error {
        id: Some(String::from("broken")),
        code: Some(String::from("your.code.broken")),
        source: Some(Source {
            pointer: Some(String::from("data[0].foobar")),
            parameter: None,
        }),
        ..Default::default()
    };
    let document: Document =
        serde_json::from_str(serialized).expect("Failed to deserialize");

    match document {
        Document::Success(_) => assert!(false),
        Document::Error(doc) => assert_eq!(doc.errors[0], expected),
    }
}

#[test]
fn serialize_success_document_simple() {
    let document = Resource {
        id: Some(String::from("123")),
        _type: String::from("example"),
        ..Default::default()
    };
    let document = Document::Success(
        SuccessDocument {
            data: Some(PrimaryData::Singular(document)),
            ..Default::default()
        }
    );
    let document = serde_json::to_value(&document).unwrap();
    let expected = json!({
        "data": {
            "id": "123",
            "type": "example"
        }
    });
    assert_eq!(document, expected);
}

#[test]
fn serialize_success_document_advanced() {
    let expected = json!({
        "data": {
            "type": "example",
            "id": "123",
            "attributes": {
                "name": "advanced",
                "duration": 987
            },
            "links": {
                "self": {
                    "href": "https://example.org/examples/123"
                }
            },
            "relationships": {
                "connections": {
                    "data": [
                        {
                            "type": "foo",
                            "id": "foo-123"
                        },
                        {
                            "type": "foo",
                            "id": "foo-234"
                        }
                    ]
                }
            }
        }
    });

    let mut attributes: Attributes = HashMap::new();
    attributes.insert(String::from("name"), serde_json::Value::String(String::from("advanced")));
    attributes.insert(String::from("duration"), serde_json::Value::Number(serde_json::Number::from(987)));

    let mut links: Links = HashMap::new();
    links.insert(String::from("self"),
        Link::Object(
            LinkObject {
                href: String::from("https://example.org/examples/123"),
                meta: None,
            }));

    let mut relationships: Relationships = HashMap::new();
    relationships.insert(String::from("connections"),
        Relationship {
            data: Some(ResourceLinkage::Multiple(
                vec![
                    ResourceIdentifier { id: String::from("foo-123"), _type: String::from("foo"), meta: None },
                    ResourceIdentifier { id: String::from("foo-234"), _type: String::from("foo"), meta: None },
                ]
            )),
            ..Default::default()
        });

    let actual = Document::Success(
        SuccessDocument {
            data: Some(PrimaryData::Singular(
                Resource {
                    id: Some(String::from("123")),
                    _type: String::from("example"),
                    attributes: Some(attributes),
                    links: Some(links),
                    relationships: Some(relationships),
                    ..Default::default()
                }
            )),
            ..Default::default()
        }
    );

    let actual: serde_json::Value = serde_json::to_value(&actual).unwrap();
    assert_eq!(expected, actual);
}
