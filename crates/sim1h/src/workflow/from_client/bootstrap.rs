use crate::dht::bbdht::dynamodb::account::describe_limits;
use crate::dht::bbdht::error::BbDhtResult;
use crate::space::Space;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h_protocol::protocol::ClientToLib3hResponse;

/// check database connection
/// optional
pub fn bootstrap(log_context: &LogContext, space: &Space) -> BbDhtResult<ClientToLib3hResponse> {
    tracer(&log_context, "bootstrap");
    // touch the database to check our connection is good
    describe_limits(&log_context, &space.connection().client())?;
    Ok(ClientToLib3hResponse::BootstrapSuccess)
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::space::fixture::space_bad;
    use crate::space::fixture::space_fresh;
    use crate::trace::tracer;
    use crate::workflow::from_client::bootstrap::bootstrap;

    #[test]
    fn bootstrap_test() {
        let log_context = "bootstrap_test";

        tracer(&log_context, "fixtures");
        let space = space_fresh();

        // success
        match bootstrap(&log_context, &space) {
            Ok(ClientToLib3hResponse::BootstrapSuccess) => {}
            Ok(v) => {
                panic!("Bad Ok {:?}", v);
            }
            Err(err) => {
                panic!("Err {:?}", err);
            }
        }
    }

    #[test]
    fn bootstrap_bad_space_test() {
        let log_context = "bootstrap_bad_space_test";

        tracer(&log_context, "fixtures");
        let space = space_bad();

        // fail
        match bootstrap(&log_context, &space) {
            Err(_) => {
                tracer(&log_context, "👌");
            }
            Ok(v) => {
                panic!("bad Ok {:?}", v);
            }
        };
    }

}
