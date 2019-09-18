pub mod local;

use dynomite::dynamodb::DynamoDbClient;
use dynomite::retry::Policy;
use dynomite::retry::RetryingDynamoDb;
use dynomite::Retries;
// use rusoto_dynamodb::DynamoDb;
use rusoto_core::Region;
// use rusoto_dynamodb::DescribeEndpointsError;
// use rusoto_core::RusotoError;
// use rusoto_dynamodb::DescribeEndpointsResponse;

pub type Client = RetryingDynamoDb<DynamoDbClient>;

pub fn client(region: Region) -> Client {
    DynamoDbClient::new(region).with_retries(Policy::default())
}

// pub fn client_endpoint_url(client: &Client) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>> {
//     client.describe_endpoints().sync()
// }

#[cfg(test)]
pub mod test {
    use crate::dht::bbdht::dynamodb::client::client;
    use rusoto_core::region::Region;
    // use crate::dht::bbdht::dynamodb::client::client_endpoint_url;
    // use crate::dht::bbdht::dynamodb::client::local::local_client;

    use crate::test::setup;

    #[test]
    fn client_smoke_test() {
        setup();

        info!("client_smoke_test building client with some config");
        client(Region::SaEast1);
    }

    // #[test]
    // fn client_endpoint_url_test() {
    //     setup();
    //
    //     let local_client = local_client();
    //
    //     info!("{:?}", client_endpoint_url(&local_client));
    //
    // }

}
