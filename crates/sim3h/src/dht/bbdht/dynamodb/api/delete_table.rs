use crate::dht::bbdht::dynamodb::api::list_tables::wait_until_table_exists_or_not;
use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DeleteTableError;
use rusoto_dynamodb::DeleteTableInput;
use rusoto_dynamodb::DeleteTableOutput;
use rusoto_dynamodb::DynamoDb;

pub fn delete_table(
    client: &Client,
    table_name: &str,
) -> Result<DeleteTableOutput, RusotoError<DeleteTableError>> {
    let delete_table_input = DeleteTableInput {
        table_name: table_name.to_string(),
    };
    let result = client.delete_table(delete_table_input).sync();
    wait_until_table_exists_or_not(client, table_name, false);
    result
}
