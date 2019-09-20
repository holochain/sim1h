use crate::trace::LogContext;
use lib3h_protocol::data_types::EntryListData;
use crate::trace::tracer;

// result of no-op is no-op
pub fn handle_get_gossiping_entry_list_result(log_context: &LogContext, entry_list_data: &EntryListData) {
    tracer(&log_context, &format!("handle_get_gossiping_entry_list_result {:?}", entry_list_data));
}
