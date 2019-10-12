use crate::dht::bbdht::dynamodb::schema::hash_key;
use crate::dht::bbdht::dynamodb::schema::string_attribute_definition;
use holochain_persistence_api::cas::content::Address;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::KeySchemaElement;

// dynamodb needs a single partition/primary key
pub const PARTITION_KEY: &str = "partition_key";
pub const SPACE_KEY: &str = "space";
pub const NETWORK_KEY: &str = "network";
pub const ITEM_KEY: &str = "item";

// addressable content
pub const CONTENT_KEY: &str = "content";

// entry aspects
pub const ASPECT_LIST_KEY: &str = "aspect_list";
pub const ASPECT_ADDRESS_KEY: &str = "aspect_address";
pub const ASPECT_TYPE_HINT_KEY: &str = "aspect_type_hint";
pub const ASPECT_KEY: &str = "aspect";
pub const ASPECT_PUBLISH_TS_KEY: &str = "aspect_publish_ts";

// direct messaging keys
pub const INBOX_KEY_PREFIX: &str = "inbox_";
pub const ALL_MESSAGES_FOLDER: &str = "all_messages";
pub const SEEN_MESSAGES_FOLDER: &str = "seen_messages";
pub const MESSAGE_FROM_KEY: &str = "message_from";
pub const MESSAGE_TO_KEY: &str = "message_to";
pub const MESSAGE_CONTENT_KEY: &str = "message_content";
pub const MESSAGE_IS_RESPONSE_KEY: &str = "message_is_response";

pub fn inbox_key(agent_id: &Address) -> String {
    format!("{}{}", INBOX_KEY_PREFIX, agent_id)
}

pub fn partition_key_schema() -> KeySchemaElement {
    hash_key(PARTITION_KEY)
}

pub fn key_schema_cas() -> Vec<KeySchemaElement> {
    vec![partition_key_schema()]
}

pub fn partition_key_attribute_definition() -> AttributeDefinition {
    string_attribute_definition(PARTITION_KEY)
}

pub fn attribute_definitions_cas() -> Vec<AttributeDefinition> {
    vec![partition_key_attribute_definition()]
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::schema::cas::attribute_definitions_cas;
    use crate::dht::bbdht::dynamodb::schema::cas::key_schema_cas;
    use crate::dht::bbdht::dynamodb::schema::cas::partition_key_attribute_definition;
    use crate::dht::bbdht::dynamodb::schema::cas::partition_key_schema;
    use crate::dht::bbdht::dynamodb::schema::cas::ITEM_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::PARTITION_KEY;
    use crate::trace::tracer;
    use rusoto_dynamodb::AttributeDefinition;
    use rusoto_dynamodb::KeySchemaElement;

    #[test]
    fn partition_key_schema_test() {
        let log_context = "partition_key_schema_test";

        tracer(&log_context, "compare values");
        assert_eq!(
            KeySchemaElement {
                attribute_name: PARTITION_KEY.into(),
                key_type: "HASH".into(),
            },
            partition_key_schema(),
        );
    }

    #[test]
    fn key_schema_cas_test() {
        let log_context = "key_schema_cas_test";

        tracer(&log_context, "compare values");
        assert_eq!(
            vec![KeySchemaElement {
                attribute_name: PARTITION_KEY.into(),
                key_type: "HASH".into(),
            }],
            key_schema_cas()
        );
    }

    #[test]
    fn partition_key_attribute_definition_test() {
        let log_context = "address_attribute_definition_test";

        tracer(&log_context, "compare values");
        assert_eq!(
            AttributeDefinition {
                attribute_name: ITEM_KEY.into(),
                attribute_type: "S".into(),
            },
            partition_key_attribute_definition(),
        );
    }

    #[test]
    fn attribute_definitions_cas_test() {
        let log_context = "attribute_definitions_cas_test";

        tracer(&log_context, "compare values");
        assert_eq!(
            partition_key_attribute_definition(),
            attribute_definitions_cas()[0]
        );
    }
}
