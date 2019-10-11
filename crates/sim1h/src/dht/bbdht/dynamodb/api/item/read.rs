use crate::dht::bbdht::dynamodb::api::item::keyed_item;
use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::dht::bbdht::dynamodb::api::item::ItemKey;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::GetItemInput;

pub fn get_item_from_space(
    log_context: &LogContext,
    space: &Space,
    item_key: &ItemKey,
) -> BbDhtResult<Option<Item>> {
    tracer(&log_context, "get_item_from_space");

    let key = keyed_item(space, item_key);
    Ok(space
        .connection()
        .client()
        .get_item(GetItemInput {
            consistent_read: Some(true),
            table_name: space.connection().table_name().into(),
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
    use crate::space::fixture::space_fresh;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::dht::bbdht::dynamodb::api::space::exist::space_exists;
    use crate::trace::tracer;

    #[test]
    fn get_item_from_space_test() {
        let log_context = "get_item_from_space_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let content = content_fresh();

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // cas exists
        assert!(space_exists(&log_context, &space).is_ok());

        // ensure content
        assert!(ensure_content(&log_context, &space, &content).is_ok());

        // TODO: get content
        // assert!(
        //     "{:?}",
        //     get_item_from_space(&local_client, &table_name, &content.address())
        // );
    }

}
