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
    use crate::test::setup;

    #[test]
    pub fn list_tables_test() {
        setup();

        info!("list_tables_test fixtures");
        let local_client = local_client();

        info!("list_tables_test check ok");
        assert!(list_tables(&local_client).is_ok());
    }

}
