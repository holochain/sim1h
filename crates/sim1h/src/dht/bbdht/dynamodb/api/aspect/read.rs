use crate::trace::LogContext;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use holochain_persistence_api::cas::content::Address;
use rusoto_dynamodb::GetItemOutput;
use rusoto_core::RusotoError;
use rusoto_dynamodb::GetItemError;
use crate::trace::tracer;
use crate::dht::bbdht::dynamodb::api::item::read::get_item_by_address;

pub fn get_aspect(log_context: &LogContext, client: &Client, table_name: &TableName, aspect_address: &Address) -> Result<GetItemOutput, RusotoError<GetItemError>> {
    tracer(&log_context, "read_aspect");

    get_item_by_address(&log_context, &client, &table_name, &aspect_address)
}

#[cfg(test)]
pub mod tests {

    use crate::trace::tracer;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::aspect::read::get_aspect;
    use crate::workflow::fixture::entry_aspect_data_fresh;
    use crate::dht::bbdht::dynamodb::api::aspect::write::put_aspect;
    use holochain_persistence_api::cas::content::Address;

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
        match get_aspect(&log_context, &local_client, &table_name, &entry_aspect_data.aspect_address) {
            Ok(v) => {
                println!("{:#?}", v);
                assert_eq!(
                    Address::from(v.item.unwrap()["address"].clone().s.unwrap()),
                    entry_aspect_data.aspect_address
                );
            },
            Err(err) => {
                tracer(&log_context, "get_aspect Err");
                panic!("{:#?}", err);
            },
        }
    }

}
