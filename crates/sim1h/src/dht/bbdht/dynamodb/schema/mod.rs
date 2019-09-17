pub mod fixture;

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
    use crate::dht::bbdht::dynamodb::schema::fixture::attribute_name_fresh;
    use crate::dht::bbdht::dynamodb::schema::string_attribute;
    use rusoto_dynamodb::AttributeDefinition;

    #[test]
    fn hash_key_test() {
        let attribute_name = attribute_name_fresh();

        let result = hash_key(&attribute_name);

        assert_eq!(
            KeySchemaElement {
                attribute_name: attribute_name.to_string(),
                key_type: String::from("HASH"),
            },
            result,
        );
    }

    #[test]
    fn string_attribute_test()  {
        let attribute_name = attribute_name_fresh();

        let result = string_attribute(&attribute_name);

        assert_eq!(
            AttributeDefinition {
                attribute_name: attribute_name.into(),
                attribute_type: "S".into(),
            },
            result,
        );
    }

}
