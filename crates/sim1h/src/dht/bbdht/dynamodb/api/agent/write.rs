use crate::agent::AgentAddress;
use crate::dht::bbdht::dynamodb::api::item::keyed_item;
use crate::dht::bbdht::dynamodb::api::item::write::should_put_item_retry;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::PutItemInput;

pub fn touch_agent(
    log_context: &LogContext,
    space: &Space,
    agent_address: &AgentAddress,
) -> BbDhtResult<()> {
    tracer(&log_context, "touch_agent");

    let item = keyed_item(space, &agent_address.into());

    if should_put_item_retry(
        log_context,
        space
            .connection()
            .client()
            .put_item(PutItemInput {
                table_name: space.connection().table_name().into(),
                item: item,
                ..Default::default()
            })
            .sync(),
    )? {
        touch_agent(log_context, space, agent_address)
    } else {
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {

    use crate::agent::fixture::agent_address_fresh;
    use crate::dht::bbdht::dynamodb::api::agent::write::touch_agent;
    use crate::space::fixture::space_fresh;
    use crate::dht::bbdht::dynamodb::api::space::create::ensure_space;
    use crate::trace::tracer;

    #[test]
    fn touch_agent_test() {
        let log_context = "touch_agent_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let agent_id = agent_address_fresh();

        // ensure cas
        assert!(ensure_space(&log_context, &space).is_ok());

        // touch agent
        assert!(touch_agent(&log_context, &space, &agent_id).is_ok());
    }

}
