use crate::dht::bbdht::dynamodb::schema::TableName;
use uuid::Uuid;

pub fn table_name_fresh() -> TableName {
    format!("table_{}", Uuid::new_v4()).into()
}
