use crate::agent::AgentAddress;
use crate::dht::bbdht::dynamodb::api::agent::inbox::check_inbox;
use crate::dht::bbdht::dynamodb::api::aspect::read::get_aspect;
use crate::dht::bbdht::dynamodb::api::aspect::read::scan_aspects;
use crate::dht::bbdht::dynamodb::api::item::Item;
use crate::space::Space;
use lib3h_protocol::data_types::GetListData;
use lib3h_protocol::data_types::StoreEntryAspectData;
use lib3h_protocol::protocol::ClientToLib3hResponse;
use lib3h_protocol::protocol::Lib3hToClient;
use lib3h_protocol::Address;
use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use uuid::Uuid;

pub type AspectAddressMap = HashMap<Address, HashSet<Address>>;
type Sim1hResult<T> = Result<T, String>;

const MIN_TOLERABLE_TICK_INTERVAL_MS: u128 = 80;

#[derive(Default)]
pub struct Sim1hState {
    pub initialized: bool,
    space: Space,
    agent_address: AgentAddress,
    pub client_request_outbox: Vec<Lib3hToClient>,
    pub client_response_outbox: Vec<ClientToLib3hResponse>,
    pub held_aspects: AspectAddressMap,
    num_ticks: u32,
    last_tick_instant: Option<Instant>,
    last_evaluated_scan_key: Option<Item>,
}

impl Sim1hState {
    pub fn new(space: &Space, agent_address: &AgentAddress) -> Self {
        Self {
            space: space.to_owned(),
            agent_address: agent_address.to_owned(),
            ..Self::default()
        }
    }

    pub fn space(&self) -> &Space {
        &self.space
    }

    pub fn agent_address(&self) -> &AgentAddress {
        &self.agent_address
    }

    fn should_get_authoring_list(&mut self) -> bool {
        self.initialized == false
    }

    fn create_authoring_gossip_list_requests(&self) -> Vec<Lib3hToClient> {
        let mut requests = Vec::new();
        requests.push(Lib3hToClient::HandleGetAuthoringEntryList(GetListData {
            space_address: self.space().space_address().into(),
            provider_agent_id: self.agent_address().into(),
            request_id: "".into(),
        }));
        requests.push(Lib3hToClient::HandleGetGossipingEntryList(GetListData {
            space_address: self.space().space_address().into(),
            provider_agent_id: self.agent_address().into(),
            request_id: "".into(),
        }));

        requests
    }

    fn create_direct_message_requests(&self) -> Vec<Lib3hToClient> {
        if !self.initialized {
            return Vec::new();
        }
        let log_context = "Sim1hState::create_direct_message_requests";
        match check_inbox(&log_context, self.space(), &self.agent_address().into()) {
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
    }

    fn create_store_requests(&mut self) -> Sim1hResult<Vec<Lib3hToClient>> {
        if !self.initialized {
            return Ok(Vec::new());
        }

        let log_context = "create_store_requests";
        let (incoming, last_evaluated_key) = scan_aspects(
            &log_context,
            self.space(),
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
                        get_aspect(&log_context, self.space(), &aspect_address.into())
                            .expect("Cannot get aspect")
                    })
                    .map(|entry_aspect| {
                        Lib3hToClient::HandleStoreEntryAspect(StoreEntryAspectData {
                            request_id: Uuid::new_v4().to_string(), // XXX: well, is this so bad?
                            space_address: self.space().space_address().into(),
                            provider_agent_id: self.agent_address().into(), // TODO: is this OK?
                            entry_address: entry_address.clone(),
                            entry_aspect,
                        })
                    })
                    .collect(),
            );
        }

        Ok(messages)
    }

    pub fn process_pending_requests_to_client(&mut self) -> Sim1hResult<Vec<Lib3hToClient>> {
        let requests = if self.should_get_authoring_list() {
            self.initialized = true;
            self.create_authoring_gossip_list_requests()
        } else {
            Vec::new()
        };

        let now = Instant::now();
        if let Some(then) = self.last_tick_instant {
            let millis = now.duration_since(then).as_millis();
            if millis < MIN_TOLERABLE_TICK_INTERVAL_MS {
                return Ok(Vec::new());
            }
        }
        self.last_tick_instant = Some(now);
        self.num_ticks += 1;

        let messages = requests
            .into_iter()
            .chain(self.create_store_requests()?.into_iter())
            .chain(self.create_direct_message_requests().into_iter())
            .chain(self.client_request_outbox.drain(..))
            .collect();

        Ok(messages)
    }

    pub fn process_pending_responses_to_client(&mut self) -> Vec<ClientToLib3hResponse> {
        self.client_response_outbox.drain(..).collect()
    }
}
