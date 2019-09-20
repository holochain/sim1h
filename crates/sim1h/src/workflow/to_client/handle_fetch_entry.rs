use crate::trace::LogContext;
use lib3h_protocol::data_types::FetchEntryData;
use crate::trace::tracer;

// -- Entry -- //
// Another node, or the network module itself is requesting data from us
// all entries are in the database
// no-op
pub fn handle_fetch_entry(log_context: &LogContext, fetch_entry_data: &FetchEntryData) {
    tracer(&log_context, &format!("handle_fetch_entry {:?}", fetch_entry_data));
}
