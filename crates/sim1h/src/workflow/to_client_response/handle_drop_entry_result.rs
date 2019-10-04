use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;

// result of no-op is no-op
pub fn handle_drop_entry_result(log_context: &LogContext, _space: &Space) {
    tracer(&log_context, &format!("handle_drop_entry_result"));
}
