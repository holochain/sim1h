use crate::trace::LogContext;
use lib3h_protocol::data_types::DisconnectedData;
use crate::trace::tracer;

// Notification of disconnection from a network
// MVP
// no-op
pub fn disconnected(log_context: &LogContext, disconnected_data: &DisconnectedData) {
    tracer(&log_context, &format!("disconnected {:?}", disconnected_data));
}
