use crate::trace::LogContext;
use lib3h_protocol::data_types::ConnectedData;
use crate::trace::tracer;

/// -- Connection -- //
/// Notification of successful connection to a network
/// MVP
/// no-op
pub fn connected(log_context: &LogContext, connected_data: &ConnectedData) {
    tracer(&log_context, &format!("connected {:?}", connected_data));
}
