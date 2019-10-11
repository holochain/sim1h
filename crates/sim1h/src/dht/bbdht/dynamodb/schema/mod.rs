pub mod cas;
pub mod fixture;

use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::AttributeValue;
use rusoto_dynamodb::KeySchemaElement;

#[derive(Debug, Default, Clone)]
pub struct TableName(String);

impl From<TableName> for String {
    fn from(table_name: TableName) -> Self {
        table_name.0
    }
}

impl From<String> for TableName {
    fn from(string: String) -> Self {
        TableName(string)
    }
}

impl From<&String> for TableName {
    fn from(string: &String) -> Self {
        (*string).clone().into()
    }
}

impl From<&TableName> for String {
    fn from(table_name: &TableName) -> Self {
        (*table_name).clone().into()
    }
}

pub fn hash_key(attribute_name: &str) -> KeySchemaElement {
    KeySchemaElement {
        attribute_name: attribute_name.into(),
        key_type: "HASH".into(),
    }
}

pub fn string_attribute_definition(attribute_name: &str) -> AttributeDefinition {
    AttributeDefinition {
        attribute_name: attribute_name.into(),
        attribute_type: "S".into(),
    }
}

pub fn string_attribute_value(value: &String) -> AttributeValue {
    AttributeValue {
        s: Some(value.to_owned()),
        ..Default::default()
    }
}

pub fn bool_attribute_value(value: bool) -> AttributeValue {
    AttributeValue {
        bool: Some(value),
        ..Default::default()
    }
}

pub fn blob_attribute_value(value: &Vec<u8>) -> AttributeValue {
    AttributeValue {
        b: Some(value.as_slice().into()),
        ..Default::default()
    }
}

pub fn number_attribute_value(value: &u64) -> AttributeValue {
    AttributeValue {
        n: Some(value.to_string()),
        ..Default::default()
    }
}

pub fn string_set_attribute_value(value: Vec<String>) -> AttributeValue {
    AttributeValue {
        ss: Some(value),
        ..Default::default()
    }
}

pub fn list_attribute_value(value: Vec<AttributeValue>) -> AttributeValue {
    AttributeValue {
        l: Some(value),
        ..Default::default()
    }
}

#[cfg(test)]
pub mod test {

    use crate::dht::bbdht::dynamodb::schema::fixture::attribute_name_fresh;
    use crate::dht::bbdht::dynamodb::schema::hash_key;
    use crate::dht::bbdht::dynamodb::schema::string_attribute_definition;
    use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
    use rusoto_dynamodb::AttributeDefinition;
    use rusoto_dynamodb::AttributeValue;
    use rusoto_dynamodb::KeySchemaElement;

    #[test]
    fn hash_key_test() {
        let attribute_name = attribute_name_fresh();

        let result = hash_key(&attribute_name);

        assert_eq!(
            KeySchemaElement {
                attribute_name: attribute_name.into(),
                key_type: String::from("HASH"),
            },
            result,
        );
    }

    #[test]
    fn string_attribute_definition_test() {
        let attribute_name = attribute_name_fresh();

        let result = string_attribute_definition(&attribute_name);

        assert_eq!(
            AttributeDefinition {
                attribute_name: attribute_name.into(),
                attribute_type: "S".into(),
            },
            result,
        );
    }

    #[test]
    fn string_attribute_value_test() {
        let value = String::from("foo");

        let result = string_attribute_value(&value);

        assert_eq!(
            AttributeValue {
                b: None,
                bool: None,
                bs: None,
                l: None,
                m: None,
                n: None,
                ns: None,
                null: None,
                s: Some(value.clone()),
                ss: None,
            },
            result,
        );
    }

}
