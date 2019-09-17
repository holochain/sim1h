use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DescribeTableError;
use rusoto_dynamodb::DescribeTableInput;
use rusoto_dynamodb::DescribeTableOutput;
use rusoto_dynamodb::DynamoDb;

pub fn describe_table(
    client: &Client,
    table_name: &str,
) -> Result<DescribeTableOutput, RusotoError<DescribeTableError>> {
    client
        .describe_table(DescribeTableInput {
            table_name: table_name.to_string(),
        })
        .sync()
}

#[cfg(test)]
pub mod test {

    #[test]
    fn describe_table_test() {
        setup();

        info!("create_table_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        info!("create_table_test check table not exists at init");
        assert!(
            !table_exists(&local_client, &table_name).expect("could not check that table exists")
        );

        info!("create_table_test create the table");
        let create_table_result = create_table(
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        );

        info!(
            "create_table_test check the table was created {}",
            table_name
        );
        assert!(create_table_result.is_ok());
        assert!(
            table_exists(&local_client, &table_name).expect("could not check that table exists")
        );

            
    }

}
