use crate::dht::bbdht::dynamodb::api::agent::inbox::check_inbox;
use crate::dht::bbdht::dynamodb::api::aspect::read::get_aspect;
use crate::dht::bbdht::dynamodb::api::aspect::read::scan_aspects;
use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::dht::bbdht::dynamodb::client::Client;
use lib3h_protocol::data_types::GetListData;
use lib3h_protocol::data_types::StoreEntryAspectData;
use lib3h_protocol::protocol::ClientToLib3hResponse;
use lib3h_protocol::protocol::Lib3hToClient;
use lib3h_protocol::Address;
use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

pub type AspectAddressMap = HashMap<Address, HashSet<Address>>;
type Sim1hResult<T> = Result<T, String>;

const TICKS_PER_BIG_FETCH: u32 = 100;

#[derive(Default)]
pub struct Sim1hState {
    pub initialized: bool,
    pub space_address: Option<Address>,
    pub agent_id: Option<Address>,
    pub client_request_outbox: Vec<Lib3hToClient>,
    pub client_response_outbox: Vec<ClientToLib3hResponse>,
    pub held_aspects: AspectAddressMap,
    num_ticks: u32,
    last_evaluated_scan_key: Option<Item>,
}

impl Sim1hState {
    fn should_get_authoring_list(&mut self) -> bool {
        self.initialized == false && self.space_address.is_some() && self.agent_id.is_some()
    }

    pub fn space_address(&self) -> Sim1hResult<&Address> {
        self.space_address
            .as_ref()
            .ok_or("Cannot get space_address: Sim1hState is not initialized".to_string())
    }

    pub fn agent_id(&self) -> Sim1hResult<&Address> {
        self.agent_id
            .as_ref()
            .ok_or("Cannot get agent_id: Sim1hState is not initialized".to_string())
    }

    fn create_authoring_gossip_list_requests(&self) -> Vec<Lib3hToClient> {
        let mut requests = Vec::new();
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

        requests
    }

    fn create_direct_message_requests(&self, client: &Client) -> Vec<Lib3hToClient> {
        if self.space_address.is_some() && self.agent_id.is_some() {
            let log_context = "Sim1hState::create_direct_message_requests";
            match check_inbox(
                &log_context,
                client,
                &self
                    .space_address
                    .clone()
                    .expect("Must be some because of if-condition")
                    .to_string(),
                &Address::from(
                    self.agent_id
                        .clone()
                        .expect("Must be some because of if-condition")
                        .to_string(),
                ),
            ) {
                Ok(direct_messages) => direct_messages
                    .into_iter()
                    .map(|(message, is_response)| {
                        if is_response {
                            Lib3hToClient::SendDirectMessageResult(message)
                        } else {
                            Lib3hToClient::HandleSendDirectMessage(message)
                        }
                    })
                    .collect(),
                Err(error) => {
                    error!("Error checking inbox: {:?}", error);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    }

    fn create_store_requests(&mut self, client: &Client) -> Sim1hResult<Vec<Lib3hToClient>> {
        self.num_ticks += 1;
        if !self.initialized || self.num_ticks % TICKS_PER_BIG_FETCH > 0 {
            return Ok(Vec::new());
        }
        let log_context = "create_store_requests";
        let agent_id = self.agent_id()?.clone();
        let space_address = self.space_address()?.clone();
        let table_name = space_address.to_string();
        let (incoming, last_evaluated_key) = scan_aspects(
            log_context,
            client,
            &table_name,
            self.last_evaluated_scan_key.clone(),
        )
        .map_err(|err| err.to_string())?;
        self.last_evaluated_scan_key = last_evaluated_key;
        let mut messages = Vec::new();

        for entry_address in incoming.keys() {
            let aspects = incoming[entry_address].clone();
            let diff = match self.held_aspects.entry(entry_address.clone()) {
                Vacant(e) => {
                    e.insert(aspects.clone());
                    aspects.into_iter().collect()
                }
                Occupied(mut e) => {
                    let old = e.insert(aspects.clone());
                    aspects.difference(&old).cloned().collect::<Vec<_>>()
                }
            };
            messages.append(
                &mut diff
                    .into_iter()
                    .filter_map(|aspect_address| {
                        get_aspect(&log_context, client, &table_name, &aspect_address)
                            .expect("Cannot get aspect")
                    })
                    .map(|entry_aspect| {
                        Lib3hToClient::HandleStoreEntryAspect(StoreEntryAspectData {
                            request_id: Uuid::new_v4().to_string(), // XXX: well, is this so bad?
                            space_address: space_address.clone(),
                            provider_agent_id: agent_id.clone(), // TODO: is this OK?
                            entry_address: entry_address.clone(),
                            entry_aspect,
                        })
                    })
                    .collect(),
            );
        }

        Ok(messages)
    }

    pub fn process_pending_requests_to_client(
        &mut self,
        client: &Client,
    ) -> Sim1hResult<Vec<Lib3hToClient>> {
        let requests = if self.should_get_authoring_list() {
            self.initialized = true;
            self.create_authoring_gossip_list_requests()
        } else {
            Vec::new()
        };

        let messages = requests
            .into_iter()
            .chain(self.create_store_requests(client)?.into_iter())
            .chain(self.create_direct_message_requests(client).into_iter())
            .chain(self.client_request_outbox.drain(..))
            .collect();

        Ok(messages)
    }

    pub fn process_pending_responses_to_client(&mut self) -> Vec<ClientToLib3hResponse> {
        self.client_response_outbox.drain(..).collect()
    }
}
