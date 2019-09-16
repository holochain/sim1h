//! implementation specific simplification of dynamodb api

use tokio::runtime::Runtime;
use crate::dht::bbdht::dynamodb::client::Client;
use dynomite::{
    dynamodb::{
        DynamoDb, ListTablesInput, ListTablesError
    },
};
use rusoto_core::RusotoError;
use rusoto_dynamodb::ListTablesOutput;

pub fn list_tables(mut runtime: Runtime, client: Client) -> Result<ListTablesOutput, RusotoError<ListTablesError>> {
    let list_tables_input: ListTablesInput = Default::default();
    runtime.block_on(client.list_tables(list_tables_input))
}

#[cfg(test)]
pub mod test {

    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use rusoto_dynamodb::ListTablesOutput;
    use crate::dht::bbdht::dynamodb::api::list_tables;
    use crate::dht::bbdht::dynamodb::client::local::local_runtime;

    #[test]
    pub fn list_tables_test() {

        let local_client = local_client();
        let local_runtime = local_runtime();

        let result = list_tables(local_runtime, local_client);

        assert_eq!(
            Ok(ListTablesOutput { last_evaluated_table_name: None, table_names: Some([].to_vec()) }),
            result,
        );

    }

}
