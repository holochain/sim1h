use crate::dht::bbdht::dynamodb::api::item::keyed_item;
use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::dht::bbdht::dynamodb::schema::cas::CONTENT_KEY;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::AddressableContent;
use rusoto_core::RusotoError;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::PutItemError;
use rusoto_dynamodb::PutItemInput;
use rusoto_dynamodb::PutItemOutput;

pub fn content_to_item(space: &Space, content: &dyn AddressableContent) -> Item {
    let mut item = keyed_item(space, &content.address().into());
    item.insert(
        String::from(CONTENT_KEY),
        string_attribute_value(&content.content().into()),
    );
    item
}

pub fn should_put_item_retry(
    log_context: &LogContext,
    put_item_result: Result<PutItemOutput, RusotoError<PutItemError>>,
) -> BbDhtResult<bool> {
    match put_item_result {
        // no need to retry any success
        Ok(_) => Ok(false),
        Err(RusotoError::Service(err)) => match err {
            PutItemError::InternalServerError(err) => {
                // retry InternalServerErrors as these often seem to be temporary
                tracer(
                    &log_context,
                    &format!("retry Service InternalServerError {:?}", err),
                );
                Ok(true)
            }
            PutItemError::ProvisionedThroughputExceeded(err) => {
                // retry throughput issues as these will hopefully recover
                tracer(
                    &log_context,
                    &format!("retry Service ProvisionedThroughputExceeded {:?}", err),
                );
                Ok(true)
            }
            PutItemError::RequestLimitExceeded(err) => {
                // retry request limits as these will hopefully recover
                tracer(
                    &log_context,
                    &format!("retry put_aspect Service RequestLimitExceeded {:?}", err),
                );
                Ok(true)
            }
            PutItemError::TransactionConflict(err) => {
                // retry transaction conflicts
                tracer(
                    &log_context,
                    &format!("retry Service TransactionConflict {:?}", err),
                );
                Ok(true)
            }
            // forward other put item errors back up the stack
            _ => Err(err.into()),
        },
        Err(RusotoError::Unknown(err)) => {
            // retry anything we don't know about
            tracer(&log_context, &format!("retry Unknown {:?}", err));
            Ok(true)
        }
        // forward other errors back up the stack
        Err(err) => Err(err.into()),
    }
}

pub fn ensure_content(
    log_context: &LogContext,
    space: &Space,
    content: &dyn AddressableContent,
) -> BbDhtResult<()> {
    tracer(&log_context, "ensure_content");

    if should_put_item_retry(
        log_context,
        space
            .connection()
            .client()
            .put_item(PutItemInput {
                item: content_to_item(space, content),
                table_name: space.connection().table_name().into(),
                ..Default::default()
            })
            .sync(),
    )? {
        ensure_content(log_context, space, content)
    } else {
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::api::item::fixture::content_fresh;
    use crate::dht::bbdht::dynamodb::api::item::write::ensure_content;
    use crate::dht::bbdht::dynamodb::api::space::exist::space_exists;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;

    #[test]
    fn ensure_content_test() {
        let log_context = "ensure_content_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let content = content_fresh();

        // ensure space
        assert!(ensure_space(&log_context, &space).is_ok());

        // space exists
        assert!(space_exists(&log_context, &space).is_ok());

        // ensure content
        assert!(ensure_content(&log_context, &space, &content).is_ok());

        // thrash a bit
        for _ in 0..100 {
            assert!(ensure_content(&log_context, &space, &content).is_ok());
        }
    }

}
