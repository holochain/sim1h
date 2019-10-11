use crate::dht::bbdht::dynamodb::api::item::keyed_item;
use crate::dht::bbdht::dynamodb::api::item::read::get_item_from_space;
use crate::dht::bbdht::dynamodb::api::item::write::should_put_item_retry;
use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::dht::bbdht::dynamodb::schema::cas::ALL_MESSAGES_FOLDER;
use crate::dht::bbdht::dynamodb::schema::cas::ITEM_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::MESSAGE_CONTENT_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::MESSAGE_FROM_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::MESSAGE_TO_KEY;
use crate::agent::AgentAddress;
use crate::dht::bbdht::dynamodb::schema::cas::SEEN_MESSAGES_FOLDER;
use crate::dht::bbdht::dynamodb::schema::cas::SPACE_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::{inbox_key, MESSAGE_IS_RESPONSE_KEY};
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::string_set_attribute_value;
use crate::dht::bbdht::dynamodb::schema::{blob_attribute_value, bool_attribute_value};
use crate::dht::bbdht::error::BbDhtError;
use crate::dht::bbdht::error::BbDhtResult;
use crate::network::RequestId;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::DirectMessageData;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::PutItemInput;
use rusoto_dynamodb::UpdateItemInput;
use std::collections::HashMap;

pub struct FromAddress(Address);
pub struct ToAddress(Address);
pub struct Folder(String);

impl From<FromAddress> for String {
    fn from(from_address: FromAddress) -> Self {
        from_address.0.into()
    }
}

impl From<&FromAddress> for String {
    fn from(from_address: &FromAddress) -> Self {
        from_address.to_owned().into()
    }
}

impl From<Address> for FromAddress {
    fn from(address: Address) -> Self {
        FromAddress(address)
    }
}

impl From<Address> for ToAddress {
    fn from(address: Address) -> Self {
        ToAddress(address)
    }
}

impl From<AgentAddress> for ToAddress {
    fn from(agent_address: AgentAddress) -> Self {
        ToAddress(agent_address.into())
    }
}

impl From<AgentAddress> for FromAddress {
    fn from(agent_address: AgentAddress) -> Self {
        FromAddress(agent_address.into())
    }
}

impl From<&AgentAddress> for ToAddress {
    fn from(agent_address: &AgentAddress) -> Self {
        agent_address.to_owned().into()
    }
}

impl From<&AgentAddress> for FromAddress {
    fn from(agent_address: &AgentAddress) -> Self {
        agent_address.to_owned().into()
    }
}

impl From<ToAddress> for String {
    fn from(to_address: ToAddress) -> Self {
        to_address.0.into()
    }
}

impl From<&ToAddress> for String {
    fn from(to_address: &ToAddress) -> Self {
        to_address.to_owned().into()
    }
}

impl From<ToAddress> for Address {
    fn from(to_address: ToAddress) -> Self {
        to_address.0
    }
}

impl From<&ToAddress> for Address {
    fn from(to_address: &ToAddress) -> Self {
        to_address.to_owned().into()
    }
}

impl From<Folder> for String {
    fn from(folder: Folder) -> Self {
        folder.0
    }
}

impl ToString for Folder {
    fn to_string(&self) -> String {
        self.into()
    }
}

impl From<&str> for Folder {
    fn from(str: &str) -> Self {
        str.to_string().into()
    }
}

impl From<String> for Folder {
    fn from(string: String) -> Self {
        Folder(string)
    }
}

impl From<&Folder> for String {
    fn from(folder: &Folder) -> Self {
        folder.to_owned().into()
    }
}

/// put an item that can be reconstructed to DirectMessageData against the request id
pub fn put_inbox_message(
    log_context: &LogContext,
    space: &Space,
    request_id: &RequestId,
    from: &FromAddress,
    to: &ToAddress,
    content: &Vec<u8>,
    response: bool,
) -> BbDhtResult<()> {
    tracer(&log_context, "put_inbox_message");

    let mut item = keyed_item(space, &request_id.into());

    item.insert(
        String::from(MESSAGE_FROM_KEY),
        string_attribute_value(&from.into()),
    );

    item.insert(
        String::from(MESSAGE_TO_KEY),
        string_attribute_value(&to.into()),
    );

    item.insert(
        String::from(MESSAGE_CONTENT_KEY),
        blob_attribute_value(&content),
    );

    item.insert(
        String::from(MESSAGE_IS_RESPONSE_KEY),
        bool_attribute_value(response),
    );

    if should_put_item_retry(
        log_context,
        space
            .connection()
            .client()
            .put_item(PutItemInput {
                table_name: space.connection().table_name().into(),
                item: item,
                ..Default::default()
            })
            .sync(),
    )? {
        put_inbox_message(log_context, space, request_id, from, to, content, response)
    } else {
        Ok(())
    }
}

