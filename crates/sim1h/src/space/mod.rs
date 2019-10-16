pub mod fixture;
use crate::dht::bbdht::dynamodb::client::connection::Connection;
use crate::network::NetworkId;
use lib3h_protocol::types::SpaceHash;

#[derive(Clone, Default)]
pub struct Space {
    connection: Connection,
    network_id: NetworkId,
    space_hash: SpaceHash,
}

impl Space {
    pub fn new(
        connection: &Connection,
        network_id: &NetworkId,
        space_hash: &SpaceHash,
    ) -> Space {
        Space {
            connection: connection.to_owned(),
            network_id: network_id.to_owned(),
            space_hash: space_hash.to_owned(),
        }
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn network_id(&self) -> &NetworkId {
        &self.network_id
    }

    pub fn space_hash(&self) -> &SpaceHash {
        &self.space_hash
    }
}

// impl From<String> for SpaceHash {
//     fn from(string: String) -> Self {
//         SpaceHash(string.into())
//     }
// }

// impl From<SpaceHash> for String {
//     fn from(space_hash: SpaceHash) -> Self {
//         space_hash.0.into()
//     }
// }

// impl From<&SpaceHash> for String {
//     fn from(space_hash: &SpaceHash) -> Self {
//         (*space_hash).clone().into()
//     }
// }

// impl From<&SpaceHash> for Address {
//     fn from(space_hash: &SpaceHash) -> Self {
//         (*space_hash).clone().into()
//     }
// }
