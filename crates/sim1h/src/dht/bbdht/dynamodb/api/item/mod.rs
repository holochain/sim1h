use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::cas::PARTITION_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ITEM_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::NETWORK_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::SPACE_KEY;
use crate::space::Space;

pub mod fixture;
pub mod read;
pub mod write;

pub type Item = HashMap<String, AttributeValue>;

pub struct ItemKey(String);

pub fn keyed_item(space: &Space, item_key: &ItemKey) -> Item {
    let mut item = HashMap::new();
    item.insert(
        String::from(PARTITION_KEY),
        string_attribute_value(
            format!(
                "{}:{}:{}",
                &space.network_id.to_string(),
                &space.space_address.to_string(),
                &item_key.to_string(),
            )
        ),
    );
    item.insert(
        String::from(ITEM_KEY),
        string_attribute_value(
                &item_key.to_string(),
        ),
    );
    item.insert(
        String::from(NETWORK_KEY),
        string_attribute_value(
            &space.network_id.to_string(),
        ),
    );
    item.insert(
        String::from(SPACE_KEY),
        string_attribute_value(
            &space.space_address.to_string(),
        ),
    );
    item
}
