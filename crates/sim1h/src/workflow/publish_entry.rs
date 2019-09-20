use crate::dht::bbdht::dynamodb::api::aspect::write::append_aspect_list_to_entry;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hError;
use lib3h::error::Lib3hResult;
use lib3h_protocol::data_types::ProvidedEntryData;
use lib3h_protocol::protocol::ClientToLib3hResponse;

pub fn publish_entry(
    log_context: &LogContext,
    client: &Client,
    provided_entry_data: &ProvidedEntryData,
) -> Lib3hResult<ClientToLib3hResponse> {
    tracer(&log_context, "publish_entry");

    match append_aspect_list_to_entry(
        &log_context,
        &client,
        &provided_entry_data.space_address.to_string(),
        &provided_entry_data.entry.entry_address,
        &provided_entry_data.entry.aspect_list,
    ) {
        Ok(_) => Ok(ClientToLib3hResponse::BootstrapSuccess),
        Err(err) => Err(Lib3hError::from(err.to_string())),
    }
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::client::fixture::bad_client;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;
    use crate::workflow::fixture::entry_address_fresh;
    use crate::workflow::fixture::provided_entry_data_fresh;
    use crate::workflow::fixture::space_data_fresh;
    use crate::workflow::join_space::join_space;
    use crate::workflow::publish_entry::publish_entry;
    use lib3h_protocol::protocol::ClientToLib3hResponse;

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
            Ok(ClientToLib3hResponse::BootstrapSuccess) => {}
            Ok(result) => {
                panic!("test OK panic: {:?} {:?}", result, &provided_entry_data);
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
