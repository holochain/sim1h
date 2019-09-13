extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate dynomite;


#[cfg(test)]
pub mod tests {
    use rusoto_core::Region;
    use dynomite::{
        dynamodb::{
            DynamoDb, DynamoDbClient, ListTablesInput
        },
        retry::Policy,
        Retries,
    };
    use tokio::runtime::Runtime;

    #[test]
    fn local_connection_test() {
        let mut rt = Runtime::new().expect("failed to initialize futures runtime");
        let client = DynamoDbClient::new(Region::Custom {
            name: "us-east-1".into(),
            endpoint: "http://localhost:8000".into(),
        })
        .with_retries(Policy::default());

        let list_tables_input: ListTablesInput = Default::default();

        let foo = rt.block_on(client.list_tables(list_tables_input));

        println!("{:?}", foo);

    }
}
