use crate::agent::AgentAddress;
use crate::aspect::AspectAddress;
use crate::entry::EntryAddress;
use crate::dht::bbdht::dynamodb::schema::cas::PARTITION_KEY;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::network::RequestId;
use crate::space::Space;
use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::EntryAspectData;
use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub mod fixture;
pub mod read;
pub mod write;

pub type Item = HashMap<String, AttributeValue>;

#[derive(Debug, Clone)]
pub struct ItemKey(String);

impl From<&RequestId> for String {
    fn from(request_id: &RequestId) -> Self {
        (*request_id).clone().into()
    }
}

impl From<RequestId> for ItemKey {
    fn from(request_id: RequestId) -> Self {
        ItemKey(request_id.into())
    }
}

impl From<&RequestId> for ItemKey {
    fn from(request_id: &RequestId) -> Self {
        (*request_id).clone().into()
    }
}

impl From<String> for ItemKey {
    fn from(string: String) -> Self {
        ItemKey(string)
    }
}

impl From<ItemKey> for String {
    fn from(item_key: ItemKey) -> String {
        item_key.0
    }
}

impl From<&ItemKey> for String {
    fn from(item_key: &ItemKey) -> String {
        (*item_key).clone().into()
    }
}

impl From<Address> for ItemKey {
    fn from(address: Address) -> Self {
        ItemKey(address.into())
    }
}

impl From<AgentAddress> for ItemKey {
    fn from(agent_address: AgentAddress) -> Self {
        ItemKey(agent_address.into())
    }
}

impl From<&AgentAddress> for ItemKey {
    fn from(agent_address: &AgentAddress) -> Self {
        (*agent_address).clone().into()
    }
}

impl From<EntryAspectData> for ItemKey {
    fn from(entry_aspect_data: EntryAspectData) -> Self {
        entry_aspect_data.aspect_address.into()
    }
}

impl From<&EntryAspectData> for ItemKey {
    fn from(entry_aspect_data: &EntryAspectData) -> Self {
        (*entry_aspect_data).clone().into()
    }
}

impl From<AspectAddress> for ItemKey {
    fn from(aspect_address: AspectAddress) -> Self {
        ItemKey(aspect_address.into())
    }
}

impl From<&AspectAddress> for ItemKey {
    fn from(aspect_address: &AspectAddress) -> Self {
        (*aspect_address).clone().into()
    }
}

impl From<EntryAddress> for ItemKey {
    fn from(entry_address: EntryAddress) -> Self {
        ItemKey(entry_address.into())
    }
}

impl From<&EntryAddress> for ItemKey {
    fn from(entry_address: &EntryAddress) -> Self {
        (*entry_address).clone().into()
    }
}

pub fn partition_key(space: &Space, address: &String) -> String {
    format!("{}:{}:{}", String::from(space.network_id()), String::from(space.space_address()), address)
}

pub fn keyed_item(space: &Space, item_key: &ItemKey) -> Item {
    let mut item = HashMap::new();
    item.insert(
        String::from(PARTITION_KEY),
        string_attribute_value(&partition_key(
            &space,
            &item_key.into(),
        )),
    );
    item
}
