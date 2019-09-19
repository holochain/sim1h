use crate::dht::bbdht::dynamodb::api::item::read::get_item_by_address;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_persistence_api::cas::content::Address;
use rusoto_core::RusotoError;
use rusoto_dynamodb::GetItemError;
use rusoto_dynamodb::GetItemOutput;

pub fn get_aspect(
    log_context: &LogContext,
    client: &Client,
    table_name: &TableName,
    aspect_address: &Address,
) -> Result<GetItemOutput, RusotoError<GetItemError>> {
    tracer(&log_context, "read_aspect");

    get_item_by_address(&log_context, &client, &table_name, &aspect_address)
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::api::aspect::read::get_aspect;
    use crate::dht::bbdht::dynamodb::api::aspect::write::put_aspect;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::schema::cas::ADDRESS_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_ADDRESS_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_PUBLISH_TS_KEY;
    use crate::dht::bbdht::dynamodb::schema::cas::ASPECT_TYPE_HINT_KEY;
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
            Ok(v) => {
                println!("{:#?}", v);
                assert_eq!(
                    v.clone().item.unwrap()[ADDRESS_KEY].clone().s.unwrap(),
                    String::from(entry_aspect_data.aspect_address.clone()),
                );
                assert_eq!(
                    v.clone().item.unwrap()[ASPECT_ADDRESS_KEY]
                        .clone()
                        .s
                        .unwrap(),
                    String::from(entry_aspect_data.aspect_address.clone()),
                );
                assert_eq!(
                    v.clone().item.unwrap()[ASPECT_TYPE_HINT_KEY]
                        .clone()
                        .s
                        .unwrap(),
                    entry_aspect_data.type_hint,
                );
                assert_eq!(
                    v.clone().item.unwrap()[ASPECT_KEY].clone().b.unwrap(),
                    entry_aspect_data.aspect.as_slice(),
                );
                assert_eq!(
                    v.clone().item.unwrap()[ASPECT_PUBLISH_TS_KEY]
                        .clone()
                        .n
                        .unwrap(),
                    entry_aspect_data.publish_ts.to_string(),
                );
            }
            Err(err) => {
                tracer(&log_context, "get_aspect Err");
                panic!("{:#?}", err);
            }
        }
    }

}
