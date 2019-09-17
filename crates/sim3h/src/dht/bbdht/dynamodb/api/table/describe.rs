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

    use crate::dht::bbdht::dynamodb::api::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::api::fixture::key_schema_a;
    use crate::dht::bbdht::dynamodb::api::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::table::create::create_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::test::setup;

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
        assert!(create_table(
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        )
        .is_ok());

        info!(
            "create_table_test check the table was created {}",
            table_name
        );
        assert!(
            table_exists(&local_client, &table_name).expect("could not check that table exists")
        );
    }

}
