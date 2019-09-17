use crate::dht::bbdht::dynamodb::schema::hash_key;
use crate::dht::bbdht::dynamodb::schema::string_attribute_definition;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::KeySchemaElement;

pub const ADDRESS_KEY: &str = "address";
pub const CONTENT_KEY: &str = "content";

pub fn address_key_schema() -> KeySchemaElement {
    hash_key(ADDRESS_KEY)
}

pub fn content_key_schema() -> KeySchemaElement {
    hash_key(CONTENT_KEY)
}

pub fn key_schema_cas() -> Vec<KeySchemaElement> {
    vec![address_key_schema()]
}

pub fn address_attribute_definition() -> AttributeDefinition {
    string_attribute_definition(ADDRESS_KEY)
}

pub fn content_attribute_definition() -> AttributeDefinition {
    string_attribute_definition(CONTENT_KEY)
}

pub fn attribute_definitions_cas() -> Vec<AttributeDefinition> {
    vec![
        address_attribute_definition(),
        // content_attribute_definition(),
    ]
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::schema::cas::address_attribute_definition;
    use crate::dht::bbdht::dynamodb::schema::cas::address_key_schema;
    use crate::dht::bbdht::dynamodb::schema::cas::attribute_definitions_cas;
    use crate::dht::bbdht::dynamodb::schema::cas::content_attribute_definition;
    use crate::dht::bbdht::dynamodb::schema::cas::content_key_schema;
    use crate::dht::bbdht::dynamodb::schema::cas::key_schema_cas;
    use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::CONTENT_KEY;
    use crate::test::setup;
    use rusoto_dynamodb::AttributeDefinition;
    use rusoto_dynamodb::KeySchemaElement;

    #[test]
    fn address_key_schema_test() {
        setup();

        let address_key_schema = address_key_schema();
        assert_eq!(
            KeySchemaElement {
                attribute_name: ADDRESS_KEY.to_string(),
                key_type: "HASH".into(),
            },
            address_key_schema,
        );
    }

    #[test]
    fn content_key_schema_test() {
        setup();

        let content_key_schema = content_key_schema();
        assert_eq!(
            KeySchemaElement {
                attribute_name: CONTENT_KEY.to_string(),
                key_type: "HASH".into(),
            },
            content_key_schema,
        );
    }

    #[test]
    fn key_schema_cas_test() {
        setup();

        let key_schema_cas = key_schema_cas();
        assert_eq!(
            vec![KeySchemaElement {
                attribute_name: ADDRESS_KEY.to_string(),
                key_type: "HASH".into(),
            }],
            key_schema_cas
        );
    }

    #[test]
    fn address_attribute_definition_test() {
        setup();

        let address_attribute_definition = address_attribute_definition();
        assert_eq!(
            AttributeDefinition {
                attribute_name: ADDRESS_KEY.to_string(),
                attribute_type: "S".into(),
            },
            address_attribute_definition,
        );
    }

    #[test]
    fn content_attribute_definition_test() {
        setup();

        let content_attribute_definition = content_attribute_definition();
        assert_eq!(
            AttributeDefinition {
                attribute_name: CONTENT_KEY.to_string(),
                attribute_type: "S".into(),
            },
            content_attribute_definition,
        );
    }

    #[test]
    fn attribute_definitions_cas_test() {
        setup();

        let attribute_definitions_cas = attribute_definitions_cas();
        assert_eq!(address_attribute_definition(), attribute_definitions_cas[0],);
        // assert_eq!(
        //     content_attribute_definition(),
        //     attribute_definitions_cas[1],
        // );
    }

}
