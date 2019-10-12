use crate::agent::AgentAddress;
use holochain_core_types::agent::AgentId;
use holochain_core_types::signature::Provenance;
use holochain_core_types::signature::Signature;
use holochain_json_api::json::JsonString;
use holochain_json_api::json::RawString;
use uuid::Uuid;

pub fn agent_address_fresh() -> AgentAddress {
    Uuid::new_v4().to_string().into()
}

pub fn core_nick_fresh() -> String {
    Uuid::new_v4().to_string()
}

pub fn core_agent_id_fresh() -> AgentId {
    AgentId {
        nick: core_nick_fresh(),
        pub_sign_key: agent_address_fresh().into(),
    }
}

pub fn provenance_fresh() -> Provenance {
    Provenance(agent_address_fresh().into(), Signature::fake())
}

pub fn provenances_fresh() -> Vec<Provenance> {
    vec![provenance_fresh(), provenance_fresh()]
}

pub fn message_content_fresh() -> Vec<u8> {
    JsonString::from(RawString::from("foo")).to_bytes()
}
