use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
use crate::dht::bbdht::dynamodb::api::table::exist::until_table_exists;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::cas::attribute_definitions_cas;
use crate::dht::bbdht::dynamodb::schema::cas::key_schema_cas;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::dht::bbdht::error::BbDhtError;
use crate::dht::bbdht::error::BbDhtResult;
use crate::trace::tracer;
use crate::trace::LogContext;
use dynomite::dynamodb::CreateTableInput;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::KeySchemaElement;
use rusoto_dynamodb::ProvisionedThroughput;
use rusoto_dynamodb::TableDescription;

pub fn create_table(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    key_schema: &Vec<KeySchemaElement>,
    attribute_definitions: &Vec<AttributeDefinition>,
) -> BbDhtResult<Option<TableDescription>> {
    tracer(&log_context, &format!("create_table {:?}", table_name));

    let create_table_input = CreateTableInput {
        table_name: table_name.into(),
        key_schema: key_schema.clone(),
        attribute_definitions: attribute_definitions.clone(),
        provisioned_throughput: Some(ProvisionedThroughput {
            read_capacity_units: 1,
            write_capacity_units: 1,
        }),
        ..Default::default()
    };

    let output = client.create_table(create_table_input).sync()?;
    until_table_exists(log_context, client, table_name);
    Ok(output.table_description)
}

pub fn ensure_table(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    key_schema: &Vec<KeySchemaElement>,
    attribute_definitions: &Vec<AttributeDefinition>,
) -> BbDhtResult<Option<TableDescription>> {
    tracer(log_context, &format!("ensure_table {:?}", &table_name));

    // well in reality we end up with concurrency issues if we do a list or describe
    // there is a specific error returned for a table that already exists so we defuse to None
    match table_exists(log_context, client, table_name) {
        Ok(false) => match create_table(
            log_context,
            client,
            table_name,
            key_schema,
            attribute_definitions,
        ) {
            Ok(created) => Ok(created),
            Err(BbDhtError::ResourceInUse(_)) => {
                tracer(&log_context, "ensure_table ResourceInUse");
                Ok(None)
            }
            Err(_err) => {
                tracer(&log_context, "ensure_table failed to create table. retry.");
                ensure_table(
                    &log_context,
                    &client,
                    &table_name,
                    &key_schema,
                    &attribute_definitions,
                )
            }
        },
        Ok(true) => Ok(None),
        Err(BbDhtError::InternalServerError(_)) => {
            tracer(&log_context, "retry ensure_table InternalServerError");
            // RusotoError::Service(CreateTableError::InternalServerError(err)),
            ensure_table(
                &log_context,
                &client,
                &table_name,
                &key_schema,
                &attribute_definitions,
            )
        }
        Err(BbDhtError::Unknown(_)) => {
            tracer(&log_context, "retry ensure_table Unknown");
            ensure_table(
                &log_context,
                &client,
                &table_name,
                &key_schema,
                &attribute_definitions,
            )
        }
        Err(BbDhtError::ResourceNotFound(_)) => {
            // this must be covered by table_exists
            tracer(&log_context, "retry ensure_table ResourceNotFound");
            ensure_table(
                &log_context,
                &client,
                &table_name,
                &key_schema,
                &attribute_definitions,
            )
        }
        Err(err) => Err(err.into()),
    }
}

pub fn ensure_cas_table(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
) -> BbDhtResult<Option<TableDescription>> {
    tracer(&log_context, &format!("ensure_cas_table {:?}", &table_name));

    ensure_table(
        log_context,
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

    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::item::write::content_to_item;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::dht::bbdht::dynamodb::api::space::exist::space_exists;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::describe::describe_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::schema::cas::attribute_definitions_cas;
    use crate::dht::bbdht::dynamodb::schema::cas::key_schema_cas;
    use crate::dht::bbdht::dynamodb::schema::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::schema::fixture::key_schema_a;
    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;
    use rusoto_dynamodb::DynamoDb;
    use rusoto_dynamodb::Put;
    use rusoto_dynamodb::TransactWriteItem;
    use rusoto_dynamodb::TransactWriteItemsInput;

    #[test]
    fn create_table_test() {
        let log_context = "create_table_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        // not exists
        assert!(!table_exists(&log_context, &local_client, &table_name)
            .expect("could not check that table exists"));

        // create
        assert!(create_table(
            &log_context,
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        )
        .is_ok());

        // exists
        assert!(table_exists(&log_context, &local_client, &table_name)
            .expect("could not check that table exists"));
    }

    #[test]
    fn ensure_table_test() {
        let log_context = "ensure_table_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        // not exists
        assert!(!table_exists(&log_context, &local_client, &table_name).unwrap());

        // ensure
        assert!(ensure_table(
            &log_context,
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        )
        .is_ok());

        // exists
        assert!(table_exists(&log_context, &local_client, &table_name).unwrap());

        // ensure again
        assert_eq!(
            None,
            ensure_table(
                &log_context,
                &local_client,
                &table_name,
                &key_schema,
                &attribute_definitions,
            )
            .expect("could not check table")
        );

        // exists
        assert!(table_exists(&log_context, &local_client, &table_name).unwrap());
    }

    #[test]
    fn ensure_cas_table_test() {
        let log_context = "ensure_cas_table_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // check cas schema
        match describe_table(&log_context, &local_client, &table_name) {
            Ok(table_description) => {
                assert_eq!(Some(key_schema_cas()), table_description.key_schema);
                assert_eq!(
                    Some(attribute_definitions_cas()),
                    table_description.attribute_definitions
                );
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }

        // thrash
        for _ in 0..100 {
            assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());
        }
    }

    #[test]
    /// older versions of dynamodb don't support transact writes
    /// test that this version is supported
    fn transact_write_item_test() {
        let log_context = "transact_write_item_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let content_a = content_fresh();
        let content_b = content_fresh();

        // ensure space
        assert!(ensure_space(&log_context, &space).is_ok());

        // space exists
        assert!(space_exists(&log_context, &space).is_ok());

        // transact
        space
            .connection()
            .client()
            .transact_write_items(TransactWriteItemsInput {
                transact_items: vec![
                    TransactWriteItem {
                        put: Some(Put {
                            table_name: space.connection().table_name().into(),
                            item: content_to_item(&space, &content_a),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    TransactWriteItem {
                        put: Some(Put {
                            table_name: space.connection().table_name().into(),
                            item: content_to_item(&space, &content_b),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
            .sync()
            .expect("could not transact write items");
    }

}
