use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::blob_attribute_value;
use crate::dht::bbdht::dynamodb::schema::cas::inbox_key;
use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::MESSAGE_CONTENT_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::MESSAGE_FROM_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::MESSAGE_SPACE_ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::MESSAGE_TO_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::REQUEST_IDS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::REQUEST_IDS_SEEN_KEY;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::string_set_attribute_value;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::dht::bbdht::error::BbDhtError;
use crate::dht::bbdht::error::BbDhtResult;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::DirectMessageData;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::GetItemInput;
use rusoto_dynamodb::PutItemError;
use rusoto_dynamodb::PutItemInput;
use rusoto_dynamodb::UpdateItemInput;
use std::collections::HashMap;

/// put an item that can be reconstructed to DirectMessageData against the request id
pub fn put_inbox_message(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    request_id: &String,
    from: &Address,
    to: &Address,
    content: &Vec<u8>,
) -> BbDhtResult<()> {
    tracer(&log_context, "put_inbox_message");

    let mut message_item = HashMap::new();

    message_item.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(request_id),
    );

    message_item.insert(
        String::from(MESSAGE_FROM_KEY),
        string_attribute_value(&from.to_string()),
    );

    message_item.insert(
        String::from(MESSAGE_TO_KEY),
        string_attribute_value(&to.to_string()),
    );

    message_item.insert(
        String::from(MESSAGE_SPACE_ADDRESS_KEY),
        string_attribute_value(&table_name.to_string()),
    );

    message_item.insert(
        String::from(MESSAGE_CONTENT_KEY),
        blob_attribute_value(&content),
    );

    match client
        .put_item(PutItemInput {
            table_name: table_name.to_string(),
            item: message_item,
            ..Default::default()
        })
        .sync()
    {
        Ok(_) => Ok(()),
        // brute force retryable failures
        // TODO do not brute force failures
        // use transactions upstream instead
        Err(RusotoError::Service(err)) => match err {
            PutItemError::InternalServerError(err) => {
                tracer(
                    &log_context,
                    &format!(
                        "retry put_inbox_message Service InternalServerError {:?}",
                        err
                    ),
                );
                put_inbox_message(
                    log_context,
                    client,
                    table_name,
                    request_id,
                    from,
                    to,
                    content,
                )
            }
            PutItemError::ProvisionedThroughputExceeded(err) => {
                tracer(
                    &log_context,
                    &format!(
                        "retry put_inbox_message Service ProvisionedThroughputExceeded {:?}",
                        err
                    ),
                );
                put_inbox_message(
                    log_context,
                    client,
                    table_name,
                    request_id,
                    from,
                    to,
                    content,
                )
            }
            PutItemError::RequestLimitExceeded(err) => {
                tracer(
                    &log_context,
                    &format!(
                        "retry put_inbox_message Service RequestLimitExceeded {:?}",
                        err
                    ),
                );
                put_inbox_message(
                    log_context,
                    client,
                    table_name,
                    request_id,
                    from,
                    to,
                    content,
                )
            }
            PutItemError::TransactionConflict(err) => {
                tracer(
                    &log_context,
                    &format!(
                        "retry put_inbox_message Service TransactionConflict {:?}",
                        err
                    ),
                );
                put_inbox_message(
                    log_context,
                    client,
                    table_name,
                    request_id,
                    from,
                    to,
                    content,
                )
            }
            _ => Err(err.into()),
        },
        Err(RusotoError::Unknown(err)) => {
            tracer(
                &log_context,
                &format!("retry put_inbox_message Unknown {:?}", err),
            );
            put_inbox_message(
                log_context,
                client,
                table_name,
                request_id,
                from,
                to,
                content,
            )
        }
        Err(err) => Err(err.into()),
    }
}

pub fn append_request_id_to_inbox(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    folder: &String,
    request_id: &String,
    to: &Address,
) -> BbDhtResult<()> {
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
    inbox_attribute_names.insert("#request_ids".to_string(), folder.to_string());

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
    from: &Address,
    to: &Address,
    content: &Vec<u8>,
) -> BbDhtResult<()> {
    tracer(&log_context, "send_to_agent_inbox");

    put_inbox_message(
        log_context,
        client,
        table_name,
        request_id,
        from,
        to,
        content,
    )?;

    append_request_id_to_inbox(
        log_context,
        client,
        table_name,
        &REQUEST_IDS_KEY.to_string(),
        request_id,
        to,
    )?;

    Ok(())
}

