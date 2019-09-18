use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use holochain_persistence_api::cas::content::Address;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::GetItemError;
use rusoto_dynamodb::GetItemInput;
use rusoto_dynamodb::GetItemOutput;
use std::collections::HashMap;

pub fn get_item_by_address(
    client: &Client,
    table_name: &str,
    address: &Address,
) -> Result<GetItemOutput, RusotoError<GetItemError>> {
    let mut key = HashMap::new();
    key.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(address.to_owned())),
    );
    client
        .get_item(GetItemInput {
            table_name: table_name.into(),
            key: key,
            ..Default::default()
        })
        .sync()
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::item::write::ensure_content;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::log::trace;

    #[test]
    fn get_item_by_address_test() {
        let log_context = "get_item_by_address_test";

        trace(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let content = content_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).expect("could not check table exists"));

        // ensure content
        assert!(ensure_content(&log_context, &local_client, &table_name, &content).is_ok());

        // TODO: get content
        // assert!(
        //     "{:?}",
        //     get_item_by_address(&local_client, &table_name, &content.address())
        // );
    }

}
