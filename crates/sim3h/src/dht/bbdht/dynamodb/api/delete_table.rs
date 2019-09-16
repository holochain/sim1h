use crate::dht::bbdht::dynamodb::api::list_tables::wait_until_table_exists_or_not;
use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DeleteTableError;
use rusoto_dynamodb::DeleteTableInput;
use rusoto_dynamodb::DeleteTableOutput;
use rusoto_dynamodb::DynamoDb;
use tokio::runtime::Runtime;

pub fn delete_table(
    runtime: &mut Runtime,
    client: &Client,
    table_name: &str,
) -> Result<DeleteTableOutput, RusotoError<DeleteTableError>> {
    let delete_table_input = DeleteTableInput {
        table_name: table_name.to_string(),
    };
    match runtime.block_on(client.delete_table(delete_table_input)) {
        Ok(create_table_output) => {
            println!("delete {:?} {}", create_table_output, table_name);
            wait_until_table_exists_or_not(runtime, client, table_name, false);
            println!("waited create!!!");
            Ok(create_table_output)
        }
        Err(err) => Err(err),
    }
}