pub fn get_inbox_request_ids(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    inbox_folder: &String,
    to: &Address,
) -> BbDhtResult<Vec<String>> {
    tracer(log_context, "get_inbox_request_ids");

    let mut key = HashMap::new();
    key.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&inbox_key(to)),
    );
    let get_item_output = client
        .get_item(GetItemInput {
            table_name: table_name.into(),
            key: key,
            ..Default::default()
        })
        .sync()?
        .item;
    Ok(match get_item_output {
        Some(item) => match item.get(inbox_folder) {
            Some(attribute) => match attribute.ss.clone() {
                Some(ss) => ss,
                None => Vec::new(),
            },
            None => Vec::new(),
        },
        None => Vec::new(),
    })
}

pub fn item_to_direct_message_data(item: &Item) -> BbDhtResult<DirectMessageData> {
    let content = match item[MESSAGE_CONTENT_KEY].b.clone() {
        Some(v) => v.to_vec(),
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing content {:?}",
                &item
            )))
        }
    };

    let from_agent_id = match item[MESSAGE_FROM_KEY].s.clone() {
        Some(v) => v,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing from {:?}",
                &item
            )))
        }
    };

    let to_agent_id = match item[MESSAGE_TO_KEY].s.clone() {
        Some(v) => v,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing to {:?}",
                &item
            )))
        }
    };

    let space_address = match item[MESSAGE_SPACE_ADDRESS_KEY].s.clone() {
        Some(v) => v,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing space_address {:?}",
                &item
            )))
        }
    };

    let request_id = match item[ADDRESS_KEY].s.clone() {
        Some(v) => v,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing request_id {:?}",
                &item
            )))
        }
    };

    Ok(DirectMessageData {
        content: content.into(),
        from_agent_id: from_agent_id.into(),
        to_agent_id: to_agent_id.into(),
        request_id: request_id,
        space_address: space_address.into(),
    })
}

pub fn request_ids_to_messages(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    request_ids: &Vec<String>,
) -> BbDhtResult<Vec<DirectMessageData>> {
    tracer(log_context, "request_ids_to_messages");

    let mut direct_message_datas = Vec::new();

    for request_id in request_ids {
        let mut key = HashMap::new();
        key.insert(
            String::from(ADDRESS_KEY),
            string_attribute_value(&request_id),
        );

        let get_item_output = client
            .get_item(GetItemInput {
                table_name: table_name.into(),
                key: key,
                ..Default::default()
            })
            .sync()?
            .item;

        match get_item_output {
            Some(item) => {
                direct_message_datas.push(item_to_direct_message_data(&item)?);
            }
            // the request ids MUST be in the db
            None => {
                return Err(BbDhtError::MissingData(format!(
                    "missing message for request id: {:?}",
                    &request_id
                )))
            }
        }
    }

    Ok(direct_message_datas)
}

pub fn check_inbox(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    to: &Address,
) -> BbDhtResult<Vec<DirectMessageData>> {
    tracer(&log_context, "check_inbox");

    let inbox_request_ids = get_inbox_request_ids(
        log_context,
        client,
        table_name,
        &REQUEST_IDS_KEY.to_string(),
        to,
    )?;
    let seen_request_ids = get_inbox_request_ids(
        log_context,
        client,
        table_name,
        &REQUEST_IDS_SEEN_KEY.to_string(),
        to,
    )?;

    let unseen_request_ids: Vec<String> = inbox_request_ids
        .iter()
        .filter(|request_id| !seen_request_ids.contains(request_id))
        .cloned()
        .collect();

    let messages = request_ids_to_messages(log_context, client, table_name, &unseen_request_ids);

    // record that we have now seen the unseen without errors (so far)
    for unseen in unseen_request_ids.clone() {
        append_request_id_to_inbox(
            log_context,
            client,
            table_name,
            &REQUEST_IDS_SEEN_KEY.to_string(),
            &unseen,
            &to,
        )?;
    }

    messages
}

