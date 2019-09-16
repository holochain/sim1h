use crate::dht::bbdht::dynamodb::schema::hash_key;
use crate::dht::bbdht::dynamodb::schema::string_attribute;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::KeySchemaElement;
use rusoto_dynamodb::ListTablesOutput;

pub fn table_name_a() -> &'static str {
    "table_a"
}

pub fn table_name_b() -> &'static str {
    "table_b"
}

pub fn primary_key_name_a() -> &'static str {
    "id"
}

pub fn key_schema_a() -> Vec<KeySchemaElement> {
    vec![hash_key(primary_key_name_a())]
}

pub fn attribute_definitions_a() -> Vec<AttributeDefinition> {
    vec![string_attribute(primary_key_name_a())]
}

pub fn empty_list_tables_output() -> ListTablesOutput {
    ListTablesOutput {
        last_evaluated_table_name: None,
        table_names: Some([].to_vec())
    }
}
