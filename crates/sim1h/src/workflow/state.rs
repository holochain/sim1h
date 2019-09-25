use lib3h_protocol::data_types::GetListData;

use lib3h_protocol::protocol::Lib3hToClient;
use lib3h_protocol::Address;

#[derive(Default)]
pub struct Sim1hState {
    pub initialized: bool,
    pub space_address: Option<Address>,
    pub agent_id: Option<Address>,
    pub client_outbox: Vec<Lib3hToClient>,
}

impl Sim1hState {
    fn should_get_authoring_list(&mut self) -> bool {
        self.initialized == false && self.space_address.is_some() && self.agent_id.is_some()
    }

    pub fn process_pending_requests_to_client(&mut self) -> Vec<Lib3hToClient> {
        let mut requests = Vec::new();
        if self.should_get_authoring_list() {
            requests.push(Lib3hToClient::HandleGetAuthoringEntryList(GetListData {
                space_address: self
                    .space_address
                    .clone()
                    .expect("Must be some because of if-condition"),
                provider_agent_id: self
                    .agent_id
                    .clone()
                    .expect("Must be some because of if-condition"),
                request_id: "".into(),
            }));
            requests.push(Lib3hToClient::HandleGetGossipingEntryList(GetListData {
                space_address: self
                    .space_address
                    .clone()
                    .expect("Must be some because of if-condition"),
                provider_agent_id: self
                    .agent_id
                    .clone()
                    .expect("Must be some because of if-condition"),
                request_id: "".into(),
            }));

            self.initialized = true;
        };

        requests
            .into_iter()
            .chain(self.client_outbox.drain(..))
            .collect()
    }
}
