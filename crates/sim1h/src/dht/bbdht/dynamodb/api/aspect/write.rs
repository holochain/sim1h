use crate::dht::bbdht::dynamodb::api::item::keyed_item;
use crate::dht::bbdht::dynamodb::api::item::partition_key;
use crate::dht::bbdht::dynamodb::api::item::write::should_put_item_retry;
use crate::dht::bbdht::dynamodb::schema::blob_attribute_value;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_LIST_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_PUBLISH_TS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_TYPE_HINT_KEY;
use crate::dht::bbdht::dynamodb::schema::number_attribute_value;
use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
use crate::dht::bbdht::dynamodb::schema::string_set_attribute_value;
use crate::dht::bbdht::error::BbDhtResult;
use crate::entry::EntryAddress;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::data_types::EntryAspectData;
use rusoto_dynamodb::AttributeValue;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::PutItemInput;
use rusoto_dynamodb::UpdateItemInput;
use std::collections::HashMap;

pub fn aspect_list_to_attribute(
    space: &Space,
    aspect_list: &Vec<EntryAspectData>,
) -> AttributeValue {
    string_set_attribute_value(
        aspect_list
            .iter()
            .map(|aspect| partition_key(space, &aspect.aspect_address.clone().into()))
            .collect(),
    )
}

pub fn put_aspect(
    log_context: &LogContext,
    space: &Space,
    aspect: &EntryAspectData,
) -> BbDhtResult<()> {
    tracer(&log_context, "put_aspect");

    let mut item = keyed_item(space, &aspect.into());

    item.insert(
        String::from(ASPECT_ADDRESS_KEY),
        string_attribute_value(&aspect.aspect_address.to_string()),
    );

    item.insert(
        String::from(ASPECT_TYPE_HINT_KEY),
        string_attribute_value(&aspect.type_hint),
    );

    item.insert(
        String::from(ASPECT_KEY),
        blob_attribute_value(&aspect.aspect),
    );

    item.insert(
        String::from(ASPECT_PUBLISH_TS_KEY),
        number_attribute_value(&aspect.publish_ts),
    );

    if should_put_item_retry(
        log_context,
        space
            .connection()
            .client()
            .put_item(PutItemInput {
                table_name: space.connection().table_name().into(),
                item: item,
                ..Default::default()
            })
            .sync(),
    )? {
        put_aspect(log_context, space, aspect)
    } else {
        Ok(())
    }
}

pub fn append_aspect_list_to_entry(
    log_context: &LogContext,
    space: &Space,
    entry_address: &EntryAddress,
    aspect_list: &Vec<EntryAspectData>,
) -> BbDhtResult<()> {
    tracer(&log_context, "append_aspects");

    // need to append all the aspects before making them discoverable under the entry
    for aspect in aspect_list {
        put_aspect(log_context, space, &aspect)?;
    }

    // the aspect addressses live under the entry address
    let aspect_addresses_key = keyed_item(space, &entry_address.into());

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":aspects".to_string(),
        aspect_list_to_attribute(space, aspect_list),
    );

    let mut expression_attribute_names = HashMap::new();
    expression_attribute_names.insert("#aspect_list".to_string(), ASPECT_LIST_KEY.to_string());

    // https://stackoverflow.com/questions/31288085/how-to-append-a-value-to-list-attribute-on-aws-dynamodb
    let update_expression = "ADD #aspect_list :aspects";

    space
        .connection()
        .client()
        .update_item(UpdateItemInput {
            table_name: space.connection().table_name().into(),
            key: aspect_addresses_key,
            update_expression: Some(update_expression.to_string()),
            expression_attribute_names: Some(expression_attribute_names),
            expression_attribute_values: Some(expression_attribute_values),
            ..Default::default()
        })
        .sync()?;

    Ok(())
}

#[cfg(test)]
pub mod tests {

    use crate::aspect::fixture::aspect_list_fresh;
    use crate::aspect::fixture::entry_aspect_data_fresh;
    use crate::dht::bbdht::dynamodb::api::aspect::write::append_aspect_list_to_entry;
    use crate::dht::bbdht::dynamodb::api::aspect::write::aspect_list_to_attribute;
    use crate::dht::bbdht::dynamodb::api::aspect::write::put_aspect;
    use crate::dht::bbdht::dynamodb::api::item::partition_key;
    use crate::dht::bbdht::dynamodb::api::item::read::get_item_from_space;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::dht::bbdht::dynamodb::api::space::exist::space_exists;
    use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_LIST_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::PARTITION_KEY;
    use crate::dht::bbdht::dynamodb::schema::string_attribute_value;
    use crate::entry::fixture::entry_address_fresh;
    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;
    use std::collections::HashMap;

    #[test]
    fn put_aspect_test() {
        let log_context = "put_aspect_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_aspect = entry_aspect_data_fresh();

        // ensure space
        assert!(ensure_space(&log_context, &space).is_ok());

        // space exists
        assert!(space_exists(&log_context, &space).is_ok());

        // put aspect
        assert!(put_aspect(&log_context, &space, &entry_aspect).is_ok());
    }

    #[test]
    fn append_aspects_to_entry_test() {
        let log_context = "append_aspects_to_entry_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_address = entry_address_fresh();
        let aspect_list = aspect_list_fresh();

        let mut expected = HashMap::new();
        expected.insert(
            ASPECT_LIST_KEY.to_string(),
            aspect_list_to_attribute(&space, &aspect_list),
        );
        expected.insert(
            PARTITION_KEY.to_string(),
            string_attribute_value(&partition_key(&space, &entry_address.clone().into())),
        );

        // ensure space
        assert!(ensure_space(&log_context, &space).is_ok());

        // space exists
        assert!(space_exists(&log_context, &space).is_ok());

        // trash/idempotency loop
        for _ in 0..3 {
            // append aspects
            assert!(append_aspect_list_to_entry(
                &log_context,
                &space,
                &entry_address,
                &aspect_list
            )
            .is_ok());

            // get matches
            match get_item_from_space(&log_context, &space, &entry_address.clone().into()) {
                Ok(get_item_output) => match get_item_output {
                    Some(item) => {
                        assert_eq!(expected[PARTITION_KEY], item[PARTITION_KEY],);
                        assert_eq!(
                            expected[ASPECT_LIST_KEY].ss.iter().count(),
                            item[ASPECT_LIST_KEY].ss.iter().count(),
                        );
                    }
                    None => {
                        tracer(&log_context, "get matches None");
                        panic!("None");
                    }
                },
                Err(err) => {
                    tracer(&log_context, "get matches err");
                    panic!("{:?}", err);
                }
            }
        }
    }
}
