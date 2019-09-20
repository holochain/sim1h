use crate::trace::LogContext;
use lib3h_protocol::data_types::FetchEntryResultData;
use crate::trace::tracer;

/// Successful data response for a `HandleFetchEntryData` request
/// result of no-op is no-op
pub fn handle_fetch_entry_result(log_context: &LogContext, fetch_entry_result_data: &FetchEntryResultData) {
    tracer(&log_context, &format!("handle_fetch_entry_result {:?}", fetch_entry_result_data));
}
