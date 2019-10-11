use crate::agent::fixture::agent_address_fresh;
use crate::network::fixture::request_id_fresh;
use crate::network::fixture::network_id_fresh;
use lib3h_protocol::data_types::SpaceData;
use crate::space::SpaceAddress;
use crate::dht::bbdht::dynamodb::client::fixture::local_connection_fresh;
use uuid::Uuid;
use crate::space::Space;
use crate::dht::bbdht::dynamodb::client::fixture::connection_bad;

pub fn space_address_fresh() -> SpaceAddress {
    Uuid::new_v4().to_string().into()
}

pub fn space_data_fresh() -> SpaceData {
    SpaceData {
        request_id: request_id_fresh().into(),
        space_address: space_address_fresh().into(),
        agent_id: agent_address_fresh().into(),
    }
}

pub fn space_fresh() -> Space {
    Space {
        connection: local_connection_fresh(),
        network_id: network_id_fresh(),
        space_address: space_address_fresh(),
    }
}

pub fn space_bad() -> Space {
    Space {
        connection: connection_bad(),
        network_id: network_id_fresh(),
        space_address: space_address_fresh(),
    }
}
