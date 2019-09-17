use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
use crate::dht::bbdht::dynamodb::api::table::exist::until_table_exists;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::cas::attribute_definitions_cas;
use crate::dht::bbdht::dynamodb::schema::cas::key_schema_cas;
use dynomite::dynamodb::{CreateTableError, CreateTableInput, DescribeTableError};
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

pub fn ensure_table(
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

pub fn ensure_cas_table(
    client: &Client,
    table_name: &str,
) -> Result<Option<TableDescription>, RusotoError<CreateTableError>> {
    ensure_table(
        client,
        table_name,
        &key_schema_cas(),
        &attribute_definitions_cas(),
    )
}

#[cfg(test)]
pub mod test {
    use crate::dht::bbdht::dynamodb::api::table::create::create_table;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_table;

    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::describe::describe_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::schema::cas::attribute_definitions_cas;
    use crate::dht::bbdht::dynamodb::schema::cas::key_schema_cas;
    use crate::dht::bbdht::dynamodb::schema::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::schema::fixture::key_schema_a;
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
    fn ensure_table_test() {
        setup();

        info!("ensure_table_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        info!("ensure_table_test checking table not exists");
        assert!(!table_exists(&local_client, &table_name).unwrap());

        info!("ensure_table_test creating table if not exists (first call)");
        assert!(ensure_table(
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        )
        .is_ok());

        info!("ensure_table_test check table exists");
        assert!(table_exists(&local_client, &table_name).unwrap());

        info!("ensure_table_test check create again");
        assert_eq!(
            None,
            ensure_table(
                &local_client,
                &table_name,
                &key_schema,
                &attribute_definitions,
            )
            .expect("could not check table")
        );
        assert!(table_exists(&local_client, &table_name).unwrap());
    }

    #[test]
    fn ensure_cas_table_test() {
        setup();

        info!("ensure_cas_table_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();

        info!("ensure_cas_table_test create cas table");
        assert!(ensure_cas_table(&local_client, &table_name).is_ok());

        info!("ensure_cas_table_test check table schema");
        let table_description =
            describe_table(&local_client, &table_name).expect("could not describe table");

        assert_eq!(Some(key_schema_cas()), table_description.key_schema);
        assert_eq!(
            Some(attribute_definitions_cas()),
            table_description.attribute_definitions
        );

        info!("ensure_cas_table_test thrash a bit");
        for _ in 0..100 {
            info!("thrashing the cas");
            assert!(ensure_cas_table(&local_client, &table_name).is_ok());
        }
    }

}
