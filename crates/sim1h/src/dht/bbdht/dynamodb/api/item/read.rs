use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::dht::bbdht::error::BbDhtResult;
use crate::trace::tracer;
use crate::trace::LogContext;
use rusoto_dynamodb::DynamoDb;
use crate::dht::bbdht::dynamodb::api::item::ItemKey;
use rusoto_dynamodb::GetItemInput;
use crate::space::Space;
use crate::dht::bbdht::dynamodb::api::item::keyed_item;

pub fn get_item_from_space(
    log_context: &LogContext,
    space: &Space,
    item_key: &ItemKey,
) -> BbDhtResult<Option<Item>> {
    tracer(&log_context, "get_item_from_space");

    let key = keyed_item(space, item_key);
    Ok(space.client
        .get_item(GetItemInput {
            consistent_read: Some(true),
            table_name: space.table_name.into(),
            key: key,
            ..Default::default()
        })
        .sync()?
        .item)
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::item::write::ensure_content;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;

    #[test]
    fn get_item_from_space_test() {
        let log_context = "get_item_from_space_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let content = content_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name)
            .expect("could not check table exists"));

        // ensure content
        assert!(ensure_content(&log_context, &local_client, &table_name, &content).is_ok());

        // TODO: get content
        // assert!(
        //     "{:?}",
        //     get_item_from_space(&local_client, &table_name, &content.address())
        // );
    }

}
