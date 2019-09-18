use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_dynamodb::DescribeLimitsOutput;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DescribeLimitsError;
use rusoto_dynamodb::DynamoDb;

pub fn describe_limits(client: &Client) -> Result<DescribeLimitsOutput, RusotoError<DescribeLimitsError>> {
    client.describe_limits().sync()
}

#[cfg(test)]
pub mod tests {

    use crate::test::setup;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::fixture::bad_client;
    use crate::dht::bbdht::dynamodb::account::describe_limits;

    #[test]
    fn describe_limits_ok_test() {
        setup();

        info!("describe_limits_ok_test fixture");
        let local_client = local_client();

        info!("describe_limits_ok_test describe limits");
        assert!(describe_limits(&local_client).is_ok());
    }

    #[test]
    fn describe_limits_bad_test() {
        setup();

        info!("describe_limits_bad_test fixture");
        let bad_client = bad_client();

        info!("describe_limits_bad_test describe limits");
        assert!(describe_limits(&bad_client).is_err());
    }

}
