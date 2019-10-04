use crate::trace::LogContext;
use crate::space::Space;
use crate::dht::bbdht::error::BbDhtResult;
use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;

pub fn ensure_space(
    log_context: &LogContext,
    space: &Space,
) -> BbDhtResult<bool> {
    ensure_cas_table(log_context, &space.client, &space.table_name)
}
