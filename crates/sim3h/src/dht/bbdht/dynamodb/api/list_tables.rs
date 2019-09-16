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

pub fn table_exists(
    runtime: &mut Runtime,
    client: &Client,
    table_name: &str,
) -> Result<bool, RusotoError<ListTablesError>> {
    let list_tables_result = list_tables(runtime, client);
    match list_tables_result {
        Ok(list_tables_output) => {
            let table_names = list_tables_output.table_names;
            info!("table_exists {:?}", table_names);
            match table_names {
                Some(names) => Ok(names.contains(&table_name.to_string())),
                None => Ok(false),
            }
        }
        Err(err) => Err(err),
    }
}

pub fn wait_until_table_exists_or_not(
    runtime: &mut Runtime,
    client: &Client,
    table_name: &str,
    exists: bool,
) {
    loop {
        match table_exists(runtime, client, table_name) {
            Ok(does_exist) => {
                info!("wait_until_table_exists_or_not {} {} {}", table_name, exists, does_exist);
                if exists == does_exist {
                    return;
                }
            }
            Err(err) => {
                error!("list error while waiting for table to exist: {}", err);
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::dht::bbdht::dynamodb::api::list_tables::list_tables;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::local::local_runtime;
    use crate::test::setup;

    #[test]
    pub fn list_tables_test() {
        setup();

        info!("list_tables_test fixtures");
        let mut local_runtime = local_runtime();
        let local_client = local_client();

        info!("list_tables_test check ok");
        assert!(list_tables(&mut local_runtime, &local_client).is_ok());
    }

}
