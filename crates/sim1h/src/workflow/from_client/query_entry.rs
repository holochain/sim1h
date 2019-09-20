use crate::dht::bbdht::dynamodb::api::aspect::read::get_entry_aspects;
use crate::dht::bbdht::dynamodb::client::Client;
use crate::dht::bbdht::error::BbDhtError;
use crate::trace::tracer;
use crate::trace::LogContext;
use holochain_core_types::network::query::NetworkQuery;
use holochain_json_api::json::JsonString;
use lib3h::error::Lib3hResult;
use lib3h_protocol::data_types::EntryAspectData;
use lib3h_protocol::data_types::Opaque;
use lib3h_protocol::data_types::QueryEntryData;
use lib3h_protocol::data_types::QueryEntryResultData;
use lib3h_protocol::protocol::ClientToLib3hResponse;
use std::convert::TryFrom;

pub fn get_entry_aspect_filter_fn(aspect: &EntryAspectData) -> bool {
    let keep = vec!["content".to_string(), "header".to_string()];
    keep.contains(&aspect.type_hint)
}

pub fn query_entry_aspects(
    log_context: &LogContext,
    client: &Client,
    query_entry_data: &QueryEntryData,
) -> Lib3hResult<Vec<EntryAspectData>> {
    tracer(&log_context, "publish_entry");

    let table_name = query_entry_data.space_address.to_string();
    let entry_address = query_entry_data.entry_address.clone();

    let query_raw = query_entry_data.query.as_slice();
    let utf8_result = std::str::from_utf8(&query_raw.clone());
    let query_str = match utf8_result {
        Ok(v) => v,
        Err(err) => Err(BbDhtError::CorruptData(err.to_string()))?,
    };
    let query_json = JsonString::from_json(&query_str.to_string());
    let query = match NetworkQuery::try_from(query_json.clone()) {
        Ok(v) => v,
        Err(err) => Err(BbDhtError::CorruptData(err.to_string()))?,
    };

    let entry_aspects = get_entry_aspects(log_context, client, &table_name, &entry_address)?;

    Ok(match query {
        NetworkQuery::GetEntry => {
            let v = entry_aspects
                .into_iter()
                .filter(get_entry_aspect_filter_fn)
                .collect::<Vec<_>>();
            v
        }
        NetworkQuery::GetLinks(
            _link_type,
            _link_tag,
            _maybe_crud_status,
            _get_links_network_query,
        ) => {
            let v = entry_aspects
                .into_iter()
                .filter(|_| true)
                .collect::<Vec<_>>();
            v
        }
    })
}

pub fn aspects_to_opaque(aspects: &Vec<EntryAspectData>) -> Opaque {
    let json = JsonString::from(aspects.clone());
    json.to_bytes().into()
}

/// 90% (need query logic to be finalised)
/// fetch all entry aspects from entry address
/// do some kind of filter based on the non-opaque query struct
/// familiar to rehydrate the opaque query struct
pub fn query_entry(
    log_context: &LogContext,
    client: &Client,
    query_entry_data: &QueryEntryData,
) -> Lib3hResult<ClientToLib3hResponse> {
    let entry_aspects = query_entry_aspects(log_context, client, query_entry_data)?;
    Ok(ClientToLib3hResponse::QueryEntryResult(
        QueryEntryResultData {
            entry_address: query_entry_data.entry_address.clone(),
            request_id: query_entry_data.request_id.clone(),
            space_address: query_entry_data.space_address.clone(),
            query_result: aspects_to_opaque(&entry_aspects),
            requester_agent_id: query_entry_data.requester_agent_id.clone(),
            responder_agent_id: query_entry_data.requester_agent_id.clone(),
        },
    ))
}

#[cfg(test)]
pub mod tests {

    use crate::aspect::fixture::content_aspect_fresh;
    use crate::aspect::fixture::deletion_aspect_fresh;
    use crate::aspect::fixture::header_aspect_fresh;
    use crate::aspect::fixture::link_add_aspect_fresh;
    use crate::aspect::fixture::link_remove_aspect_fresh;
    use crate::aspect::fixture::update_aspect_fresh;
    use crate::dht::bbdht::dynamodb::client::local::local_client;
    use crate::entry::fixture::entry_address_fresh;
    use crate::entry::fixture::entry_fresh;
    use crate::space::fixture::space_data_fresh;
    use crate::test::unordered_vec_compare;
    use crate::trace::tracer;
    use crate::workflow::from_client::fixture::provided_entry_data_fresh;
    use crate::workflow::from_client::fixture::query_entry_data_fresh;
    use crate::workflow::from_client::join_space::join_space;
    use crate::workflow::from_client::publish_entry::publish_entry;
    use crate::workflow::from_client::query_entry::get_entry_aspect_filter_fn;
    use crate::workflow::from_client::query_entry::query_entry_aspects;

    #[test]
    pub fn get_entry_aspect_filter_fn_test() {
        // things that should persist
        assert!(get_entry_aspect_filter_fn(&content_aspect_fresh().into()));
        assert!(get_entry_aspect_filter_fn(
            &header_aspect_fresh(&entry_fresh()).into()
        ));

        // things that should be dropped
        assert!(!get_entry_aspect_filter_fn(
            &link_add_aspect_fresh(&entry_fresh()).into()
        ));
        assert!(!get_entry_aspect_filter_fn(
            &link_remove_aspect_fresh(&entry_fresh()).into()
        ));
        assert!(!get_entry_aspect_filter_fn(
            &update_aspect_fresh(&entry_fresh()).into()
        ));
        assert!(!get_entry_aspect_filter_fn(
            &deletion_aspect_fresh(&entry_fresh()).into()
        ));
    }

    #[test]
    pub fn query_entry_aspects_test() {
        let log_context = "query_entry_aspects_test";

        tracer(&log_context, "fixtures");
        let local_client = local_client();
        let space_data = space_data_fresh();
        let entry_address = entry_address_fresh();
        let query_entry_data = query_entry_data_fresh(&space_data, &entry_address);
        let provided_entry_data = provided_entry_data_fresh(&space_data, &entry_address);

        // join space
        assert!(join_space(&log_context, &local_client, &space_data).is_ok());

        // publish entry
        assert!(publish_entry(&log_context, &local_client, &provided_entry_data).is_ok());

        match query_entry_aspects(&log_context, &local_client, &query_entry_data) {
            Ok(v) => assert!(unordered_vec_compare(
                v,
                provided_entry_data.entry.aspect_list
            )),
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

}
