pub mod fixture;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::network::NetworkId;
use holochain_persistence_api::cas::content::Address;

pub struct SpaceAddress(Address);

pub struct Space {
    pub client: Client,
    pub table_name: TableName,
    pub network_id: NetworkId,
    pub space_address: SpaceAddress,
}
