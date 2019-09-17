use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DescribeTableError;
use rusoto_dynamodb::DescribeTableInput;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::TableDescription;

pub fn describe_table(
    client: &Client,
    table_name: &str,
) -> Result<TableDescription, RusotoError<DescribeTableError>> {
    match client
        .describe_table(DescribeTableInput {
            table_name: table_name.to_string(),
        })
        .sync()?
        .table
    {
        Some(table_description) => Ok(table_description),
        None => Err(RusotoError::Service(DescribeTableError::ResourceNotFound(
            String::from("None returned for table description"),
        ))),
    }
}

#[cfg(test)]
pub mod test {

    use crate::dht::bbdht::dynamodb::schema::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::schema::fixture::key_schema_a;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::table::create::create_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::api::table::describe::describe_table;
    use crate::test::setup;

    use rusoto_core::RusotoError;
    use rusoto_dynamodb::DescribeTableError;

    #[test]
    fn describe_table_test() {
        setup();

        info!("describe_table_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        info!("describe_table_test check table not exists at init");
        assert!(
            !table_exists(&local_client, &table_name).expect("could not check that table exists")
        );

        info!("describe_table_test create the table");
        assert!(create_table(
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        )
        .is_ok());

        info!(
            "describe_table_test check the table was created {}",
            table_name
        );
        assert!(
            table_exists(&local_client, &table_name).expect("could not check that table exists")
        );

        info!("describe_table_test check the description of the table");
        assert_eq!(
            Some(String::from("ACTIVE")),
            describe_table(&local_client, &table_name).expect("could not describe table").table_status,
        );
    }

    #[test]
    fn describe_table_missing_test() {
        setup();

        info!("describe_table_missing_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();

        info!("describe_table_missing_test describe a table that does not exist");
        let description = describe_table(&local_client, &table_name);
        assert_eq!(
            Err(RusotoError::Service(DescribeTableError::ResourceNotFound(String::from("Cannot do operations on a non-existent table")))),
            description,
        );
    }

}
