use crate::dht::bbdht::dynamodb::client::Client;
use dynomite::dynamodb::{CreateTableError, CreateTableInput, CreateTableOutput};
use rusoto_core::RusotoError;
use rusoto_dynamodb::AttributeDefinition;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::KeySchemaElement;
use rusoto_dynamodb::ProvisionedThroughput;
use tokio::runtime::Runtime;

pub fn create_table(
    runtime: &mut Runtime,
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
    runtime.block_on(client.create_table(create_table_input))
}

#[cfg(test)]
pub mod test {

    use crate::dht::bbdht::dynamodb::api::create_table::create_table;
    use crate::dht::bbdht::dynamodb::api::delete_table::delete_table;
    use crate::dht::bbdht::dynamodb::api::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::api::fixture::key_schema_a;
    use crate::dht::bbdht::dynamodb::api::fixture::table_name_a;
    use crate::dht::bbdht::dynamodb::api::list_tables::list_tables;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::local::local_runtime;
    use rusoto_dynamodb::ListTablesOutput;

    #[test]
    fn create_table_test() {
        let mut local_runtime = local_runtime();
        let local_client = local_client();
        let table_name = table_name_a();

        let create_table_result = create_table(
            &mut local_runtime,
            &local_client,
            table_name,
            &key_schema_a(),
            &attribute_definitions_a(),
        );
        assert!(create_table_result.is_ok());

        let list_tables_result = list_tables(&mut local_runtime, &local_client);
        assert_eq!(
            Ok(ListTablesOutput {
                last_evaluated_table_name: None,
                table_names: Some(vec![table_name.into()])
            }),
            list_tables_result
        );

        let delete_table_result = delete_table(&mut local_runtime, &local_client, table_name);
        assert!(delete_table_result.is_ok());

        let list_tables_result_2 = list_tables(&mut local_runtime, &local_client);
        assert!(list_tables_result_2.is_ok());
        assert_eq!(
            list_tables_result_2.unwrap().table_names,
            Some(Vec::new()),
        );
    }

}
