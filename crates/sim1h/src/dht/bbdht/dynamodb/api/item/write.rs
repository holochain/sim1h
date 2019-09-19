use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_LIST;
use crate::dht::bbdht::dynamodb::schema::cas::CONTENT_KEY;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;
use holochain_persistence_api::cas::content::AddressableContent;
use crate::dht::bbdht::dynamodb::schema::string_set_attribute_value;
use lib3h_protocol::data_types::EntryAspectData;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::PutItemError;
use rusoto_dynamodb::PutItemInput;
use rusoto_dynamodb::PutItemOutput;
use rusoto_dynamodb::TransactWriteItem;
use rusoto_dynamodb::TransactWriteItemsError;
use rusoto_dynamodb::TransactWriteItemsInput;
use rusoto_dynamodb::TransactWriteItemsOutput;
use rusoto_dynamodb::Update;
use std::collections::HashMap;

pub fn ensure_content(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    content: &dyn AddressableContent,
) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    tracer(&log_context, "ensure_content");
    let mut item = HashMap::new();
    item.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(content.address())),
    );
    item.insert(
        String::from(CONTENT_KEY),
        string_attribute_value(&String::from(content.content())),
    );

    client
        .put_item(PutItemInput {
            item: item,
            table_name: table_name.to_string(),
            ..Default::default()
        })
        .sync()
}

pub fn touch_agent(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    agent_id: &Address,
) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    tracer(&log_context, "touch_agent");

    let mut item = HashMap::new();
    item.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(agent_id.to_owned())),
    );
    client
        .put_item(PutItemInput {
            item: item,
            table_name: table_name.to_string(),
            ..Default::default()
        })
        .sync()
}

pub fn append_aspects(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    entry_address: &Address,
    aspect_list: &Vec<EntryAspectData>,
) -> Result<TransactWriteItemsOutput, RusotoError<TransactWriteItemsError>> {
    tracer(&log_context, "append_aspects");

    let mut transact_items = Vec::new();

    // the aspect addressses live under the entry address
    let mut aspect_addresses_key = HashMap::new();
    aspect_addresses_key.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(entry_address.to_owned())),
    );
    let aspect_addresses_attributes = aspect_list
        .iter()
        .map(|aspect| string_attribute_value(&aspect.aspect_address.to_string()))
        .collect();

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":s".to_string(),
        string_set_attribute_value(aspect_addresses_attributes),
    );

    tracer(&log_context, &format!("{:?}", &expression_attribute_values));

    transact_items.push(TransactWriteItem {
        update: Some(Update {
            table_name: table_name.to_string(),
            key: aspect_addresses_key,
            // https://stackoverflow.com/questions/31288085/how-to-append-a-value-to-list-attribute-on-aws-dynamodb
            update_expression: format!("SET {0} = list_append({0}, :s)", ASPECT_LIST),
            expression_attribute_values: Some(expression_attribute_values),
            ..Default::default()
        }),
        ..Default::default()
    });

    client
        .transact_write_items(TransactWriteItemsInput {
            transact_items: transact_items,
            ..Default::default()
        })
        .sync()
}

#[cfg(test)]
pub mod tests {

    use crate::agent::fixture::agent_id_fresh;
    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::item::write::ensure_content;
    use crate::dht::bbdht::dynamodb::api::item::write::append_aspects;
    use crate::dht::bbdht::dynamodb::api::item::write::touch_agent;
    use crate::workflow::fixture::aspect_list_fresh;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::workflow::fixture::entry_address_fresh;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;

    #[test]
    fn ensure_content_test() {
        let log_context = "ensure_content_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let content = content_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name)
            .expect("could not check table exists"));

        // ensure content
        assert!(ensure_content(&log_context, &local_client, &table_name, &content).is_ok());

        // thrash a bit
        for _ in 0..100 {
            assert!(ensure_content(&log_context, &local_client, &table_name, &content).is_ok());
        }
    }

    #[test]
    fn touch_agent_test() {
        let log_context = "touch_agent_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let agent_id = agent_id_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).is_ok());

        // touch agent
        assert!(touch_agent(&log_context, &local_client, &table_name, &agent_id).is_ok());
    }

    #[test]
    fn append_aspects_test() {
        let log_context = "append_aspects_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let entry_address = entry_address_fresh();
        let aspect_list = aspect_list_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).is_ok());

        // append aspects
        println!("{:?}", append_aspects(&log_context, &local_client, &table_name, &entry_address, &aspect_list));
    }

}
