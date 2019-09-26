use crate::trace::tracer;
use crate::trace::LogContext;
use crate::workflow::state::Sim1hState;
use lib3h_protocol::data_types::EntryListData;

impl Sim1hState {
    pub fn handle_get_gossiping_entry_list_result(
        &mut self,
        log_context: &LogContext,
        entry_list_data: &EntryListData,
    ) {
        tracer(
            &log_context,
            &format!(
                "handle_get_gossiping_entry_list_result {:?}",
                entry_list_data
            ),
        );
        self.entries_held = entry_list_data.address_map.clone();
    }
}
