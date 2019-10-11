use crate::aspect::AspectAddress;
use crate::dht::bbdht::dynamodb::api::item::read::get_item_from_space;
use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_LIST_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_PUBLISH_TS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_TYPE_HINT_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ITEM_KEY;
use crate::dht::bbdht::error::BbDhtError;
use crate::dht::bbdht::error::BbDhtResult;
use crate::entry::EntryAddress;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use crate::workflow::state::AspectAddressMap;

use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::ScanInput;

use holochain_persistence_api::cas::content::Address;
use lib3h_protocol::data_types::EntryAspectData;
use std::collections::HashMap;
use std::fmt::Debug;

fn get_or_err<'a, V: Debug>(item: &'a HashMap<String, V>, key: &'a str) -> BbDhtResult<&'a V> {
    item.get(&key.to_string()).ok_or_else(|| {
        BbDhtError::MissingData(format!(
            "Key not present in hashmap! key={}, hashmap={:?}",
            key, item
        ))
    })
}

fn try_aspect_from_item(item: Item) -> BbDhtResult<EntryAspectData> {
    let aspect_address: AspectAddress = match get_or_err(&item, ASPECT_ADDRESS_KEY)?.s.clone() {
        Some(address) => address.into(),
        None => {
            return Err(BbDhtError::MissingData(format!(
                "Missing aspect_address: {:?}",
                item
            )))
        }
    };

    let aspect = match get_or_err(&item, ASPECT_KEY)?.b.clone() {
        Some(binary_data) => binary_data.to_vec().into(),
        None => {
            return Err(BbDhtError::MissingData(format!(
                "Missing aspect: {:?}",
                item
            )))
        }
    };

    let publish_ts = match get_or_err(&item, ASPECT_PUBLISH_TS_KEY)?.n.clone() {
        Some(publish_ts) => publish_ts.parse()?,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "Missing publish_ts: {:?}",
                item
            )))
        }
    };

    let type_hint = match get_or_err(&item, ASPECT_TYPE_HINT_KEY)?.s.clone() {
        Some(type_hint) => type_hint,
        None => {
            return Err(BbDhtError::MissingData(format!(
                "Missing type_hint: {:?}",
                item
            )))
        }
    };

    Ok(EntryAspectData {
        aspect_address: aspect_address.into(),
        aspect: aspect,
        publish_ts: publish_ts,
        type_hint: type_hint,
    })
}

pub fn try_aspect_list_from_item(item: Item) -> BbDhtResult<Vec<AspectAddress>> {
    let addresses = match get_or_err(&item, ASPECT_LIST_KEY)?.ss.clone() {
        Some(addresses) => addresses.iter().map(|s| s.into()).collect(),
        None => {
            return Err(BbDhtError::MissingData(format!(
                "Missing aspect_list: {:?}",
                item
            )))
        }
    };

    Ok(addresses)
}

pub fn get_aspect(
    log_context: &LogContext,
    space: &Space,
    aspect_address: &AspectAddress,
) -> BbDhtResult<Option<EntryAspectData>> {
    tracer(&log_context, "read_aspect");

    match get_item_from_space(&log_context, &space, &aspect_address.into()) {
        Ok(get_output) => match get_output {
            Some(aspect_item) => Ok(Some(try_aspect_from_item(aspect_item)?)),
            None => Ok(None),
        },
        Err(err) => Err(err.into()),
    }
}

pub fn get_entry_aspects(
    log_context: &LogContext,
    space: &Space,
    entry_address: &EntryAddress,
) -> BbDhtResult<Vec<EntryAspectData>> {
    match get_item_from_space(log_context, space, &entry_address.into()) {
        Ok(get_item_output) => match get_item_output {
            Some(item) => {
                let aspect_list = try_aspect_list_from_item(item)?;
                let mut aspects = Vec::new();
                for aspect_address in aspect_list {
                    aspects.push(
                        match get_aspect(log_context, space, &aspect_address.clone().into()) {
                            Ok(Some(aspect)) => aspect,
                            Ok(None) => {
                                return Err(BbDhtError::MissingData(format!(
                                    "Missing entry aspect data: {:?}",
                                    &aspect_address
                                )))
                            }
                            Err(err) => return Err(err),
                        },
                    )
                }
                Ok(aspects)
            }
            None => Ok(Vec::new()),
        },
        Err(err) => Err(err.into()),
    }
}

