use uuid::Uuid;
use rusoto_dynamodb::KeySchemaElement;
use crate::dht::bbdht::dynamodb::schema::hash_key;
use rusoto_dynamodb::AttributeDefinition;
use crate::dht::bbdht::dynamodb::schema::string_attribute;

pub fn primary_key_attribute_name_a() -> String {
    "key_a".into()
}

pub fn attribute_name_fresh() -> String {
    format!("key_{}", Uuid::new_v4())
}

pub fn key_schema_a() -> Vec<KeySchemaElement> {
    vec![hash_key(&primary_key_attribute_name_a())]
}

pub fn attribute_definitions_a() -> Vec<AttributeDefinition> {
    vec![string_attribute(&primary_key_attribute_name_a())]
}
