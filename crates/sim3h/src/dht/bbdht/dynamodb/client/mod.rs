pub mod local;

use dynomite::dynamodb::DynamoDbClient;
use dynomite::retry::Policy;
use dynomite::retry::RetryingDynamoDb;
use dynomite::Retries;
use rusoto_core::Region;

pub type Client = RetryingDynamoDb<DynamoDbClient>;

pub fn client(region: Region) -> Client {
    DynamoDbClient::new(region).with_retries(Policy::default())
}

#[cfg(test)]
pub mod test {
    use crate::dht::bbdht::dynamodb::client::client;
    use rusoto_core::region::Region;

    #[test]
    fn client_smoke_test() {
        client(Region::SaEast1);
    }

}