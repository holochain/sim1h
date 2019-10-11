use crate::agent::AgentAddress;
use crate::dht::bbdht::dynamodb::api::agent::write::touch_agent;
use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use crate::workflow::state::Sim1hState;
use lib3h_protocol::protocol::ClientToLib3hResponse;

impl Sim1hState {
    /// create space if not exists
    /// touch agent
    pub fn join_space(
        log_context: &LogContext,
        space: &Space,
        agent_address: &AgentAddress,
    ) -> BbDhtResult<(ClientToLib3hResponse, Sim1hState)> {
        tracer(&log_context, "join_space");

        ensure_space(&log_context, &space)?;
        touch_agent(&log_context, &space, &agent_address)?;

        let state = Sim1hState::new(space, agent_address);

        Ok((ClientToLib3hResponse::JoinSpaceResult, state))
    }
}

#[cfg(test)]
pub mod tests {

    use super::Sim1hState;
    use crate::space::fixture::space_fresh;
    use crate::space::fixture::space_bad;
    use crate::trace::tracer;
    use crate::agent::fixture::agent_address_fresh;
    use lib3h_protocol::protocol::ClientToLib3hResponse;

    #[test]
    fn join_space_test() {
        let log_context = "join_space_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let agent_address = agent_address_fresh();

        tracer(&log_context, "check response");

        match Sim1hState::join_space(&log_context, &space, &agent_address) {
            Ok((ClientToLib3hResponse::JoinSpaceResult, _)) => {}
            Ok((result, _)) => {
                panic!("test OK panic: {:?}", result);
            }
            Err(err) => {
                tracer(&log_context, "join_space_test Err panic");
                panic!("{:?}", err);
            }
        }
    }

    #[test]
    fn join_space_bad_client_test() {
        let log_context = "join_space_bad_client_test";

        tracer(&log_context, "fixtures");
        let space = space_bad();
        let agent_address = agent_address_fresh();

        tracer(&log_context, "check response");
        match Sim1hState::join_space(&log_context, &space, &agent_address) {
            Err(_) => {
                tracer(&log_context, "👌");
            }
            Ok((v, _)) => {
                panic!("bad Ok {:?}", v);
            }
        }
    }
}
