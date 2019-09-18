use crate::dht::bbdht::dynamodb::api::item::write::touch_agent;
use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hError;
use lib3h::error::Lib3hResult;
use lib3h_protocol::data_types::SpaceData;
use lib3h_protocol::protocol::ClientToLib3hResponse;

pub fn join_space(
    log_context: &LogContext,
    client: &Client,
    join_space_data: &SpaceData,
) -> Lib3hResult<ClientToLib3hResponse> {
    tracer(&log_context, "join_space");

    let table_name = String::from(join_space_data.space_address.clone());

    match ensure_cas_table(&log_context, &client, &table_name) {
        Ok(_) => {}
        Err(err) => return Err(Lib3hError::from(err.to_string())),
    };
    match touch_agent(
        &log_context,
        &client,
        &table_name,
        &join_space_data.agent_id,
    ) {
        Ok(_) => Ok(ClientToLib3hResponse::JoinSpaceResult),
        Err(err) => Err(Lib3hError::from(err.to_string())),
    }
}
