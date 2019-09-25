use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::data_types::{EntryListData, FetchEntryData};
use lib3h_protocol::protocol::Lib3hToClient;
use lib3h_protocol::Address;

// result of no-op is no-op
pub fn handle_get_authoring_entry_list_result(
    log_context: &LogContext,
    entry_list_data: &EntryListData,
) {
    tracer(
        &log_context,
        &format!(
            "handle_get_authoring_entry_list_result {:?}",
            entry_list_data
        ),
    );

    // Fetch every entry that core is claiming to have authored:
    for (entry_address, aspect_addresses) in entry_list_data.address_map.iter() {
        Lib3hToClient::HandleFetchEntry(FetchEntryData{
            space_address: Address::new(),
            entry_address: entry_address.clone(),
            // When we get back the result as Lib3hToClientResponse::FetchEntryResult,
            // this will tell us that we should go ahead and publish the fetched entry:
            request_id: String::from("fetch-and-publish"),
            provider_agent_id: Address::new(),
            aspect_address_list: Some(aspect_addresses.clone())
        });
    }
}
