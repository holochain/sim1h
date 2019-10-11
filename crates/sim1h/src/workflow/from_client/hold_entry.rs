use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use crate::workflow::state::Sim1hState;
use lib3h_protocol::data_types::ProvidedEntryData;

impl Sim1hState {
    pub fn hold_entry(
        log_context: &LogContext,
        _space: &Space,
        _data: &ProvidedEntryData,
    ) -> BbDhtResult<()> {
        tracer(&log_context, "hold_entry");
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {

    use super::Sim1hState;
    use crate::entry::fixture::entry_address_fresh;
    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;
    use crate::workflow::from_client::fixture::provided_entry_data_fresh;

    #[test]
    fn hold_entry_test() {
        let log_context = "hold_entry_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space, &entry_address.into());

        tracer(&log_context, "check response");
        match Sim1hState::hold_entry(&log_context, &space, &provided_entry_data) {
            Ok(()) => {}
            Err(e) => {
                panic!("bad error {:?}", e);
            }
        }
    }

    #[test]
    fn hold_entry_no_join_test() {
        let log_context = "hold_entry_no_join_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space, &entry_address.into());

        tracer(&log_context, "check response");
        match Sim1hState::hold_entry(&log_context, &space, &provided_entry_data) {
            Ok(()) => {
                tracer(&log_context, "👌");
            }
            Err(e) => {
                panic!("bad error {:?}", e);
            }
        }
    }
}
