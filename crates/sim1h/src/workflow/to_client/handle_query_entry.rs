use crate::trace::LogContext;
use lib3h_protocol::data_types::QueryEntryData;
use crate::trace::tracer;
use crate::dht::bbdht::dynamodb::client::Client;

// Request a node to handle a QueryEntry request
// queries are simulated on the outgoing side
// no-op
pub fn handle_query_entry(log_context: &LogContext, _client: &Client, query_entry_data: &QueryEntryData) {
    tracer(&log_context, &format!("handle_query_entry {:?}", query_entry_data));
}
