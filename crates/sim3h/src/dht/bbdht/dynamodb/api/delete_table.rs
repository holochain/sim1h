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
    runtime.block_on(client.delete_table(delete_table_input))
}
