use crate::trace::tracer;
use lib3h::error::Lib3hResult;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::trace::LogContext;
use crate::dht::bbdht::error::BbDhtError;
use lib3h_protocol::data_types::QueryEntryData;
use crate::dht::bbdht::dynamodb::api::aspect::read::get_entry_aspects;
use holochain_json_api::json::JsonString;
use holochain_core_types::network::query::NetworkQuery;
use std::convert::TryFrom;

pub fn query_entry(
    log_context: &LogContext,
    client: &Client,
    query_entry_data: &QueryEntryData,
) -> Lib3hResult<()> {
    tracer(&log_context, "publish_entry");

    let table_name = query_entry_data.space_address.to_string();
    let entry_address = query_entry_data.entry_address.clone();

    let query_raw = query_entry_data.query.as_slice();
    let utf8_result = std::str::from_utf8(&query_raw.clone());
    let query_str = match utf8_result {
        Ok(v) => v,
        Err(err) => {
            Err(BbDhtError::CorruptData(err.to_string()))?
        }
    };
    let query_json = JsonString::from_json(&query_str.to_string());
    let query = match NetworkQuery::try_from(query_json.clone()) {
        Ok(v) => v,
        Err(err) => {
            Err(BbDhtError::CorruptData(err.to_string()))?
        }
    };

    let _entry_aspects = get_entry_aspects(
        log_context,
        client,
        &table_name,
        &entry_address,
    );


    Ok(())
}

#[cfg(test)]
pub mod tests {

    use crate::workflow::query_entry::query_entry;
    use crate::trace::tracer;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::workflow::fixture::query_entry_data_fresh;
    use crate::workflow::fixture::space_data_fresh;
    use crate::workflow::fixture::entry_address_fresh;

    #[test]
    pub fn query_entry_test() {
        let log_context = "query_entry_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let space_data = space_data_fresh();
        let entry_address = entry_address_fresh();
        let query_entry_data = query_entry_data_fresh(&space_data, &entry_address);

        assert!(query_entry(&log_context, &local_client, &query_entry_data).is_ok());
    }

}
