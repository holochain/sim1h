//! settings and convenience fns for a local client

use crate::dht::bbdht::dynamodb::client::client;
use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_core::region::Region;

/// the region means nothing for a local install
const LOCAL_REGION: &str = "us-east-1";
/// the endpoint needs to be explicitly set to hit the local database
const LOCAL_ENDPOINT: &str = "http://localhost:8000";

pub fn local_region() -> Region {
    Region::Custom {
        name: LOCAL_REGION.into(),
        endpoint: LOCAL_ENDPOINT.into(),
    }
}

pub fn local_client() -> Client {
    client(local_region())
}

#[cfg(test)]
pub mod tests {
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::local::local_region;
    use crate::dht::bbdht::dynamodb::client::local::LOCAL_ENDPOINT;
    use crate::dht::bbdht::dynamodb::client::local::LOCAL_REGION;

    use crate::test::setup;
    use rusoto_core::region::Region;

    #[test]
    /// check the value is what we want
    fn local_region_test() {
        setup();

        info!("local_region_test compare values");
        let region = local_region();
        assert_eq!(
            Region::Custom {
                name: LOCAL_REGION.into(),
                endpoint: LOCAL_ENDPOINT.into(),
            },
            region
        );
    }

    #[test]
    fn local_client_smoke_test() {
        setup();

        info!("build local client");
        local_client();
    }
}