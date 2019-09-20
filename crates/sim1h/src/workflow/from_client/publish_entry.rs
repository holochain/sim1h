use crate::dht::bbdht::dynamodb::api::aspect::write::append_aspect_list_to_entry;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hResult;
use lib3h_protocol::data_types::ProvidedEntryData;

pub fn publish_entry(
    log_context: &LogContext,
    client: &Client,
    provided_entry_data: &ProvidedEntryData,
) -> Lib3hResult<()> {
    tracer(&log_context, "publish_entry");

    append_aspect_list_to_entry(
        &log_context,
        &client,
        &provided_entry_data.space_address.to_string(),
        &provided_entry_data.entry.entry_address,
        &provided_entry_data.entry.aspect_list,
    )?;
    Ok(())
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::client::fixture::bad_client;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::entry::fixture::entry_address_fresh;
    use crate::space::fixture::space_data_fresh;
    use crate::trace::tracer;
    use crate::workflow::from_client::fixture::provided_entry_data_fresh;
    use crate::workflow::from_client::join_space::join_space;
    use crate::workflow::from_client::publish_entry::publish_entry;

    #[test]
    fn publish_entry_test() {
        let log_context = "publish_entry_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let space_data = space_data_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space_data, &entry_address);

        tracer(&log_context, "check response");

        assert!(join_space(&log_context, &local_client, &space_data).is_ok());

        match publish_entry(&log_context, &local_client, &provided_entry_data) {
            Ok(()) => {
                tracer(&log_context, "👌");
            }
            Err(err) => {
                tracer(&log_context, "publish_entry_test Err panic");
                panic!("{:?} {:?}", err, &provided_entry_data);
            }
        }
    }

    #[test]
    // publishing an entry before joining a space is an error
    fn publish_entry_no_join_test() {
        let log_context = "publish_entry_no_join_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let space_data = space_data_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space_data, &entry_address);

        tracer(&log_context, "check response");

        match publish_entry(&log_context, &local_client, &provided_entry_data) {
            Ok(v) => {
                panic!("bad Ok {:?}", v);
            }
            Err(_) => {
                tracer(&log_context, "👌");
            }
        }
    }

    #[test]
    fn publish_entry_bad_client_test() {
        let log_context = "publish_entry_bad_client_test";

        tracer(&log_context, "fixtures");
        let bad_client = bad_client();
        let space_data = space_data_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space_data, &entry_address);

        tracer(&log_context, "check response");
        match publish_entry(&log_context, &bad_client, &provided_entry_data) {
            Ok(result) => {
                panic!("test OK panic: {:?} {:?}", result, &provided_entry_data);
            }
            Err(_) => {
                tracer(&log_context, "👌");
            }
        }
    }

}
