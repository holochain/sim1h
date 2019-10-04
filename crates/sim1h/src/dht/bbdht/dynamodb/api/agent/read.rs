use crate::agent::AgentAddress;
use crate::dht::bbdht::dynamodb::api::item::read::get_item_from_space;
use crate::dht::bbdht::dynamodb::api::space::exist::space_exists;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;

pub fn agent_exists(
    log_context: &LogContext,
    space: &Space,
    agent_address: &AgentAddress,
) -> BbDhtResult<bool> {
    tracer(&log_context, "agent_exists");

    // agent only exists in the space if the space exists
    Ok(if space_exists(log_context, space)? {
        get_item_from_space(log_context, space, &agent_address.into())?.is_some()
    } else {
        false
    })
}

#[cfg(test)]
pub mod tests {

    use crate::space::fixture::space_fresh;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::agent::fixture::agent_address_fresh;
    use crate::dht::bbdht::dynamodb::api::agent::read::agent_exists;
    use crate::dht::bbdht::dynamodb::api::agent::write::touch_agent;
    use crate::trace::tracer;

    #[test]
    fn agent_exists_test() {
        let log_context = "agent_exists";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let agent_id = agent_address_fresh();

        // agent not exists if space not exists
        match agent_exists(&log_context, &space, &agent_id) {
            Ok(false) => {
                tracer(&log_context, "👌");
            }
            Ok(true) => {
                panic!("apparently agent exists before the space does");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // agent not exists if not join space
        match agent_exists(&log_context, &space, &agent_id) {
            Ok(false) => {
                tracer(&log_context, "👌");
            }
            Ok(true) => {
                panic!("agent exists before join");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        };

        // join
        assert!(touch_agent(&log_context, &space, &agent_id).is_ok());

        // agent exists now
        match agent_exists(&log_context, &space, &agent_id) {
            Ok(false) => {
                panic!("agent not exists after join");
            }
            Ok(true) => {
                tracer(&log_context, "👌");
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

}
