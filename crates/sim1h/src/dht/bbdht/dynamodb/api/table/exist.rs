use crate::dht::bbdht::dynamodb::api::table::describe::describe_table;
use crate::dht::bbdht::dynamodb::client::Client;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DescribeTableError;

pub fn table_exists(
    client: &Client,
    table_name: &str,
) -> Result<bool, RusotoError<DescribeTableError>> {
    let table_description_result = describe_table(client, table_name);
    match table_description_result {
        Ok(table_description) => Ok(match table_description.table_status {
            Some(status) => {
                if status == "ACTIVE".to_string() {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }),
        Err(err) => match err {
            RusotoError::Service(DescribeTableError::ResourceNotFound(_)) => Ok(false),
            _ => Err(err),
        },
    }
}

pub fn until_table_exists_or_not(client: &Client, table_name: &str, exists: bool) {
    loop {
        match table_exists(client, table_name) {
            Ok(does_exist) => {
                if exists == does_exist {
                    break;
                }
            }
            Err(err) => {
                error!("list error while waiting for table to exist: {}", err);
            }
        }
    }
}

pub fn until_table_exists(client: &Client, table_name: &str) {
    until_table_exists_or_not(client, table_name, true);
}

pub fn until_table_not_exists(client: &Client, table_name: &str) {
    until_table_exists_or_not(client, table_name, false);
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::api::table::create::create_table;
    use crate::dht::bbdht::dynamodb::api::table::delete::delete_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::schema::fixture::attribute_definitions_a;
    use crate::dht::bbdht::dynamodb::schema::fixture::key_schema_a;
    use crate::test::setup;

    #[test]
    fn table_exists_test() {
        setup();

        info!("table_exists_test fixtures");

        let local_client = local_client();
        let table_name = table_name_fresh();
        let key_schema = key_schema_a();
        let attribute_definitions = attribute_definitions_a();

        info!("table_exists_test table not exists before created");
        assert!(!table_exists(&local_client, &table_name).expect("could not check if table exists"));

        info!("table_exists_test create a table");
        assert!(create_table(
            &local_client,
            &table_name,
            &key_schema,
            &attribute_definitions
        )
        .is_ok());

        info!("table_exists_test table exists after create");
        assert!(table_exists(&local_client, &table_name).expect("could not check if table exists"));

        info!("table_exists_test delete the table");
        assert!(delete_table(&local_client, &table_name).is_ok());

        info!("table_exists_test table not exists after delete");
        assert!(!table_exists(&local_client, &table_name).expect("could not check if table exists"));
    }

}
