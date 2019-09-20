use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hResult;
use lib3h_protocol::data_types::ProvidedEntryData;

pub fn hold_entry(
    log_context: &LogContext,
    _client: &Client,
    _provided_entry_data: &ProvidedEntryData,
) -> Lib3hResult<()> {
    tracer(&log_context, "hold_entry");

    // it is valid for the provided_agent_id to not have joined the network
    // it is a remote client and they may be "offline"

    Ok(())
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;
    use crate::entry::fixture::entry_address_fresh;
    use crate::workflow::fixture::provided_entry_data_fresh;
    use crate::space::fixture::space_data_fresh;
    use crate::workflow::hold_entry::hold_entry;

    #[test]
    fn hold_entry_test() {
        let log_context = "hold_entry_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let space_data = space_data_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space_data, &entry_address);

        tracer(&log_context, "check response");
        match hold_entry(&log_context, &local_client, &provided_entry_data) {
            Ok(()) => {}
            Err(e) => {
                panic!("bad error {:?}", e);
            }
        }
    }

    #[test]
    fn hold_entry_no_join_test() {
        let log_context = "hold_entry_no_join_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let space_data = space_data_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space_data, &entry_address);

        tracer(&log_context, "check response");
        match hold_entry(&log_context, &local_client, &provided_entry_data) {
            Ok(()) => {
                tracer(&log_context, "👌");
            }
            Err(e) => {
                panic!("bad error {:?}", e);
            }
        }
    }

}
