pub mod fixture;
use holochain_persistence_api::cas::content::Address;

#[derive(Clone)]
pub struct EntryAddress(Address);

impl From<EntryAddress> for String {
    fn from(entry_address: EntryAddress) -> Self {
        entry_address.0.into()
    }
}

impl From<&EntryAddress> for String {
    fn from(entry_address: &EntryAddress) -> Self {
        entry_address.to_owned().into()
    }
}

impl From<Address> for EntryAddress {
    fn from(address: Address) -> Self {
        EntryAddress(address)
    }
}

impl From<&Address> for EntryAddress {
    fn from(address: &Address) -> Self {
        address.to_owned().into()
    }
}

impl From<EntryAddress> for Address {
    fn from(entry_address: EntryAddress) -> Self {
        entry_address.0
    }
}

impl From<String> for EntryAddress {
    fn from(string: String) -> Self {
        EntryAddress(string.into())
    }
}
