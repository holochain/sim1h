use crate::dht::bbdht::dynamodb::account::describe_limits;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::tracer;
use crate::trace::LogContext;
use lib3h::error::Lib3hError;
use lib3h::error::Lib3hResult;
use lib3h_protocol::protocol::ClientToLib3hResponse;

pub fn bootstrap(log_context: &LogContext, client: &Client) -> Lib3hResult<ClientToLib3hResponse> {
    tracer(&log_context, "bootstrap");
    match describe_limits(&log_context, &client) {
        Ok(_) => Ok(ClientToLib3hResponse::BootstrapSuccess),
        Err(err) => Err(Lib3hError::from(err.to_string())),
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::dht::bbdht::dynamodb::client::fixture::bad_client;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;
    use crate::workflow::bootstrap::bootstrap;

    #[test]
    fn bootstrap_test() {
        let log_context = "bootstrap_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();

        // success
        match bootstrap(&log_context, &local_client) {
            Ok(ClientToLib3hResponse::BootstrapSuccess) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn bootstrap_bad_client_test() {
        let log_context = "bootstrap_bad_client_test";

        tracer(&log_context, "fixtures");
        let bad_client = bad_client();

        // fail
        match bootstrap(&log_context, &bad_client) {
            Err(err) => {
                assert_eq!(
                    err.to_string(),
                    "Unknown error encountered: \'error trying to connect: failed to lookup address information: Name or service not known\'.".to_string(),
                );
            }
            Ok(_) => unreachable!(),
        };
    }

}
