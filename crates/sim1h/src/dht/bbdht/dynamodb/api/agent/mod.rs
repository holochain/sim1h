pub mod inbox;
pub mod read;
pub mod write;

use holochain_persistence_api::cas::content::Address;

pub struct AgentAddress(Address);
