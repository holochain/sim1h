use crate::dht::bbdht::dynamodb::api::aspect::write::append_aspect_list;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hResult;
use lib3h::error::Lib3hError;
use lib3h_protocol::data_types::ProvidedEntryData;
use lib3h_protocol::protocol::ClientToLib3hResponse;

pub fn publish_entry(
    log_context: &LogContext,
    client: &Client,
    provided_entry_data: &ProvidedEntryData,
) -> Lib3hResult<ClientToLib3hResponse> {
    tracer(&log_context, "publish_entry");

    match append_aspect_list(
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
