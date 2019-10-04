pub mod fixture;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::network::NetworkId;
use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::SpaceData;

pub struct SpaceAddress(Address);

pub struct Space {
    client: Client,
    table_name: TableName,
    network_id: NetworkId,
    space_address: SpaceAddress,
}

impl Space {
    pub fn new(client: &Client, table_name: &TableName, network_id: &NetworkId, space_data: &SpaceData) -> Self {
        Space {
            client: client.clone(),
            table_name: table_name.clone(),
            network_id: network_id.clone(),
            space_address: space_data.space_address.into(),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

impl From<SpaceAddress> for String {
    fn from(space_address: SpaceAddress) -> Self {
        space_address.0.into()
    }
}
