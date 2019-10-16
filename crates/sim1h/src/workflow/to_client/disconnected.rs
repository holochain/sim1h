use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::data_types::DisconnectedData;

// Notification of disconnection from a network
// no-op
pub fn disconnected(
    log_context: &LogContext,
    _space: &Space,
    disconnected_data: &DisconnectedData,
) {
    tracer(
        &log_context,
        &format!("disconnected {:?}", disconnected_data),
    );
}
