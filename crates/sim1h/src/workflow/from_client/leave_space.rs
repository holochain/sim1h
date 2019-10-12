use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::protocol::ClientToLib3hResponse;

/// no-op
pub fn leave_space(log_context: &LogContext, _space: &Space) -> BbDhtResult<ClientToLib3hResponse> {
    tracer(&log_context, "leave_space");
    // leave space is a no-op in a simulation
    Ok(ClientToLib3hResponse::LeaveSpaceResult)
}

#[cfg(test)]
pub mod tests {

    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;
    use crate::workflow::from_client::leave_space::leave_space;
    use lib3h_protocol::protocol::ClientToLib3hResponse;

    #[test]
    fn leave_space_test() {
        let log_context = "leave_space_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();

        tracer(&log_context, "check response");
        match leave_space(&log_context, &space) {
            Ok(ClientToLib3hResponse::LeaveSpaceResult) => {
                tracer(&log_context, "👌");
            }
            Ok(v) => {
                panic!("bad Ok {:?}", v);
            }
            Err(err) => {
                panic!("Err {:?}", err);
            }
        }
    }

}
