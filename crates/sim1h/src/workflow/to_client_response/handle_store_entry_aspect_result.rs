use crate::trace::LogContext;
use crate::trace::tracer;

// result of no-op is no-op
pub fn handle_store_entry_aspect_result(log_context: &LogContext) {
    tracer(&log_context, &format!("handle_store_entry_aspect_result"));
}
