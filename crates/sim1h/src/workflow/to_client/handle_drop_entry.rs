use crate::trace::LogContext;
use lib3h_protocol::data_types::DropEntryData;
use crate::trace::tracer;

/// Local client does not need to hold that entry anymore.
/// Local client doesn't 'have to' comply.
/// all entries are in the database
/// no-op
pub fn handle_drop_entry(log_context: &LogContext, drop_entry_data: &DropEntryData) {
    tracer(&log_context, &format!("handle_drop_entry {:?}", drop_entry_data));
}
