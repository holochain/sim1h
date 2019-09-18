use lib3h_protocol::data_types::SpaceData;
use uuid::Uuid;
use holochain_persistence_api::cas::content::Address;
use crate::agent::fixture::agent_id_fresh;

pub fn request_id_fresh() -> String {
    Uuid::new_v4().to_string()
}

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
