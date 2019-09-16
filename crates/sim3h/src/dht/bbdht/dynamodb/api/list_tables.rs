use crate::dht::bbdht::dynamodb::client::Client;
use dynomite::dynamodb::{DynamoDb, ListTablesError, ListTablesInput, ListTablesOutput};
use rusoto_core::RusotoError;
use tokio::runtime::Runtime;

pub fn list_tables(
    runtime: &mut Runtime,
    client: &Client,
) -> Result<ListTablesOutput, RusotoError<ListTablesError>> {
    let list_tables_input: ListTablesInput = Default::default();
    runtime.block_on(client.list_tables(list_tables_input))
}

#[cfg(test)]
pub mod test {
use crate::dht::bbdht::dynamodb::api::fixture::empty_list_tables_output;
    use crate::dht::bbdht::dynamodb::api::list_tables::list_tables;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::local::local_runtime;

    #[test]
    pub fn list_tables_test() {
        let mut local_runtime = local_runtime();
        let local_client = local_client();

        let result = list_tables(&mut local_runtime, &local_client);

        assert_eq!(
            Ok(empty_list_tables_output()),
            result,
        );
    }

}
