use uuid::Uuid;
use holochain_persistence_api::cas::content::Address;

pub fn agent_id_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}