#[cfg(test)]
pub mod tests {

    use crate::agent::fixture::agent_id_fresh;
    use crate::agent::fixture::message_content_fresh;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::append_request_id_to_inbox;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::check_inbox;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::get_inbox_request_ids;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::put_inbox_message;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::send_to_agent_inbox;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::schema::cas::REQUEST_IDS_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::REQUEST_IDS_SEEN_KEY;
    use crate::network::fixture::request_id_fresh;
    use crate::trace::tracer;
    use lib3h_protocol::data_types::DirectMessageData;

    fn folders() -> Vec<String> {
        vec![
            REQUEST_IDS_KEY.to_string(),
            REQUEST_IDS_SEEN_KEY.to_string(),
        ]
    }

    #[test]
    fn append_request_id_to_inbox_test() {
        let log_context = "append_request_id_to_inbox_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let request_id = request_id_fresh();
        let to = agent_id_fresh();

        for folder in folders() {
            // ensure cas
            assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

            // append request_id
            assert!(append_request_id_to_inbox(
                &log_context,
                &local_client,
                &table_name,
                &folder,
                &request_id,
                &to
            )
            .is_ok());
        }
    }

    #[test]
    fn put_inbox_message_test() {
        let log_context = "put_inbox_message_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let request_id = request_id_fresh();
        let from = agent_id_fresh();
        let to = agent_id_fresh();
        let content = message_content_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // pub inbox message
        assert!(put_inbox_message(
            &log_context,
            &local_client,
            &table_name,
            &request_id,
            &from,
            &to,
            &content
        )
        .is_ok());
    }

    #[test]
    fn send_to_agent_inbox_test() {
        let log_context = "send_to_agent_inbox_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let request_id = request_id_fresh();
        let from = agent_id_fresh();
        let to = agent_id_fresh();
        let content = message_content_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // pub inbox message
        assert!(send_to_agent_inbox(
            &log_context,
            &local_client,
            &table_name,
            &request_id,
            &from,
            &to,
            &content
        )
        .is_ok());
    }

    #[test]
    fn get_inbox_request_ids_test() {
        let log_context = "get_inbox_request_ids_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let request_id = request_id_fresh();
        let from = agent_id_fresh();
        let to = agent_id_fresh();
        let content = message_content_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // pub inbox message
        assert!(send_to_agent_inbox(
            &log_context,
            &local_client,
            &table_name,
            &request_id.clone(),
            &from,
            &to,
            &content
        )
        .is_ok());

        // get inbox message
        match get_inbox_request_ids(
            &log_context,
            &local_client,
            &table_name,
            &REQUEST_IDS_KEY.to_string(),
            &to,
        ) {
            Ok(request_ids) => assert_eq!(vec![request_id.clone()], request_ids),
            Err(err) => panic!("incorrect request id {:?}", err),
        };
    }

    #[test]
    fn check_inbox_test() {
        let log_context = "get_inbox_request_ids_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let request_id = request_id_fresh();
        let from = agent_id_fresh();
        let to = agent_id_fresh();
        let content = message_content_fresh();

        let direct_message_data = DirectMessageData {
            content: content.clone().into(),
            from_agent_id: from.clone(),
            to_agent_id: to.clone(),
            request_id: request_id.clone(),
            space_address: table_name.clone().into(),
        };

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // pub inbox message
        assert!(send_to_agent_inbox(
            &log_context,
            &local_client,
            &table_name,
            &request_id.clone(),
            &from,
            &to,
            &content
        )
        .is_ok());

        // check inbox
        match check_inbox(&log_context, &local_client, &table_name, &to) {
            Ok(messages) => assert_eq!(vec![direct_message_data.clone()], messages),
            Err(err) => panic!("incorrect request id {:?}", err),
        };

        // check again, should be empty
        match check_inbox(&log_context, &local_client, &table_name, &to) {
            Ok(request_ids) => {
                let v: Vec<DirectMessageData> = Vec::new();
                assert_eq!(v, request_ids);
            }
            Err(err) => panic!("incorrect request id {:?}", err),
        };
    }
}
