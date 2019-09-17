use crate::dht::bbdht::dynamodb::client::Client;
use dynomite::dynamodb::{DynamoDb, ListTablesError, ListTablesInput, ListTablesOutput};
use rusoto_core::RusotoError;
use std::{thread, time};

const EXISTS_WAIT: u64 = 10;

pub fn list_tables(
    client: &Client,
) -> Result<ListTablesOutput, RusotoError<ListTablesError>> {
    let list_tables_input: ListTablesInput = Default::default();
    client.list_tables(list_tables_input).sync()
}

pub fn table_exists(
    client: &Client,
    table_name: &str,
) -> Result<bool, RusotoError<ListTablesError>> {
    let list_tables_result = list_tables(client);
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
    client: &Client,
    table_name: &str,
    exists: bool,
) {
    loop {
        let ten_millis = time::Duration::from_millis(EXISTS_WAIT);
        thread::sleep(ten_millis);

        match table_exists(client, table_name) {
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
    use crate::test::setup;

    #[test]
    pub fn list_tables_test() {
        setup();

        info!("list_tables_test fixtures");
        let local_client = local_client();

        info!("list_tables_test check ok");
        assert!(list_tables(&local_client).is_ok());
    }

}
