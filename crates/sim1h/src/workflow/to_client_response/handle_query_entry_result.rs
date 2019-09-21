use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::data_types::QueryEntryResultData;

/// Response to a `HandleQueryEntry` request
/// result of no-op is no-op
pub fn handle_query_entry_result(
    log_context: &LogContext,
    query_entry_result_data: &QueryEntryResultData,
) {
    tracer(
        &log_context,
        &format!("handle_query_entry_result {:?}", query_entry_result_data),
    );
}
