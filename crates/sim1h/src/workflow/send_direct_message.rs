use crate::dht::bbdht::dynamodb::api::item::write::append_agent_message;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hResult;
use lib3h_protocol::data_types::DirectMessageData;
use lib3h_protocol::protocol::ClientToLib3hResponse;

pub fn send_direct_message(
    log_context: &LogContext,
    client: &Client,
    direct_message_data: &DirectMessageData,
) -> Lib3hResult<ClientToLib3hResponse> {
    tracer(&log_context, "send_direct_message");
    match append_agent_message(
        &log_context,
        &client,
        &direct_message_data.space_address.to_string(),
        &direct_message_data.request_id,
        &direct_message_data.from_agent_id,
        &direct_message_data.to_agent_id,
        &direct_message_data.content,
    ) {
        _ => Ok(ClientToLib3hResponse::SendDirectMessageResult(
            direct_message_data.clone(),
        )),
    }
}