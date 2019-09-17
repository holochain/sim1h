use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::KeySchemaElement;

pub fn hash_key(attribute_name: &str) -> KeySchemaElement {
    KeySchemaElement {
        attribute_name: attribute_name.into(),
        key_type: "HASH".into(),
    }
}

pub fn string_attribute(attribute_name: &str) -> AttributeDefinition {
    AttributeDefinition {
        attribute_name: attribute_name.into(),
        attribute_type: "S".into(),
    }
}

#[cfg(test)]
pub mod test {

    use crate::dht::bbdht::dynamodb::schema::hash_key;
    use rusoto_dynamodb::KeySchemaElement;

    #[test]
    fn hash_key_test() {
        let attribute_name = "foo";

        let result = hash_key(attribute_name);

        assert_eq!(
            KeySchemaElement {
                attribute_name: attribute_name.to_string(),
                key_type: String::from("HASH"),
            },
            result,
        );
    }

}
