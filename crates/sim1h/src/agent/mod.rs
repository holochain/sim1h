pub mod fixture;

use holochain_persistence_api::cas::content::Address;

#[derive(Default, Clone)]
pub struct AgentAddress(Address);

impl From<AgentAddress> for Address {
    fn from(agent_address: AgentAddress) -> Self {
        agent_address.0
    }
}

impl From<String> for AgentAddress {
    fn from(string: String) -> Self {
        AgentAddress(string.into())
    }
}

impl From<&AgentAddress> for Address {
    fn from(agent_address: &AgentAddress) -> Self {
        agent_address.to_owned().into()
    }
}

impl From<AgentAddress> for String {
    fn from(agent_address: AgentAddress) -> Self {
        agent_address.0.into()
    }
}
