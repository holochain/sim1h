use crate::dht::bbdht::dynamodb::schema::hash_key;
use crate::dht::bbdht::dynamodb::schema::string_attribute;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::KeySchemaElement;
use uuid::Uuid;

pub fn table_name_fresh() -> String {
    format!("table_{}", Uuid::new_v4())
}

pub fn primary_key_name_a() -> String {
    "key_a".into()
}

pub fn primary_key_name_fresh() -> String {
    format!("key_{}", Uuid::new_v4())
}

pub fn key_schema_a() -> Vec<KeySchemaElement> {
    vec![hash_key(&primary_key_name_a())]
}

pub fn attribute_definitions_a() -> Vec<AttributeDefinition> {
    vec![string_attribute(&primary_key_name_a())]
}
