use crate::trace::LogContext;
use lib3h_protocol::data_types::GetListData;
use crate::trace::tracer;
use crate::dht::bbdht::dynamodb::client::Client;

/// -- Entry lists -- //
/// database stored everything
/// no-op
pub fn handle_get_gossiping_entry_list(log_context: &LogContext, _client: &Client, get_list_data: &GetListData) {
    tracer(&log_context, &format!("handle_get_gossiping_entry_list {:?}", get_list_data));
}
