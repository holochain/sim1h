use holochain_persistence_api::cas::content::Address;
use uuid::Uuid;
use lib3h_protocol::data_types::SpaceData;
use crate::agent::fixture::agent_id_fresh;
use crate::network::fixture::request_id_fresh;

pub fn space_address_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}

pub fn space_data_fresh() -> SpaceData {
    SpaceData {
        request_id: request_id_fresh(),
        space_address: space_address_fresh(),
        agent_id: agent_id_fresh(),
    }
}
