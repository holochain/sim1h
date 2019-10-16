use crate::dht::bbdht::dynamodb::api::aspect::write::append_aspect_list_to_entry;
use crate::dht::bbdht::error::BbDhtResult;
use crate::entry::EntryAddress;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::data_types::ProvidedEntryData;

/// MVP
/// append list of aspect addresses to entry address
/// drop all aspects into database under each of their addresses
/// later:
/// make all this in a transaction
pub fn publish_entry(
    log_context: &LogContext,
    space: &Space,
    provided_entry_data: &ProvidedEntryData,
) -> BbDhtResult<()> {
    tracer(&log_context, "publish_entry");

    append_aspect_list_to_entry(
        &log_context,
        &space,
        &EntryAddress::from(&provided_entry_data.entry.entry_address),
        &provided_entry_data.entry.aspect_list,
    )?;
    Ok(())
}

#[cfg(test)]
pub mod tests {

    use crate::agent::fixture::agent_address_fresh;
    use crate::entry::fixture::entry_address_fresh;
    use crate::space::fixture::space_bad;
    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;
    use crate::workflow::from_client::fixture::provided_entry_data_fresh;
    use crate::workflow::from_client::publish_entry::publish_entry;
    use crate::workflow::state::Sim1hState;

    #[test]
    fn publish_entry_test() {
        let log_context = "publish_entry_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space, &entry_address.into());
        let agent_address = agent_address_fresh();

        tracer(&log_context, "check response");

        assert!(Sim1hState::join_space(&log_context, &space, &agent_address).is_ok());

        match publish_entry(&log_context, &space, &provided_entry_data) {
            Ok(()) => {
                tracer(&log_context, "👌");
            }
            Err(err) => {
                tracer(&log_context, "publish_entry_test Err panic");
                panic!("{:?} {:?}", err, &provided_entry_data);
            }
        }
    }

    #[test]
    // publishing an entry before joining a space is an error
    fn publish_entry_no_join_test() {
        let log_context = "publish_entry_no_join_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space, &entry_address.into());

        tracer(&log_context, "check response");

        match publish_entry(&log_context, &space, &provided_entry_data) {
            Ok(v) => {
                panic!("bad Ok {:?}", v);
            }
            Err(_) => {
                tracer(&log_context, "👌");
            }
        }
    }

    #[test]
    fn publish_entry_bad_client_test() {
        let log_context = "publish_entry_bad_space_test";

        tracer(&log_context, "fixtures");
        let space = space_bad();
        let entry_address = entry_address_fresh();
        let provided_entry_data = provided_entry_data_fresh(&space, &entry_address.into());

        tracer(&log_context, "check response");
        match publish_entry(&log_context, &space, &provided_entry_data) {
            Ok(result) => {
                panic!("test OK panic: {:?} {:?}", result, &provided_entry_data);
            }
            Err(_) => {
                tracer(&log_context, "👌");
            }
        }
    }
}
