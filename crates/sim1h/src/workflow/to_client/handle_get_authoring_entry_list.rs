use crate::trace::LogContext;
use lib3h_protocol::data_types::GetListData;
use crate::trace::tracer;

// -- Entry lists -- //
// MVP
// database stored everything
// no-op
pub fn handle_get_authoring_entry_list(log_context: &LogContext, get_list_data: &GetListData) {
    tracer(&log_context, &format!("handle_get_gossiping_entry_list {:?}", get_list_data));
}