pub fn scan_aspects(
    log_context: &LogContext,
    space: &Space,
    exclusive_start_key: Option<Item>,
) -> BbDhtResult<(AspectAddressMap, Option<Item>)> {
    tracer(log_context, "scan_aspects");
    space
        .connection()
        .client()
        .scan(ScanInput {
            consistent_read: Some(true),
            table_name: space.connection().table_name().into(),
            projection_expression: projection_expression(vec![ITEM_KEY, ASPECT_LIST_KEY]),
            exclusive_start_key,
            ..Default::default()
        })
        .sync()
        .map_err(|err| err.into())
        .map(|result| {
            let items = result
                .items
                .unwrap_or(Vec::new())
                .into_iter()
                .filter_map(|mut item: Item| {
                    Some((
                        Address::from(item.remove(ITEM_KEY)?.s?),
                        item.remove(ASPECT_LIST_KEY)?
                            .ss?
                            .into_iter()
                            .map(Address::from)
                            .collect(),
                    ))
                })
                .collect();
            (items, result.last_evaluated_key)
        })
}

fn projection_expression(fields: Vec<&str>) -> Option<String> {
    Some(fields.join(", "))
}

#[cfg(test)]
pub mod tests {
    use crate::aspect::fixture::aspect_list_fresh;
    use crate::aspect::fixture::entry_aspect_data_fresh;
    use crate::dht::bbdht::dynamodb::api::aspect::read::get_aspect;
    use crate::dht::bbdht::dynamodb::api::aspect::read::get_entry_aspects;
    use crate::dht::bbdht::dynamodb::api::aspect::read::scan_aspects;
    use crate::dht::bbdht::dynamodb::api::aspect::write::append_aspect_list_to_entry;
    use crate::dht::bbdht::dynamodb::api::aspect::write::put_aspect;
    use crate::space::fixture::space_fresh;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::entry::fixture::entry_address_fresh;
    use crate::test::unordered_vec_compare;
    use crate::trace::tracer;
    use lib3h_protocol::data_types::EntryAspectData;

    #[test]
    fn get_entry_aspects_test() {
        let log_context = "get_entry_aspects_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_address = entry_address_fresh();
        let aspect_list = aspect_list_fresh();

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // empty aspect list
        match get_entry_aspects(&log_context, &space, &entry_address) {
            Ok(aspects) => {
                let expected: Vec<EntryAspectData> = Vec::new();
                assert_eq!(expected, aspects);
            }
            Err(err) => {
                panic!("found entry aspects before adding list {:?}", err);
            }
        }

        // put aspect list
        assert!(append_aspect_list_to_entry(
            &log_context,
            &space,
            &entry_address,
            &aspect_list
        )
        .is_ok());

        // get aspect list
        match get_entry_aspects(&log_context, &space, &entry_address) {
            Ok(aspects) => {
                assert!(unordered_vec_compare(aspect_list, aspects));
            }
            Err(err) => {
                panic!("no aspects found {:?}", err);
            }
        }
    }

    #[test]
    fn read_aspect_test() {
        let log_context = "read_aspect_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_aspect_data = entry_aspect_data_fresh();

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // put aspect
        assert!(put_aspect(&log_context, &space, &entry_aspect_data).is_ok());

        // get aspect
        match get_aspect(
            &log_context,
            &space,
            &entry_aspect_data.aspect_address.clone().into(),
        ) {
            Ok(Some(v)) => {
                println!("{:#?}", v);
                assert_eq!(v.aspect_address, entry_aspect_data.aspect_address,);
                assert_eq!(v.aspect_address, entry_aspect_data.aspect_address,);
                assert_eq!(v.type_hint, entry_aspect_data.type_hint,);
                assert_eq!(v.aspect, entry_aspect_data.aspect,);
                assert_eq!(v.publish_ts, entry_aspect_data.publish_ts,);
            }
            Ok(None) => {
                panic!("get_aspect None");
            }
            Err(err) => {
                tracer(&log_context, "get_aspect Err");
                panic!("{:#?}", err);
            }
        }
    }

    #[test]
    fn scan_aspects_test() {
        let log_context = "scan_aspects_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_address = entry_address_fresh();
        let aspect_list = aspect_list_fresh();
        let aspect_addresses = aspect_list
            .iter()
            .map(|a| a.aspect_address.clone())
            .collect();

        ensure_space(&log_context, &space).unwrap();

        {
            let (items, _) = scan_aspects(&log_context, &space, None)
                .unwrap_or_else(|err| panic!("error while scanning: {:?}", err));
            assert!(items.len() == 0);
        }

        append_aspect_list_to_entry(
            &log_context,
            &space,
            &entry_address,
            &aspect_list,
        )
        .unwrap();

        let (items, _) = scan_aspects(&log_context, &space, None)
            .unwrap_or_else(|err| panic!("error while scanning: {:?}", err));

        assert!(items.len() == 1);
        assert!(unordered_vec_compare(
            items[&entry_address.into()].clone().into_iter().collect(),
            aspect_addresses
        ));
    }
}
