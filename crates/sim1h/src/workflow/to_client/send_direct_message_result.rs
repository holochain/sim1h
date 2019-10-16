use crate::dht::bbdht::dynamodb::api::agent::inbox::send_to_agent_inbox;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::data_types::DirectMessageData;

// -- Direct Messaging -- //
// the response received from a previous `SendDirectMessage`
// B puts a message back to A
// works exactly the same as the original send
pub fn send_direct_message_result(
    log_context: &LogContext,
    space: &Space,
    direct_message_data: &DirectMessageData,
) -> BbDhtResult<()> {
    tracer(
        &log_context,
        &format!("send_direct_message_result {:?}", direct_message_data),
    );
    send_to_agent_inbox(
        &log_context,
        &space,
        &direct_message_data.request_id.clone().into(),
        &direct_message_data.from_agent_id.clone().into(),
        &direct_message_data.to_agent_id.clone().into(),
        &direct_message_data.content,
        true,
    )?;
    Ok(())
}
