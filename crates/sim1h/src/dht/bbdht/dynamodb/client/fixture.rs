//! fixtures for test clients

use crate::dht::bbdht::dynamodb::client::client;
use crate::dht::bbdht::dynamodb::client::connection::Connection;
use crate::dht::bbdht::dynamodb::client::local::local_client;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
use rusoto_core::region::Region;

/// the region means nothing for a local install
const BAD_REGION: &str = "badbad";
/// the endpoint needs to be explicitly set to hit the local database
const BAD_ENDPOINT: &str = "http://example.com";

pub fn bad_region() -> Region {
    Region::Custom {
        name: BAD_REGION.into(),
        endpoint: BAD_ENDPOINT.into(),
    }
}

pub fn bad_client() -> Client {
    client(bad_region())
}

pub fn local_connection_fresh() -> Connection {
    Connection::new(
        &local_client(),
        &table_name_fresh(),
    )
}

pub fn connection_bad() -> Connection {
    Connection::new(
        &bad_client(),
        &table_name_fresh(),
    )
}

#[cfg(test)]
pub mod tests {
    use crate::dht::bbdht::dynamodb::client::fixture::bad_client;
    use crate::dht::bbdht::dynamodb::client::fixture::bad_region;
    use crate::dht::bbdht::dynamodb::client::fixture::BAD_ENDPOINT;
    use crate::dht::bbdht::dynamodb::client::fixture::BAD_REGION;

    use crate::trace::tracer;
    use rusoto_core::region::Region;

    #[test]
    /// check the value is what we want
    fn bad_region_test() {
        let log_context = "bad_region_test";

        tracer(&log_context, "compare values");
        let bad_region = bad_region();
        assert_eq!(
            Region::Custom {
                name: BAD_REGION.into(),
                endpoint: BAD_ENDPOINT.into(),
            },
            bad_region
        );
    }

    #[test]
    fn bad_client_smoke_test() {
        let log_context = "bad_client_smoke_test";

        tracer(&log_context, "smoke test");
        bad_client();
    }
}
