use lib3h_protocol::protocol::Lib3hToClient;
use std::sync::atomic::{AtomicBool, Ordering};
use lib3h_protocol::Address;
use lib3h_protocol::data_types::GetListData;
use parking_lot::Mutex;

pub mod connected;
pub mod disconnected;
pub mod handle_drop_entry;
pub mod handle_fetch_entry;
pub mod handle_get_authoring_entry_list;
pub mod handle_get_gossiping_entry_list;
pub mod handle_query_entry;
pub mod handle_send_direct_message;
pub mod handle_store_entry_aspect;
pub mod send_direct_message_result;

lazy_static! {
    pub static ref SIM1H_INITIALIZED: AtomicBool = AtomicBool::new(false);
    pub static ref SPACE_ADDRESS: Mutex<Option<Address>> = Mutex::new(None);
    pub static ref AGENT_ID: Mutex<Option<Address>> = Mutex::new(None);
}

fn should_get_authoring_list() -> bool {
    SIM1H_INITIALIZED.load(Ordering::Relaxed) == false &&
        SPACE_ADDRESS.lock().is_some() &&
        AGENT_ID.lock().is_some()
}

pub fn process_pending_requests_to_client() -> Vec<Lib3hToClient> {
    let mut requests = Vec::new();
    if should_get_authoring_list() {
        requests.push(Lib3hToClient::HandleGetAuthoringEntryList(GetListData{
            space_address: SPACE_ADDRESS.lock().clone().expect("Must be some because of if-condition"),
            provider_agent_id: AGENT_ID.lock().clone().expect("Must be some because of if-condition"),
            request_id: "".into(),
        }));
        requests.push(Lib3hToClient::HandleGetGossipingEntryList(GetListData{
            space_address: SPACE_ADDRESS.lock().clone().expect("Must be some because of if-condition"),
            provider_agent_id: AGENT_ID.lock().clone().expect("Must be some because of if-condition"),
            request_id: "".into(),
        }));

        SIM1H_INITIALIZED.store(true, Ordering::Relaxed);
    };

    requests
}