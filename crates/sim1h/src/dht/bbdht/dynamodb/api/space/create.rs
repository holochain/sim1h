use crate::dht::bbdht::dynamodb::api::table::create::ensure_cas_table;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::LogContext;

pub fn ensure_space(log_context: &LogContext, space: &Space) -> BbDhtResult<()> {
    match ensure_cas_table(
        log_context,
        &space.connection().client(),
        &space.connection().table_name(),
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
