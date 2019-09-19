
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_LIST_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::CONTENT_KEY;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::string_set_attribute_value;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;
use crate::dht::bbdht::dynamodb::schema::cas::inbox_key;
use holochain_persistence_api::cas::content::AddressableContent;
use lib3h_protocol::data_types::EntryAspectData;
use rusoto_core::RusotoError;
use rusoto_dynamodb::AttributeValue;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::PutItemError;
use rusoto_dynamodb::PutItemInput;
use rusoto_dynamodb::PutItemOutput;
use rusoto_dynamodb::UpdateItemError;
use rusoto_dynamodb::UpdateItemInput;
use rusoto_dynamodb::UpdateItemOutput;
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

pub fn put_aspect(log_context: &LogContext, client: &Client, table_name: &TableName, aspect: &EntryAspectData) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    tracer(&log_context, "put_aspect");

    let mut aspect_item = HashMap::new();
    aspect_item.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&aspect.aspect_address.to_string()),
    );

    match client.put_item(PutItemInput {
        table_name: table_name.to_string(),
        item: aspect_item,
        ..Default::default()
    }).sync() {
        Ok(v) => Ok(v),
        Err(e) => {
            // brute force failures
            tracer(&log_context, &format!("{:?}", e));
            put_aspect(&log_context, &client, &table_name, &aspect)
        }
    }
}

pub fn aspect_list_to_attribute(aspect_list: &Vec<EntryAspectData>) -> AttributeValue {
    string_set_attribute_value(
        aspect_list
            .iter()
            .map(|aspect| aspect.aspect_address.to_string())
            .collect(),
    )
}

pub fn append_aspects(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    entry_address: &Address,
    aspect_list: &Vec<EntryAspectData>,
) -> Result<UpdateItemOutput, RusotoError<UpdateItemError>> {
    tracer(&log_context, "append_aspects");

    // the aspect addressses live under the entry address
    let mut aspect_addresses_key = HashMap::new();
    aspect_addresses_key.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(entry_address.to_owned())),
    );

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":aspects".to_string(),
        aspect_list_to_attribute(&aspect_list),
    );

    let mut expression_attribute_names = HashMap::new();
    expression_attribute_names.insert("#aspect_list".to_string(), ASPECT_LIST_KEY.to_string());

    let update_expression = "ADD #aspect_list :aspects";

    let aspect_list_update = UpdateItemInput {
        table_name: table_name.to_string(),
        key: aspect_addresses_key,
        // https://stackoverflow.com/questions/31288085/how-to-append-a-value-to-list-attribute-on-aws-dynamodb
        update_expression: Some(update_expression.to_string()),
        expression_attribute_names: Some(expression_attribute_names),
        expression_attribute_values: Some(expression_attribute_values),
        ..Default::default()
    };

    client.update_item(aspect_list_update).sync()
}

pub fn append_agent_message(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    _request_id: &String,
    _from: &Address,
    to: &Address,
    _content: &Vec<u8>,
) -> Result<UpdateItemOutput, RusotoError<UpdateItemError>> {
    tracer(&log_context, "append_agent_message");

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

    client.update_item(inbox_update).sync()
}

#[cfg(test)]
pub mod tests {

    use crate::agent::fixture::agent_id_fresh;
    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::item::read::get_item_by_address;
    use crate::dht::bbdht::dynamodb::api::item::write::append_aspects;
    use crate::dht::bbdht::dynamodb::api::item::write::aspect_list_to_attribute;
    use crate::dht::bbdht::dynamodb::api::item::write::ensure_content;
    use crate::dht::bbdht::dynamodb::api::item::write::touch_agent;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::workflow::fixture::entry_aspect_data_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::api::item::write::put_aspect;
    use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_LIST_KEY;
    use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
    use crate::trace::tracer;
    use crate::workflow::fixture::aspect_list_fresh;
    use crate::workflow::fixture::entry_address_fresh;
    use std::collections::HashMap;

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
    fn put_aspect_test() {
        let log_context = "put_aspect_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let entry_aspect = entry_aspect_data_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).is_ok());

        // put aspect
        println!("{:#?}", put_aspect(&log_context, &local_client, &table_name, &entry_aspect).is_ok());
    }

    #[test]
    fn append_aspects_test() {
        let log_context = "append_aspects_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let entry_address = entry_address_fresh();
        let aspect_list = aspect_list_fresh();

        let mut expected = HashMap::new();
        expected.insert(
            ASPECT_LIST_KEY.to_string(),
            aspect_list_to_attribute(&aspect_list),
        );
        expected.insert(
            ADDRESS_KEY.to_string(),
            string_attribute_value(&String::from(entry_address.clone())),
        );

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).is_ok());

        // trash/idempotency loop
        for _ in 0..100 {
            // append aspects
            assert!(append_aspects(
                &log_context,
                &local_client,
                &table_name,
                &entry_address,
                &aspect_list
            )
            .is_ok());

            // get matches
            match get_item_by_address(&log_context, &local_client, &table_name, &entry_address) {
                Ok(get_item_output) => match get_item_output.item {
                    Some(item) => {
                        assert_eq!(expected["address"], item["address"],);
                        assert_eq!(
                            expected["aspect_list"].ss.iter().count(),
                            item["aspect_list"].ss.iter().count(),
                        );
                    }
                    None => {
                        tracer(&log_context, "get matches None");
                        panic!("None");
                    }
                },
                Err(err) => {
                    tracer(&log_context, "get matches err");
                    panic!("{:?}", err);
                }
            }
        }
    }

}
