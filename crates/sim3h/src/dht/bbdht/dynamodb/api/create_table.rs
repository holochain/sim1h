use crate::dht::bbdht::dynamodb::api::list_tables::wait_until_table_exists_or_not;
use crate::dht::bbdht::dynamodb::client::Client;
use dynomite::dynamodb::{CreateTableError, CreateTableInput, CreateTableOutput};
use rusoto_core::RusotoError;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::KeySchemaElement;
use rusoto_dynamodb::ProvisionedThroughput;
use tokio::runtime::Runtime;

pub fn create_table(
    client: &Client,
    table_name: &str,
    key_schema: &Vec<KeySchemaElement>,
    attribute_definitions: &Vec<AttributeDefinition>,
) -> Result<CreateTableOutput, RusotoError<CreateTableError>> {
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

    let create_table_future = client.create_table(create_table_input);
    loop {
        match create_table_future.poll() {
        Ok(create_table_output) => {
            wait_until_table_exists_or_not(runtime, client, table_name, true);
            Ok(create_table_output)
        }
        Err(err) => Err(err)
    }
    }
}

pub fn create_table_if_not_exists(
    runtime: &mut Runtime,
    client: &Client,
    table_name: &str,
    key_schema: &Vec<KeySchemaElement>,
    attribute_definitions: &Vec<AttributeDefinition>,
) -> Result<Option<CreateTableOutput>, RusotoError<CreateTableError>> {
    // well in reality we end up with concurrency issues if we do a list or describe
    // there is a specific error returned for a table that already exists so we defuse to None
    match create_table(
        runtime,
        client,
        table_name,
        key_schema,
        attribute_definitions,
    ) {
        Ok(created) => Ok(Some(created)),
        Err(RusotoError::Service(CreateTableError::ResourceInUse(_))) => Ok(None),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
pub mod test {
    use crate::dht::bbdht::dynamodb::api::create_table::create_table;
    use crate::dht::bbdht::dynamodb::api::create_table::create_table_if_not_exists;

    use crate::dht::bbdht::dynamodb::api::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::api::fixture::key_schema_a;
    use crate::dht::bbdht::dynamodb::api::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::list_tables::table_exists;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::local::local_runtime;
    use crate::test::setup;

    #[test]
    fn create_table_test() {
        setup();

        info!("create_table_test fixtures");
        let mut local_runtime = local_runtime();
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        info!("create_table_test check table not exists at init");
        assert!(
            !table_exists(&mut local_runtime, &local_client, &table_name)
                .expect("could not check that table exists")
        );

        info!("create_table_test create the table");
        let create_table_result = create_table(
            &mut local_runtime,
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        );

        info!("create_table_test check the table was created {}", table_name);
        assert!(create_table_result.is_ok());
        assert!(table_exists(&mut local_runtime, &local_client, &table_name)
            .expect("could not check that table exists"));
    }

    // #[test]
    // fn create_table_if_not_exists_test() {
    //     setup();
    //
    //     info!("create_table_if_not_exists_test fixtures");
    //     let mut local_runtime = local_runtime();
    //     let local_client = local_client();
    //     let table_name = table_name_fresh();
    //     let key_schema = key_schema_a();
    //     let attribute_definitions = attribute_definitions_a();
    //
    //     info!("create_table_if_not_exists_test checking table not exists");
    //     assert!(!table_exists(&mut local_runtime, &local_client, &table_name).unwrap());
    //
    //     info!("create_table_if_not_exists_test creating table if not exists (first call)");
    //     let create_table_if_not_exists_result = create_table_if_not_exists(
    //         &mut local_runtime,
    //         &local_client,
    //         &table_name,
    //         &key_schema,
    //         &attribute_definitions,
    //     );
    //
    //     info!("create_table_if_not_exists_test check table exists");
    //     assert!(create_table_if_not_exists_result.is_ok());
    //     assert!(table_exists(&mut local_runtime, &local_client, &table_name).unwrap());
    // }

}
