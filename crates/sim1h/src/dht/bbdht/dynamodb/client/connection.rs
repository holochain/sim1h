use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::dynamodb::schema::TableName;
use crate::dht::bbdht::dynamodb::client::local::local_client;

#[derive(Clone)]
pub struct Connection {
    client: Client,
    table_name: TableName,
}

impl Default for Connection {
    fn default() -> Self {
        Connection {
            client: local_client(),
            table_name: TableName::default()
        }
    }
}


impl Connection {
    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn table_name(&self) -> &TableName {
        &self.table_name
    }

    pub fn new(client: &Client, table_name: &TableName) -> Connection {
        Connection {
            client: client.to_owned(),
            table_name: table_name.to_owned(),
        }
    }
}
