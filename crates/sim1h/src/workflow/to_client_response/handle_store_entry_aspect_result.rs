use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;

// result of no-op is no-op
pub fn handle_store_entry_aspect_result(log_context: &LogContext, _space: &Space) {
    tracer(&log_context, &format!("handle_store_entry_aspect_result"));
    // TODO: update held_aspects. But, need the protocol message to tell us which aspect was held!
}
