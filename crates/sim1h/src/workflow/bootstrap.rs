use crate::dht::bbdht::dynamodb::account::describe_limits;
use lib3h_zombie_actor::GhostResult;
use crate::dht::bbdht::dynamodb::client::Client;
use std::error::Error;
use lib3h_zombie_actor::WorkWasDone;
use lib3h_zombie_actor::GhostError;

pub fn bootstrap(client: &Client) -> GhostResult<WorkWasDone> {
    match describe_limits(&client) {
        Ok(_) => Ok(true.into()),
        Err(err) => Err(GhostError::from(err.description())),
    }
}

#[cfg(test)]
pub mod tests {

    use crate::workflow::bootstrap::bootstrap;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::dht::bbdht::dynamodb::client::fixture::bad_client;
    use crate::test::setup;
    use lib3h_zombie_actor::GhostError;


    #[test]
    fn bootstrap_test() {
        setup();

        info!("bootstrap_test fixtures");
        let local_client = local_client();

        info!("bootstrap_test bootstrap successful");
        assert_eq!(
            Ok(true.into()),
            bootstrap(&local_client),
        );
    }

    #[test]
    fn bootstrap_bad_client_test() {
        setup();

        info!("boostrap_bad_client_test fixtures");
        let bad_client = bad_client();

        info!("boostrap_bad_client_test fails");
        assert_eq!(
            Err(GhostError::from(String::from("error trying to connect: failed to lookup address information: Name or service not known"))),
            bootstrap(&bad_client),
        );
    }

}
