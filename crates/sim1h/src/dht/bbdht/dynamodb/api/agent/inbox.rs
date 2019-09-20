use std::collections::HashMap;
use crate::trace::LogContext;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use holochain_persistence_api::cas::content::Address;
use crate::dht::bbdht::error::BbDhtResult;
use crate::trace::tracer;
use crate::dht::bbdht::dynamodb::schema::cas::inbox_key;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use rusoto_dynamodb::UpdateItemInput;
use rusoto_dynamodb::DynamoDb;

pub fn send_to_agent_inbox(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    _request_id: &String,
    _from: &Address,
    to: &Address,
    _content: &Vec<u8>,
) -> BbDhtResult<()> {
    tracer(&log_context, "send_to_agent_inbox");

    // the recipient is the key address
    let mut inbox_address_key = HashMap::new();
    inbox_address_key.insert(
        String::from(inbox_key(to)),
        string_attribute_value(&String::from(to.to_owned())),
    );

    // inbox_address_key.insert

    // TODO
    let inbox_update = UpdateItemInput {
        table_name: table_name.to_string(),
        key: inbox_address_key,
        // update_expression: Some("".to_string()),
        ..Default::default()
    };

    client.update_item(inbox_update).sync()?;
    Ok(())
}
