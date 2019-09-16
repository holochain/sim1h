const LOCAL_REGION: &str = "us-east-1";
const LOCAL_ENDPOINT: &str = "http://localhost:8000";

pub fn local_region() -> Region {
    Region {
        name: LOCAL_REGION.into(),
        endpoint: LOCAL_ENDPOINT.into(),
    }
}

pub fn local_client() -> DynamoDbClient {
    client(local_region())
}

#[cfg(test)]
pub mod tests {
    use rusoto_core::Region;
    use dynomite::{
        dynamodb::{
            DynamoDb, DynamoDbClient, ListTablesInput
        },
        retry::Policy,
        Retries,
    };
    use tokio::runtime::Runtime;
    use rusoto_dynamodb::ListTablesOutput;

    #[test]
    /// we should be able to open up a connection to the local db and find it empty
    fn local_connection_test() {
        let mut rt = Runtime::new().expect("failed to initialize futures runtime");
        let client = DynamoDbClient::new(local_region())
        .with_retries(Policy::default());

        let list_tables_input: ListTablesInput = Default::default();

        let foo = rt.block_on(client.list_tables(list_tables_input));

        assert_eq!(Ok(ListTablesOutput { last_evaluated_table_name: None, table_names: Some([].to_vec()) }), foo);

    }
}
