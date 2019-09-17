use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
use crate::dht::bbdht::dynamodb::api::table::exist::until_table_exists;
use crate::dht::bbdht::dynamodb::client::Client;
use dynomite::dynamodb::{
    CreateTableError, CreateTableInput, DescribeTableError,
};
use rusoto_core::RusotoError;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::KeySchemaElement;
use rusoto_dynamodb::ProvisionedThroughput;
use rusoto_dynamodb::TableDescription;

pub fn create_table(
    client: &Client,
    table_name: &str,
    key_schema: &Vec<KeySchemaElement>,
    attribute_definitions: &Vec<AttributeDefinition>,
) -> Result<Option<TableDescription>, RusotoError<CreateTableError>> {
    let create_table_input = CreateTableInput {
        table_name: table_name.to_string(),
        key_schema: key_schema.clone(),
        attribute_definitions: attribute_definitions.clone(),
        provisioned_throughput: Some(ProvisionedThroughput {
            read_capacity_units: 1,
            write_capacity_units: 1,
        }),
        ..Default::default()
    };

    let output = client.create_table(create_table_input).sync()?;
    until_table_exists(client, table_name);
    Ok(output.table_description)
}

pub fn create_table_if_not_exists(
    client: &Client,
    table_name: &str,
    key_schema: &Vec<KeySchemaElement>,
    attribute_definitions: &Vec<AttributeDefinition>,
) -> Result<Option<TableDescription>, RusotoError<CreateTableError>> {
    // well in reality we end up with concurrency issues if we do a list or describe
    // there is a specific error returned for a table that already exists so we defuse to None
    match table_exists(client, table_name) {
        Ok(false) => match create_table(client, table_name, key_schema, attribute_definitions) {
            Ok(created) => Ok(created),
            Err(RusotoError::Service(CreateTableError::ResourceInUse(_))) => Ok(None),
            Err(err) => Err(err),
        },
        Ok(true) => Ok(None),
        Err(RusotoError::Service(DescribeTableError::InternalServerError(err))) => Err(
            RusotoError::Service(CreateTableError::InternalServerError(err)),
        ),
        // the only other error is resource in use which is false for table exists
        _ => unreachable!(),
    }
}

#[cfg(test)]
pub mod test {
    use crate::dht::bbdht::dynamodb::api::table::create::create_table;
    use crate::dht::bbdht::dynamodb::api::table::create::create_table_if_not_exists;

    use crate::dht::bbdht::dynamodb::api::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::api::fixture::key_schema_a;
    use crate::dht::bbdht::dynamodb::api::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::test::setup;

    #[test]
    fn create_table_test() {
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

    #[test]
    fn create_table_if_not_exists_test() {
        setup();

        info!("create_table_if_not_exists_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        info!("create_table_if_not_exists_test checking table not exists");
        assert!(!table_exists(&local_client, &table_name).unwrap());

        info!("create_table_if_not_exists_test creating table if not exists (first call)");
        assert!(create_table_if_not_exists(
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        )
        .is_ok());

        info!("create_table_if_not_exists_test check table exists");
        assert!(table_exists(&local_client, &table_name).unwrap());

        info!("create_table_if_not_exists_test check create again");
        assert_eq!(
            None,
            create_table_if_not_exists(
                &local_client,
                &table_name,
                &key_schema,
                &attribute_definitions,
            )
            .expect("could not check table")
        );
        assert!(table_exists(&local_client, &table_name).unwrap());
    }

}
