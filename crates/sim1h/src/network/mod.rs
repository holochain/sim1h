pub mod fixture;

use holochain_persistence_api::cas::content::Address;

#[derive(Debug, PartialEq, Clone)]
pub struct RequestId(String);
#[derive(Clone, Default)]
pub struct NetworkId(String);

impl From<NetworkId> for String {
    fn from(network_id: NetworkId) -> Self {
        network_id.0
    }
}

impl From<String> for NetworkId {
    fn from(string: String) -> Self {
        NetworkId(string)
    }
}

impl From<&NetworkId> for String {
    fn from(network_id: &NetworkId) -> Self {
        network_id.to_owned().into()
    }
}

impl From<RequestId> for String {
    fn from(request_id: RequestId) -> Self {
        request_id.0
    }
}

impl From<&String> for RequestId {
    fn from(string: &String) -> Self {
        string.to_owned().into()
    }
}

impl From<RequestId> for Address {
    fn from(request_id: RequestId) -> Self {
        let string: String = request_id.into();
        Address::from(string)
    }
}

impl From<&RequestId> for Address {
    fn from(request_id: &RequestId) -> Self {
        request_id.to_owned().into()
    }
}

impl From<String> for RequestId {
    fn from(string: String) -> Self {
        RequestId(string)
    }
}
