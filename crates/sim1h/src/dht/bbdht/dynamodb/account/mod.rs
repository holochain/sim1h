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

    use crate::log::trace;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::fixture::bad_client;
    use crate::dht::bbdht::dynamodb::account::describe_limits;

    #[test]
    fn describe_limits_ok_test() {
        let log_context = "describe_limits_ok_test";

        trace(&log_context, "fixtures");
        let local_client = local_client();

        // describe limits
        assert!(describe_limits(&local_client).is_ok());
    }

    #[test]
    fn describe_limits_bad_test() {
        let log_context = "describe_limits_bad_test";

        trace(&log_context, "fixtures");
        let bad_client = bad_client();

        // fail to describe limits
        assert!(describe_limits(&bad_client).is_err());
    }

}
