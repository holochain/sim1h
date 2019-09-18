use lib3h::error::Lib3hError;
use lib3h_zombie_actor::GhostActor;
use lib3h_protocol::protocol::Lib3hToClient;
use lib3h_protocol::protocol::Lib3hToClientResponse;
use lib3h_protocol::protocol::ClientToLib3hResponse;
use lib3h_protocol::protocol::ClientToLib3h;
use lib3h_zombie_actor::WorkWasDone;
use lib3h_zombie_actor::GhostResult;
use lib3h_zombie_actor::GhostEndpoint;
use lib3h_zombie_actor::create_ghost_channel;
use lib3h_zombie_actor::GhostContextEndpoint;
use detach::Detach;

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
}

impl SimGhostActor {

    pub fn new(_netname: String) -> Self {
        let (endpoint_parent, endpoint_self) = create_ghost_channel();
        Self {
            client_endpoint: Some(endpoint_parent),
            lib3h_endpoint: Detach::new(
                endpoint_self
                    .as_context_endpoint_builder()
                    .request_id_prefix(REQUEST_ID_PREFIX)
                    .build(),
            ),
            // reciever: None,
        }
    }

}

impl<'engine>
    GhostActor<
        Lib3hToClient,
        Lib3hToClientResponse,
        ClientToLib3h,
        ClientToLib3hResponse,
        Lib3hError,
    > for SimGhostActor {

        /// our parent gets a reference to the parent side of our channel
        fn take_parent_endpoint(
            &mut self,
        ) -> Option<GhostEndpoint<ClientToLib3h, ClientToLib3hResponse, Lib3hToClient, Lib3hToClientResponse, Lib3hError>>
        {
            std::mem::replace(&mut self.client_endpoint, None)
        }

        /// our parent will call this process function
        fn process(&mut self) -> GhostResult<WorkWasDone> {
            // it would be awesome if this trait level could handle things like:
            //  `self.endpoint_self.process();`
            self.process_concrete()
        }

        /// we, as a ghost actor implement this, it will get called from
        /// process after the subconscious process items have run
        fn process_concrete(&mut self) -> GhostResult<WorkWasDone> {
            Ok(false.into())
        }

    }
