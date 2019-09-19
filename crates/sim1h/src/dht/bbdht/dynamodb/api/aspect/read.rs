use crate::dht::bbdht::dynamodb::api::item::read::get_item_by_address;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;
use crate::dht::bbdht::error::BbDhtResult;
use crate::dht::bbdht::error::BbDhtError;
use lib3h_protocol::data_types::EntryAspectData;
use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_ADDRESS_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_TYPE_HINT_KEY;
use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_PUBLISH_TS_KEY;

fn try_aspect_from_item(item: HashMap<String, AttributeValue>) -> BbDhtResult<EntryAspectData> {
    let aspect_address = match item[ASPECT_ADDRESS_KEY].s.clone() {
        Some(address) => Address::from(address),
        None => return Err(BbDhtError::MissingData(format!("Missing aspect_address: {:?}", item))),
    };

    let aspect = match item[ASPECT_KEY].b.clone() {
        Some(binary_data) => binary_data.to_vec().into(),
        None => return Err(BbDhtError::MissingData(format!("Missing aspect: {:?}", item))),
    };

    let publish_ts = match item[ASPECT_PUBLISH_TS_KEY].n.clone() {
        Some(publish_ts) => publish_ts.parse()?,
        None => return Err(BbDhtError::MissingData(format!("Missing publish_ts: {:?}", item))),
    };

    let type_hint = match item[ASPECT_TYPE_HINT_KEY].s.clone() {
        Some(type_hint) => type_hint,
        None => return Err(BbDhtError::MissingData(format!("Missing type_hint: {:?}", item))),
    };

    Ok(EntryAspectData {
        aspect_address: aspect_address,
        aspect: aspect,
        publish_ts: publish_ts,
        type_hint: type_hint,
    })
}

pub fn get_aspect(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    aspect_address: &Address,
) -> BbDhtResult<Option<EntryAspectData>> {
    tracer(&log_context, "read_aspect");

    match get_item_by_address(&log_context, &client, &table_name, &aspect_address) {
        Ok(get_output) => {
            match get_output.item {
                Some(aspect_item) => Ok(Some(try_aspect_from_item(aspect_item)?)),
                None => Ok(None),
            }
        },
        Err(err) => Err(err.into()),
    }
}

pub fn get_entry_aspects (
    _log_context: &LogContext,
    _client: &Client,
    _table_name: &TableName,
    _entry_address: &Address,
// ) -> BbDhtResult<Vec<EntryAspectData>> {
) {
    // get_item_by_address(log_context, client, table_name, entry_address);
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::api::aspect::read::get_aspect;
    use crate::dht::bbdht::dynamodb::api::aspect::write::put_aspect;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;
    use crate::workflow::fixture::entry_aspect_data_fresh;

    #[test]
    fn read_aspect_test() {
        let log_context = "read_aspect_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let entry_aspect_data = entry_aspect_data_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).is_ok());

        // put aspect
        assert!(put_aspect(&log_context, &local_client, &table_name, &entry_aspect_data).is_ok());

        // get aspect
        match get_aspect(
            &log_context,
            &local_client,
            &table_name,
            &entry_aspect_data.aspect_address,
        ) {
            Ok(Some(v)) => {
                println!("{:#?}", v);
                assert_eq!(
                    v.aspect_address,
                    entry_aspect_data.aspect_address,
                );
                assert_eq!(
                    v.aspect_address,
                    entry_aspect_data.aspect_address,
                );
                assert_eq!(
                    v.type_hint,
                    entry_aspect_data.type_hint,
                );
                assert_eq!(
                    v.aspect,
                    entry_aspect_data.aspect,
                );
                assert_eq!(
                    v.publish_ts,
                    entry_aspect_data.publish_ts,
                );
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

}
