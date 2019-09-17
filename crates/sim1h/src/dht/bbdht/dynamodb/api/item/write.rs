use crate::dht::bbdht::dynamodb::client::Client;
use holochain_persistence_api::cas::content::AddressableContent;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DynamoDb;
use std::collections::HashMap;
// use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use rusoto_dynamodb::PutItemError;
use rusoto_dynamodb::PutItemInput;
use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::CONTENT_KEY;
use rusoto_dynamodb::PutItemOutput;

pub fn put_content(
    client: &Client,
    table_name: &str,
    content: &dyn AddressableContent,
) -> Result<PutItemOutput, RusotoError<PutItemError>> {
    // let mut item = PutItemInputAttributeMap::default();
    // item.insert(
    //     String::from(content.address()),
    //     val!(S => String::from(content.content())),
    // );
    let mut item = HashMap::new();
    item.insert(
        String::from(ADDRESS_KEY),
        string_attribute_value(&String::from(content.address())),
    );
    item.insert(
        String::from(CONTENT_KEY),
        string_attribute_value(&String::from(content.content())),
    );

    client
        .put_item(PutItemInput {
            item: item,
            table_name: table_name.to_string(),
            ..Default::default()
        })
        .sync()
}

#[cfg(test)]
pub mod tests {

    use crate::test::setup;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::item::write::put_content;

    #[test]
    fn put_content_test() {
        setup();

        info!("put_content_test fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let content = content_fresh();

        info!("put_content_test ensure cas table");
        assert!(ensure_cas_table(&local_client, &table_name).is_ok());

        info!("put_content_test check table exists");
        assert!(table_exists(&local_client, &table_name).expect("could not check table exists"));

        info!("put_content_test put content");
        assert!(put_content(&local_client, &table_name, &content).is_ok());
    }

}
