pub mod fixture;
use holochain_json_api::json::JsonString;
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use holochain_core_types::network::entry_aspect::EntryAspect;
use lib3h_protocol::data_types::EntryAspectData;
use holochain_persistence_api::cas::content::AddressableContent;

pub fn entry_aspect_to_entry_aspect_data(entry_aspect: EntryAspect) -> EntryAspectData {
    EntryAspectData {
        aspect_address: entry_aspect.address(),
        type_hint: entry_aspect.type_hint(),
        aspect: JsonString::from(entry_aspect).to_bytes().into(),
        publish_ts: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() as u64,
    }
}
