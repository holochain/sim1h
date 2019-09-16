use dynomite::Retries;
use dynomite::dynamodb::DynamoDbClient;
use rusoto_core::Region;
use dynomite::retry::Policy;
use dynomite::retry::RetryingDynamoDb;

pub fn client (region: Region) -> RetryingDynamoDb<DynamoDbClient> {
    DynamoDbClient::new(region).with_retries(Policy::default())
}

#[cfg(test)]
pub mod test {
    use rusoto_core::region::Region;
    use crate::dht::bbdht::dynamodb::client::client;

    #[test]
    fn client_smoke_test() {
        client(Region::SaEast1);
    }

}
