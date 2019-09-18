pub mod local;
pub mod fixture;

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
    use crate::test::setup;

    #[test]
    fn client_smoke_test() {
        setup();

        info!("client_smoke_test building client with some config");
        client(Region::SaEast1);
    }

}
