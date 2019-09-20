use holochain_json_api::json::JsonString;
use holochain_core_types::network::entry_aspect::EntryAspect;
use holochain_persistence_api::cas::content::Address;
use uuid::Uuid;
use lib3h_protocol::data_types::EntryAspectData;
use lib3h_protocol::data_types::Opaque;
use crate::entry::fixture::entry_fresh;
use crate::entry::fixture::chain_header_fresh;

pub fn entry_aspect_data_fresh() -> EntryAspectData {
    EntryAspectData {
        aspect_address: aspect_address_fresh(),
        type_hint: type_hint_fresh(),
        aspect: opaque_aspect_fresh(),
        publish_ts: publish_ts_fresh(),
    }
}

pub fn aspect_list_fresh() -> Vec<EntryAspectData> {
    let mut aspects = Vec::new();

    for _ in 0..10 {
        aspects.push(entry_aspect_data_fresh())
    }

    aspects.into()
}

pub fn opaque_aspect_fresh() -> Opaque {
    let entry = entry_fresh();
    JsonString::from(EntryAspect::Content(
        entry.clone(),
        chain_header_fresh(&entry),
    )).to_bytes().into()
}

pub fn aspect_address_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}

pub fn type_hint_fresh() -> String {
    "content".to_string()
}

pub fn publish_ts_fresh() -> u64 {
    1568858140
}
