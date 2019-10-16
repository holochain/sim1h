use crate::agent::fixture::agent_address_fresh;
use crate::dht::bbdht::dynamodb::client::fixture::connection_bad;
use crate::dht::bbdht::dynamodb::client::fixture::local_connection_fresh;
use crate::network::fixture::network_id_fresh;
use crate::network::fixture::request_id_fresh;
use crate::space::Space;
use lib3h_protocol::data_types::SpaceData;
use lib3h_protocol::types::SpaceHash;
use uuid::Uuid;

pub fn space_hash_fresh() -> SpaceHash {
    Uuid::new_v4().to_string().into()
}

pub fn space_data_fresh() -> SpaceData {
    SpaceData {
        request_id: request_id_fresh().into(),
        space_hash: space_hash_fresh().into(),
        agent_id: agent_address_fresh().into(),
    }
}

pub fn space_fresh() -> Space {
    Space {
        connection: local_connection_fresh(),
        network_id: network_id_fresh(),
        space_hash: space_hash_fresh(),
    }
}

pub fn space_bad() -> Space {
    Space {
        connection: connection_bad(),
        network_id: network_id_fresh(),
        space_hash: space_hash_fresh(),
    }
}
