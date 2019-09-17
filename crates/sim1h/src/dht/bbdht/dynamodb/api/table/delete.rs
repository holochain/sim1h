use crate::dht::bbdht::dynamodb::api::table::exist::until_table_not_exists;
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
    until_table_not_exists(client, table_name);
    result
}

#[cfg(test)]
pub mod test {

    use crate::dht::bbdht::dynamodb::schema::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::schema::fixture::key_schema_a;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::table::create::create_table;
    use crate::dht::bbdht::dynamodb::api::table::delete::delete_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::test::setup;

    #[test]
    fn delete_table_test() {
        setup();

        info!("delete_table_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        info!("delete_table_test check table not exists at init");
        assert!(
            !table_exists(&local_client, &table_name).expect("could not check that table exists")
        );

        info!("delete_table_test create the table");
        assert!(create_table(
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions,
        )
        .is_ok());

        info!("delete_table_test check table exists");
        assert!(
            table_exists(&local_client, &table_name).expect("could not check that table exists")
        );

        info!("delete_table_test delete table");
        assert!(delete_table(&local_client, &table_name).is_ok());

        info!("delete_table_test table deleted");
        assert!(!table_exists(&local_client, &table_name)
            .expect("could not check that the table exists"));
    }

}
