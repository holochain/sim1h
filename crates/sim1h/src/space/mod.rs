pub mod fixture;
use crate::dht::bbdht::dynamodb::client::connection::Connection;
use crate::network::NetworkId;
use holochain_persistence_api::cas::content::Address;

#[derive(Clone, Default)]
pub struct SpaceAddress(Address);

#[derive(Clone, Default)]
pub struct Space {
    connection: Connection,
    network_id: NetworkId,
    space_address: SpaceAddress,
}

impl Space {
    pub fn new(
        connection: &Connection,
        network_id: &NetworkId,
        space_address: &SpaceAddress,
    ) -> Space {
        Space {
            connection: connection.to_owned(),
            network_id: network_id.to_owned(),
            space_address: space_address.to_owned(),
        }
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn network_id(&self) -> &NetworkId {
        &self.network_id
    }

    pub fn space_address(&self) -> &SpaceAddress {
        &self.space_address
    }
}

impl From<String> for SpaceAddress {
    fn from(string: String) -> Self {
        SpaceAddress(string.into())
    }
}

impl From<SpaceAddress> for String {
    fn from(space_address: SpaceAddress) -> Self {
        space_address.0.into()
    }
}

impl From<&SpaceAddress> for String {
    fn from(space_address: &SpaceAddress) -> Self {
        (*space_address).clone().into()
    }
}

impl From<SpaceAddress> for Address {
    fn from(space_address: SpaceAddress) -> Self {
        space_address.0
    }
}

impl From<&SpaceAddress> for Address {
    fn from(space_address: &SpaceAddress) -> Self {
        (*space_address).clone().into()
    }
}
