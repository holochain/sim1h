use crate::trace::LogContext;
use lib3h_protocol::data_types::DirectMessageData;
use crate::trace::tracer;

// -- Direct Messaging -- //
// the response received from a previous `SendDirectMessage`
// ?? dubious ??
// B has put a result in A inbox
// A queries inbox
// A records seen
pub fn send_direct_message_result(log_context: &LogContext, direct_message_data: &DirectMessageData) {
    tracer(&log_context, &format!("send_direct_message_result {:?}", direct_message_data));
}
