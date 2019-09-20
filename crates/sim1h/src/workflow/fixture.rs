use crate::agent::fixture::agent_id_fresh;
use holochain_core_types::network::query::NetworkQuery;
use holochain_json_api::json::JsonString;
use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::EntryAspectData;
use lib3h_protocol::data_types::EntryData;
use lib3h_protocol::data_types::Opaque;
use lib3h_protocol::data_types::ProvidedEntryData;
use lib3h_protocol::data_types::QueryEntryData;
use lib3h_protocol::data_types::SpaceData;
use uuid::Uuid;

pub fn request_id_fresh() -> String {
    Uuid::new_v4().to_string()
}

pub fn space_address_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}

pub fn entry_address_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}

pub fn aspect_address_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}

pub fn type_hint_fresh() -> String {
    "content".to_string()
}

pub fn opaque_fresh() -> Opaque {
    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into()
}

pub fn publish_ts_fresh() -> u64 {
    1568858140
}

pub fn entry_aspect_data_fresh() -> EntryAspectData {
    EntryAspectData {
        aspect_address: aspect_address_fresh(),
        type_hint: type_hint_fresh(),
        aspect: opaque_fresh(),
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

pub fn entry_data_fresh(entry_address: &Address) -> EntryData {
    EntryData {
        entry_address: entry_address.clone(),
        aspect_list: aspect_list_fresh(),
    }
}

pub fn space_data_fresh() -> SpaceData {
    SpaceData {
        request_id: request_id_fresh(),
        space_address: space_address_fresh(),
        agent_id: agent_id_fresh(),
    }
}

pub fn provided_entry_data_fresh(
    space_data: &SpaceData,
    entry_address: &Address,
) -> ProvidedEntryData {
    ProvidedEntryData {
        space_address: space_data.space_address.clone(),
        provider_agent_id: agent_id_fresh(),
        entry: entry_data_fresh(entry_address),
    }
}

pub fn query_fresh(_entry_address: &Address) -> Opaque {
    let query = NetworkQuery::GetEntry;
    let json: JsonString = query.into();
    json.to_bytes().into()
}

pub fn query_entry_data_fresh(space_data: &SpaceData, entry_address: &Address) -> QueryEntryData {
    QueryEntryData {
        space_address: space_data.space_address.clone(),
        entry_address: entry_address.clone(),
        request_id: request_id_fresh(),
        requester_agent_id: agent_id_fresh(),
        query: query_fresh(&entry_address),
    }
}
