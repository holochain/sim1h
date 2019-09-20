use crate::dht::bbdht::dynamodb::client::Client;
use dynomite::dynamodb::{DynamoDb, ListTablesError, ListTablesInput, ListTablesOutput};
use rusoto_core::RusotoError;

pub fn list_tables(client: &Client) -> Result<ListTablesOutput, RusotoError<ListTablesError>> {
    client
        .list_tables(ListTablesInput {
            ..Default::default()
        })
        .sync()
}

#[cfg(test)]
pub mod test {
    use crate::dht::bbdht::dynamodb::api::table::list::list_tables;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::trace::tracer;

    #[test]
    pub fn list_tables_test() {
        let log_context = "list_tables_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();

        // list
        assert!(list_tables(&local_client).is_ok());
    }

}
