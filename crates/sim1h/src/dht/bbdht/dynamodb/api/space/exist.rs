use crate::dht::bbdht::dynamodb::api::table::exist::table_exists;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::LogContext;

pub fn space_exists(log_context: &LogContext, space: &Space) -> BbDhtResult<bool> {
    table_exists(
        log_context,
        &space.connection().client(),
        &space.connection().table_name(),
    )
}
