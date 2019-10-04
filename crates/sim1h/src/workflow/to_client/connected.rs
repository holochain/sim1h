use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::data_types::ConnectedData;

/// -- Connection -- //
/// Notification of successful connection to a network
/// no-op
pub fn connected(log_context: &LogContext, _space: &Space, connected_data: &ConnectedData) {
    tracer(&log_context, &format!("connected {:?}", connected_data));
}
