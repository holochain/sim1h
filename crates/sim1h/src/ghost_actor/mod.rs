use detach::Detach;
use lib3h::error::Lib3hError;
use lib3h_protocol::protocol::ClientToLib3h;
use lib3h_protocol::protocol::ClientToLib3hResponse;
use lib3h_protocol::protocol::Lib3hToClient;
use lib3h_protocol::protocol::Lib3hToClientResponse;
use lib3h_zombie_actor::create_ghost_channel;
use lib3h_zombie_actor::GhostActor;
use crate::workflow::bootstrap::bootstrap;
use lib3h_zombie_actor::GhostContextEndpoint;
use lib3h_zombie_actor::GhostEndpoint;
use lib3h_zombie_actor::GhostResult;
use lib3h_zombie_actor::GhostCanTrack;
use lib3h_zombie_actor::WorkWasDone;
use url::Url;
use lib3h::engine::engine_actor::ClientToLib3hMessage;
use lib3h::engine::CanAdvertise;
use rusoto_core::Region;
use crate::dht::bbdht::dynamodb::client::{client, Client};

const REQUEST_ID_PREFIX: &str = "sim";

pub struct SimGhostActor {
    pub lib3h_endpoint: Detach<
        GhostContextEndpoint<
            SimGhostActor,
            Lib3hToClient,
            Lib3hToClientResponse,
            ClientToLib3h,
            ClientToLib3hResponse,
            Lib3hError,
        >,
    >,
    pub client_endpoint: Option<
        GhostEndpoint<
            ClientToLib3h,
            ClientToLib3hResponse,
            Lib3hToClient,
            Lib3hToClientResponse,
            Lib3hError,
        >,
    >,
    #[allow(dead_code)]
    dbclient: Client,
}

impl SimGhostActor {
    pub fn new(endpoint: &String) -> Self {
        let (endpoint_parent, endpoint_self) = create_ghost_channel();
        Self {
            client_endpoint: Some(endpoint_parent),
            lib3h_endpoint: Detach::new(
                endpoint_self
                    .as_context_endpoint_builder()
                    .request_id_prefix(REQUEST_ID_PREFIX)
                    .build(),
            ),
            dbclient: client(Region::Custom{
                name: "".to_string(),
                endpoint: endpoint.to_string()
            }),
        }
    }

    pub fn handle_msg_from_client(&mut self, mut msg: ClientToLib3hMessage) -> GhostResult<WorkWasDone> {
        match msg.take_message().expect("exists") {
            ClientToLib3h::Bootstrap(data) => {
                trace!("ClientToLib3h::Bootstrap: {:?}", &data);
                msg.respond(bootstrap(&self.dbclient))?;
                // msg.respond(Err(Lib3hError::from("test error")))?;
                Ok(true.into())
            },
            ClientToLib3h::JoinSpace(data) => {
                trace!("ClientToLib3h::JoinSpace: {:?}", &data);
                Ok(true.into())
            },
            ClientToLib3h::LeaveSpace(data) => {
                trace!("ClientToLib3h::LeaveSpace: {:?}", &data);
                Ok(true.into())
            },
            ClientToLib3h::SendDirectMessage(data) => {
                trace!("ClientToLib3h::SendDirectMessage: {:?}", &data);
                Ok(true.into())
            },
            ClientToLib3h::PublishEntry(data) => {
                trace!("ClientToLib3h::PublishEntry: {:?}", &data);
                Ok(true.into())
            },
            ClientToLib3h::HoldEntry(data) => {
                trace!("ClientToLib3h::HoldEntry: {:?}", &data);
                Ok(true.into())
            },
            ClientToLib3h::QueryEntry(data) => {
                trace!("ClientToLib3h::QueryEntry: {:?}", &data);
                Ok(true.into())
            },
            ClientToLib3h::FetchEntry(data) => {
                trace!("ClientToLib3h::FetchEntry: {:?}", &data);
                Ok(true.into())
            },
        }
    }
}

impl CanAdvertise for SimGhostActor {

    fn advertise(&self) -> Url {
        Url::parse("ws://example.com").unwrap()
    }

}

impl<'engine>
    GhostActor<
        Lib3hToClient,
        Lib3hToClientResponse,
        ClientToLib3h,
        ClientToLib3hResponse,
        Lib3hError,
    > for SimGhostActor
{
    /// our parent gets a reference to the parent side of our channel
    fn take_parent_endpoint(
        &mut self,
    ) -> Option<
        GhostEndpoint<
            ClientToLib3h,
            ClientToLib3hResponse,
            Lib3hToClient,
            Lib3hToClientResponse,
            Lib3hError,
        >,
    > {
        std::mem::replace(&mut self.client_endpoint, None)
    }

    /// we, as a ghost actor implement this, it will get called from
    /// process after the subconscious process items have run
    fn process_concrete(&mut self) -> GhostResult<WorkWasDone> {
        // always run the endpoint process loop
        detach_run!(&mut self.lib3h_endpoint, |cs| { cs.process(self) })?;

        let mut work_was_done = false;
        // process any messages from the client to us
        for msg in self.lib3h_endpoint.as_mut().drain_messages() {
            match self.handle_msg_from_client(msg) {
                Ok(msg_work_was_done) => work_was_done = work_was_done || msg_work_was_done.into(),
                Err(err) => return Err(err),
            }
        }

        // Done
        Ok(work_was_done.into())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use lib3h_tracing::test_span;
    use lib3h_zombie_actor::GhostCallbackData;
    use lib3h_protocol::{
        data_types::*,
        Address,
    };

    fn get_response_to_request(request: ClientToLib3h) -> GhostCallbackData<ClientToLib3hResponse, Lib3hError> {
        let mut engine = SimGhostActor::new(&"invalid-endpoint".to_string());
        let mut parent_endpoint: GhostContextEndpoint<(), _, _, _, _, _> = engine
            .take_parent_endpoint()
            .expect("Could not get parent endpoint")
            .as_context_endpoint_builder()
            .request_id_prefix("parent")
            .build();

        let (s, r) = crossbeam_channel::unbounded();

        parent_endpoint.request(test_span(""), request, Box::new(move |_, callback_data| {
            s.send(callback_data).unwrap();
            Ok(())
        })).ok();

        for _ in 0..2 { // process a few times, once isn't enough..
            parent_endpoint.process(&mut ()).ok();
            engine.process().ok();
        }

        r.recv().expect("Could not retrieve result")
    }

    #[test]
    fn can_send_bootstrap_message() {
        let bootstrap_data = BootstrapData {
            space_address: Address::from(""),
            bootstrap_uri: Url::parse("http://fake_url").unwrap(),
        };

        match get_response_to_request(ClientToLib3h::Bootstrap(bootstrap_data)) {
            GhostCallbackData::Response(Err(_)) => assert!(true),
            GhostCallbackData::Timeout => panic!("unexpected timeout"),
            r => panic!("unexpected response: {:?}", r)
        }
    }
}
