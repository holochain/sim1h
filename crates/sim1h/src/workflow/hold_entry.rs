use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hResult;
use lib3h_protocol::data_types::ProvidedEntryData;
use lib3h_protocol::protocol::ClientToLib3hResponse;

pub fn hold_entry(
    log_context: &LogContext,
    _client: &Client,
    _provided_entry_data: &ProvidedEntryData,
) -> Lib3hResult<ClientToLib3hResponse> {
    tracer(&log_context, "hold_entry");
    // TODO: this seems like a dumb response
    Ok(ClientToLib3hResponse::BootstrapSuccess)
}

#[cfg(test)]
pub mod tests {

    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;
    use crate::workflow::fixture::provided_entry_data_fresh;
    use crate::workflow::hold_entry::hold_entry;
    use crate::workflow::fixture::space_data_fresh;
    use lib3h_protocol::protocol::ClientToLib3hResponse;

    #[test]
    fn hold_entry_test() {
        let log_context = "hold_entry_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let space_data = space_data_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space_data);

        tracer(&log_context, "check response");
        match hold_entry(&log_context, &local_client, &provided_entry_data) {
            Ok(ClientToLib3hResponse::BootstrapSuccess) => {}
            Ok(o) => {
                panic!("bad ok {:?}", o);
            }
            Err(e) => {
                panic!("bad error {:?}", e);
            }
        }
    }

}