pub fn append_request_id_to_inbox(
    log_context: &LogContext,
    space: &Space,
    folder: &Folder,
    request_id: &RequestId,
    to: &ToAddress,
) -> BbDhtResult<()> {
    tracer(&log_context, "append_request_id_to_inbox");

    let inbox_address_key = keyed_item(space, &inbox_key(&to.into()).into());

    // the request id appended under the inbox address
    let mut inbox_attribute_values = HashMap::new();
    inbox_attribute_values.insert(
        ":request_ids".to_string(),
        string_set_attribute_value(vec![request_id.into()]),
    );

    let mut inbox_attribute_names = HashMap::new();
    inbox_attribute_names.insert("#request_ids".to_string(), folder.into());

    // https://stackoverflow.com/questions/31288085/how-to-append-a-value-to-list-attribute-on-aws-dynamodb
    let update_expression = "ADD #request_ids :request_ids";

    let request_ids_update = UpdateItemInput {
        table_name: space.connection().table_name().into(),
        key: inbox_address_key,
        update_expression: Some(update_expression.to_string()),
        expression_attribute_names: Some(inbox_attribute_names),
        expression_attribute_values: Some(inbox_attribute_values),
        ..Default::default()
    };

    space
        .connection()
        .client()
        .update_item(request_ids_update)
        .sync()?;
    Ok(())
}

pub fn send_to_agent_inbox(
    log_context: &LogContext,
    space: &Space,
    request_id: &RequestId,
    from: &FromAddress,
    to: &ToAddress,
    content: &Vec<u8>,
    response: bool,
) -> BbDhtResult<()> {
    tracer(&log_context, "send_to_agent_inbox");

    put_inbox_message(log_context, space, request_id, from, to, content, response)?;

    append_request_id_to_inbox(
        log_context,
        space,
        &ALL_MESSAGES_FOLDER.into(),
        request_id,
        to,
    )?;

    Ok(())
}

pub fn get_inbox_request_ids(
    log_context: &LogContext,
    space: &Space,
    inbox_folder: &Folder,
    to: &ToAddress,
) -> BbDhtResult<Vec<RequestId>> {
    tracer(log_context, "get_inbox_request_ids");

    Ok(
        match get_item_from_space(log_context, space, &inbox_key(&to.into()).into())? {
            Some(item) => match item.get(&inbox_folder.to_string()) {
                Some(attribute) => match attribute.ss.clone() {
                    Some(ss) => ss.iter().map(|s| s.into()).collect(),
                    None => Vec::new(),
                },
                None => Vec::new(),
            },
            None => Vec::new(),
        },
    )
}

pub fn item_to_direct_message_data(item: &Item) -> BbDhtResult<(DirectMessageData, bool)> {
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

    let space_address = match item[SPACE_KEY].s.clone() {
        Some(v) => v,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing space_address {:?}",
                &item
            )))
        }
    };

    let request_id = match item[ITEM_KEY].s.clone() {
        Some(v) => v,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing request_id {:?}",
                &item
            )))
        }
    };

    let is_response = match item[MESSAGE_IS_RESPONSE_KEY].bool.clone() {
        Some(v) => v,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "message item missing response flag {:?}",
                &item
            )))
        }
    };

    Ok((
        DirectMessageData {
            content: content.into(),
            from_agent_id: from_agent_id.into(),
            to_agent_id: to_agent_id.into(),
            request_id: request_id,
            space_address: space_address.into(),
        },
        is_response,
    ))
}

