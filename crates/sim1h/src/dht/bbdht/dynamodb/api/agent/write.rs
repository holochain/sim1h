use crate::dht::bbdht::dynamodb::api::item::write::should_put_item_retry;
use crate::dht::bbdht::error::BbDhtResult;
use crate::trace::tracer;
use crate::trace::LogContext;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::PutItemInput;
use crate::dht::bbdht::dynamodb::api::item::keyed_item;
use crate::dht::bbdht::dynamodb::api::agent::AgentAddress;
use crate::space::Space;

pub fn touch_agent(
    log_context: &LogContext,
    space: &Space,
    agent_address: &AgentAddress,
) -> BbDhtResult<()> {
    tracer(&log_context, "touch_agent");

    let item = keyed_item(log_context, space, &String::from(agent_address.to_owned()));

    if should_put_item_retry(
        log_context,
        space.client
            .put_item(PutItemInput {
                table_name: space.table_name.to_string(),
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

    use crate::agent::fixture::agent_id_fresh;
    use crate::dht::bbdht::dynamodb::api::agent::write::touch_agent;
    use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
    use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
    use crate::dht::bbdht::dynamodb::api::table::fixture::table_name_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;

    #[test]
    fn touch_agent_test() {
        let log_context = "touch_agent_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let table_name = table_name_fresh();
        let agent_id = agent_id_fresh();

        // ensure cas
        assert!(ensure_cas_table(&log_context, &local_client, &table_name).is_ok());

        // cas exists
        assert!(table_exists(&log_context, &local_client, &table_name).is_ok());

        // touch agent
        assert!(touch_agent(&log_context, &local_client, &table_name, &agent_id).is_ok());
    }

}
