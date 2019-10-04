use uuid::Uuid;
use crate::dht::bbdht::dynamodb::schema::TableName;

pub fn table_name_fresh() -> TableName {
    format!("table_{}", Uuid::new_v4()).into()
}
