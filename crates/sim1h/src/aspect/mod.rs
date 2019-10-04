pub mod fixture;
use holochain_core_types::network::entry_aspect::EntryAspect;
use holochain_json_api::json::JsonString;
use holochain_persistence_api::cas::content::AddressableContent;
use lib3h_protocol::data_types::EntryAspectData;
use holochain_persistence_api::cas::content::Address;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[derive(Debug)]
pub struct AspectAddress(Address);
pub struct EntryAddress(Address);

impl From<String> for AspectAddress {
    fn from(string: String) -> Self {
        AspectAddress(string.into())
    }
}

impl From<AspectAddress> for String {
    fn from(aspect_address: AspectAddress) -> Self {
        aspect_address.0.into()
    }
}

impl From<AspectAddress> for Address {
    fn from(aspect_address: AspectAddress) -> Self {
        aspect_address.0
    }
}

impl From<EntryAddress> for String {
    fn from(entry_address: EntryAddress) -> Self {
        entry_address.0.into()
    }
}

pub fn entry_aspect_to_entry_aspect_data(entry_aspect: EntryAspect) -> EntryAspectData {
    EntryAspectData {
        aspect_address: entry_aspect.address(),
        type_hint: entry_aspect.type_hint(),
        aspect: JsonString::from(entry_aspect).to_bytes().into(),
        publish_ts: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64,
    }
}
