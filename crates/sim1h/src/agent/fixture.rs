use holochain_persistence_api::cas::content::Address;
use uuid::Uuid;

pub fn agent_id_fresh() -> Address {
    Address::from(Uuid::new_v4().to_string())
}