pub fn request_ids_to_messages(
    log_context: &LogContext,
    space: &Space,
    request_ids: &Vec<RequestId>,
) -> BbDhtResult<Vec<(DirectMessageData, bool)>> {
    tracer(log_context, "request_ids_to_messages");

    let mut direct_message_datas = Vec::new();

    for request_id in request_ids {
        match get_item_from_space(log_context, space, &request_id.into())? {
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
    space: &Space,
    to: &ToAddress,
) -> BbDhtResult<Vec<(DirectMessageData, bool)>> {
    tracer(&log_context, "check_inbox");

    let inbox_request_ids =
        get_inbox_request_ids(log_context, space, &ALL_MESSAGES_FOLDER.into(), to)?;
    let seen_request_ids =
        get_inbox_request_ids(log_context, space, &SEEN_MESSAGES_FOLDER.into(), to)?;

    let unseen_request_ids: Vec<RequestId> = inbox_request_ids
        .iter()
        .filter(|request_id| !seen_request_ids.contains(request_id))
        .cloned()
        .collect();

    let messages = request_ids_to_messages(log_context, space, &unseen_request_ids);

    // record that we have now seen the unseen without errors (so far)
    for unseen in unseen_request_ids.clone() {
        append_request_id_to_inbox(
            log_context,
            space,
            &SEEN_MESSAGES_FOLDER.into(),
            &unseen,
            &to,
        )?;
    }

    messages
}

#[cfg(test)]
pub mod tests {

    use crate::agent::fixture::agent_address_fresh;
    use crate::agent::fixture::message_content_fresh;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::append_request_id_to_inbox;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::check_inbox;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::get_inbox_request_ids;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::put_inbox_message;
    use crate::dht::bbdht::dynamodb::api::agent::inbox::send_to_agent_inbox;
    use crate::dht::bbdht::dynamodb::schema::cas::ALL_MESSAGES_FOLDER;
    use crate::dht::bbdht::dynamodb::schema::cas::SEEN_MESSAGES_FOLDER;
    use crate::network::fixture::request_id_fresh;
    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;
    use super::ToAddress;
    use super::FromAddress;
    use lib3h_protocol::data_types::DirectMessageData;

    fn folders() -> Vec<String> {
        vec![
            ALL_MESSAGES_FOLDER.to_string(),
            SEEN_MESSAGES_FOLDER.to_string(),
        ]
    }

    #[test]
    fn append_request_id_to_inbox_test() {
        let log_context = "append_request_id_to_inbox_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let request_id = request_id_fresh();
        let to = agent_address_fresh();

        for folder in folders() {
            // ensure cas
            assert!(ensure_space(&log_context, &space).is_ok());

            // append request_id
            assert!(append_request_id_to_inbox(
                &log_context,
                &space,
                &folder.into(),
                &request_id,
                &ToAddress::from(&to)
            )
            .is_ok());
        }
    }

    #[test]
    fn put_inbox_message_test() {
        let log_context = "put_inbox_message_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let request_id = request_id_fresh();
        let from = agent_address_fresh();
        let to = agent_address_fresh();
        let content = message_content_fresh();
        let is_response = false;

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // pub inbox message
        assert!(put_inbox_message(
            &log_context,
            &space,
            &request_id,
            &from.into(),
            &to.into(),
            &content,
            is_response,
        )
        .is_ok());
    }

    #[test]
    fn send_to_agent_inbox_test() {
        let log_context = "send_to_agent_inbox_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let request_id = request_id_fresh();
        let from = agent_address_fresh();
        let to = agent_address_fresh();
        let content = message_content_fresh();
        let is_response = false;

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // pub inbox message
        assert!(send_to_agent_inbox(
            &log_context,
            &space,
            &request_id,
            &FromAddress::from(&from),
            &ToAddress::from(&to),
            &content,
            is_response,
        )
        .is_ok());
    }

    #[test]
    fn get_inbox_request_ids_test() {
        let log_context = "get_inbox_request_ids_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let request_id = request_id_fresh();
        let from = agent_address_fresh();
        let to = agent_address_fresh();
        let content = message_content_fresh();
        let is_response = false;

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // pub inbox message
        assert!(send_to_agent_inbox(
            &log_context,
            &space,
            &request_id,
            &FromAddress::from(&from),
            &ToAddress::from(&to),
            &content,
            is_response,
        )
        .is_ok());

        // get inbox message
        match get_inbox_request_ids(
            &log_context,
            &space,
            &ALL_MESSAGES_FOLDER.into(),
            &to.into(),
        ) {
            Ok(request_ids) => assert_eq!(vec![request_id.clone()], request_ids),
            Err(err) => panic!("incorrect request id {:?}", err),
        };
    }

    #[test]
    fn check_inbox_test() {
        let log_context = "get_inbox_request_ids_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let request_id = request_id_fresh();
        let from = agent_address_fresh();
        let to = agent_address_fresh();
        let content = message_content_fresh();
        let is_response = false;

        let direct_message_data = DirectMessageData {
            content: content.clone().into(),
            from_agent_id: from.clone().into(),
            to_agent_id: to.clone().into(),
            request_id: request_id.clone().into(),
            space_address: space.space_address().into(),
        };

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // pub inbox message
        assert!(send_to_agent_inbox(
            &log_context,
            &space,
            &request_id,
            &FromAddress::from(&from),
            &ToAddress::from(&to),
            &content,
            is_response,
        )
        .is_ok());

        // check inbox
        match check_inbox(&log_context, &space, &to.clone().into()) {
            Ok(messages) => assert_eq!(vec![(direct_message_data.clone(), is_response)], messages),
            Err(err) => panic!("incorrect request id {:?}", err),
        };

        // check again, should be empty
        match check_inbox(&log_context, &space, &to.clone().into()) {
            Ok(request_ids) => {
                let v: Vec<(DirectMessageData, bool)> = Vec::new();
                assert_eq!(v, request_ids);
            }
            Err(err) => panic!("incorrect request id {:?}", err),
        };
    }
}
