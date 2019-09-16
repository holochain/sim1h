use crate::dht::bbdht::dynamodb::client::client;
use dynomite::dynamodb::DynamoDbClient;
use rusoto_core::region::Region;
use tokio::runtime::Runtime;
use dynomite::retry::RetryingDynamoDb;

/// the region means nothing for a local install
const LOCAL_REGION: &str = "us-east-1";
/// the endpoint needs to be explicitly set to hit the local database
const LOCAL_ENDPOINT: &str = "http://localhost:8000";

pub fn local_runtime() -> Runtime {
    Runtime::new().expect("failed to initialize futures runtime for local dynamodb client")
}

pub fn local_region() -> Region {
    Region::Custom{
        name: LOCAL_REGION.into(),
        endpoint: LOCAL_ENDPOINT.into(),
    }
}

pub fn local_client() -> RetryingDynamoDb<DynamoDbClient> {
    client(local_region())
}

#[cfg(test)]
pub mod tests {
    use crate::dht::bbdht::dynamodb::local::local_client;
    use crate::dht::bbdht::dynamodb::local::local_runtime;
    use crate::dht::bbdht::dynamodb::local::local_region;
    use crate::dht::bbdht::dynamodb::local::LOCAL_REGION;
    use crate::dht::bbdht::dynamodb::local::LOCAL_ENDPOINT;
    use dynomite::{
        dynamodb::{
            DynamoDb, ListTablesInput
        },
    };
    use rusoto_core::region::Region;
    use rusoto_dynamodb::ListTablesOutput;

    #[test]
    /// boot a local runtime
    fn local_runtime_smoke_test() {
        local_runtime();
    }

    #[test]
    /// check the value is what we want
    fn local_region_test() {
        let region = local_region();
        assert_eq!(
            Region::Custom {
                name: LOCAL_REGION.into(),
                endpoint: LOCAL_ENDPOINT.into(),
            },
            region);
    }

    #[test]
    /// we should be able to open up a connection to the local db and find it empty
    fn local_client_test() {
        let client = local_client();

        let list_tables_input: ListTablesInput = Default::default();

        let foo = local_runtime().block_on(client.list_tables(list_tables_input));

        assert_eq!(
            Ok(ListTablesOutput { last_evaluated_table_name: None, table_names: Some([].to_vec()) }),
            foo
        );

    }
}
