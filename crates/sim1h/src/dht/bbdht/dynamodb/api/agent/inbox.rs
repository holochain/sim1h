use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::cas::inbox_key;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::dht::bbdht::error::BbDhtResult;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::UpdateItemInput;
use std::collections::HashMap;
use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::REQUEST_IDS_KEY;
use crate::dht::bbdht::dynamodb::schema::string_set_attribute_value;

pub fn append_request_id_to_inbox(log_context: &LogContext, client: &Client, table_name: &TableName, to: &Address, request_id: &String) -> BbDhtResult<()> {
    tracer(&log_context, "append_request_id_to_inbox");

    let mut inbox_address_key = HashMap::new();

    // primary key is the inbox name "inbox_<agent_id>"
    inbox_address_key.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&inbox_key(to)),
    );

    // the request id appended under the inbox address
    let mut inbox_attribute_values = HashMap::new();
    inbox_attribute_values.insert(
        ":request_ids".to_string(),
        string_set_attribute_value(vec![request_id.to_string()]),
    );

    let mut inbox_attribute_names = HashMap::new();
    inbox_attribute_names.insert("#request_ids".to_string(), REQUEST_IDS_KEY.to_string());

    // https://stackoverflow.com/questions/31288085/how-to-append-a-value-to-list-attribute-on-aws-dynamodb
    let update_expression = "ADD #request_ids :request_ids";

    let request_ids_update = UpdateItemInput {
        table_name: table_name.to_string(),
        key: inbox_address_key,
        update_expression: Some(update_expression.to_string()),
        expression_attribute_names: Some(inbox_attribute_names),
        expression_attribute_values: Some(inbox_attribute_values),
        ..Default::default()
    };

    client.update_item(request_ids_update).sync()?;
    Ok(())
}

pub fn send_to_agent_inbox(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    request_id: &String,
    _from: &Address,
    to: &Address,
    _content: &Vec<u8>,
) -> BbDhtResult<()> {
    tracer(&log_context, "send_to_agent_inbox");

    append_request_id_to_inbox(log_context, client, table_name, to, request_id)?;


    // the recipient is the key address


    // inbox_address_key.insert

    // TODO
    // let inbox_update = UpdateItemInput {
    //     table_name: table_name.to_string(),
    //     key: inbox_address_key,
    //     // update_expression: Some("".to_string()),
    //     ..Default::default()
    // };

    // client.update_item(inbox_update).sync()?;
    Ok(())
}

#[cfg(test)]
pub mod tests {

    use crate::trace::tracer;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::network::fixture::request_id_fresh;
    use crate::agent::fixture::agent_id_fresh;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::append_request_id_to_inbox;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;

    #[test]
    fn append_request_id_to_inbox_test() {
        let log_context = "append_request_id_to_inbox_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let request_id = request_id_fresh();
        let to = agent_id_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // append request_id
        assert!(append_request_id_to_inbox(&log_context, &local_client, &table_name, &to, &request_id).is_ok());
    }

}
