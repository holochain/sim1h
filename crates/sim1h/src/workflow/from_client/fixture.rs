use crate::agent::fixture::agent_address_fresh;
use crate::entry::fixture::entry_data_fresh;
use crate::network::fixture::request_id_fresh;
use crate::space::Space;
use holochain_core_types::network::query::NetworkQuery;
use holochain_json_api::json::JsonString;
use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::Opaque;
use lib3h_protocol::data_types::ProvidedEntryData;
use lib3h_protocol::data_types::QueryEntryData;

pub fn query_fresh(_entry_address: &Address) -> Opaque {
    let query = NetworkQuery::GetEntry;
    let json: JsonString = query.into();
    json.to_bytes().into()
}

pub fn query_entry_data_fresh(space: &Space, entry_address: &Address) -> QueryEntryData {
    QueryEntryData {
        space_address: space.space_address().into(),
        entry_address: entry_address.to_owned(),
        request_id: request_id_fresh().into(),
        requester_agent_id: agent_address_fresh().into(),
        query: query_fresh(&entry_address),
    }
}

pub fn provided_entry_data_fresh(space: &Space, entry_address: &Address) -> ProvidedEntryData {
    ProvidedEntryData {
        space_address: space.space_address().into(),
        provider_agent_id: agent_address_fresh().into(),
        entry: entry_data_fresh(entry_address),
    }
}
