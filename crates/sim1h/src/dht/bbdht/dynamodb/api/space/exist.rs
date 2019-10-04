use crate::trace::LogContext;
use crate::space::Space;
use crate::dht::bbdht::error::BbDhtResult;
use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;

pub fn space_exists(
    log_context: &LogContext,
    space: &Space,
) -> BbDhtResult<bool> {
    table_exists(log_context, &space.client, &space.table_name)
}
