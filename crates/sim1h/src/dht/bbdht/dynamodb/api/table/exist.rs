use crate::dht::bbdht::dynamodb::api::table::describe::describe_table;
use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DescribeTableError;

pub fn table_exists(
    client: &Client,
    table_name: &str,
) -> Result<bool, RusotoError<DescribeTableError>> {
    let describe_table_result = describe_table(client, table_name);
    match describe_table_result {
        Ok(describe_table_output) => Ok(match describe_table_output.table {
            Some(table_description) => match table_description.table_status {
                Some(status) => {
                    if status == "ACTIVE".to_string() {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
            _ => false,
        }),
        Err(err) => match err {
            RusotoError::Service(DescribeTableError::ResourceNotFound(_)) => Ok(false),
            _ => Err(err),
        },
    }
}

pub fn until_table_exists_or_not(client: &Client, table_name: &str, exists: bool) {
    loop {
        match table_exists(client, table_name) {
            Ok(does_exist) => {
                if exists == does_exist {
                    break;
                }
            }
            Err(err) => {
                error!("list error while waiting for table to exist: {}", err);
            }
        }
    }
}

pub fn until_table_exists(client: &Client, table_name: &str) {
    until_table_exists_or_not(client, table_name, true);
}

pub fn until_table_not_exists(client: &Client, table_name: &str) {
    until_table_exists_or_not(client, table_name, false);
}
