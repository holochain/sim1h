use holochain_persistence_api::cas::content::Address;
use uuid::Uuid;
use holochain_core_types::entry::Entry;
use holochain_core_types::chain_header::ChainHeader;
use crate::agent::fixture::core_agent_id_fresh;
use crate::agent::fixture::provenances_fresh;
use lib3h_protocol::data_types::EntryData;
use crate::network::fixture::timestamp_fresh;
use crate::aspect::fixture::aspect_list_fresh;
use holochain_persistence_api::cas::content::AddressableContent;

pub fn entry_address_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}

pub fn entry_fresh() -> Entry {
    Entry::AgentId(core_agent_id_fresh())
}

pub fn header_address_fresh() -> Address {
    Uuid::new_v4().to_string().into()
}

pub fn chain_header_fresh(entry: &Entry) -> ChainHeader {
    ChainHeader::new(
        &entry.entry_type(),
        &entry.address(),
        &provenances_fresh(),
        &Some(header_address_fresh()),
        &Some(header_address_fresh()),
        &Some(header_address_fresh()),
        &timestamp_fresh(),
    )
}

pub fn entry_data_fresh(entry_address: &Address) -> EntryData {
    EntryData {
        entry_address: entry_address.clone(),
        aspect_list: aspect_list_fresh(),
    }
}
