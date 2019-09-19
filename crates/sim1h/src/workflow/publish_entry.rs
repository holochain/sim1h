pub fn publish_entry(log_context: &LogContext, client: &Client, provided_entry_data: &ProvidedEntryData) -> Lib3hResult<ClientToLib3hResponse> {
    let mut writes = Vec::new();
    let mut aspect_addresses = Vec::new();

    for aspect in provided_entry_data.entry.aspect_list {
        let mut entry_aspect = HashMap::new();
        entry_aspect.insert(ADDRESS_KEY.to_string(), string_attribute(aspect.aspect_address));
        entry_aspect.insert(ASPECT_ADDRESS.to_string(), string_attribute(aspect.aspect_address));
        entry_aspect.insert(TYPE_HINT.to_string(), string_attribute(aspect.type_hint));
        entry_aspect.insert(ASPECT.to_string(), blob_attribute(aspect.aspect));
        entry_aspect.insert(PUBLISH_TS.to_string(), number_attribute(aspect.publish_ts));
        writes.append(entry_aspect);
        aspect_addresses.append(aspect.aspect_address);
    }

    // https://stackoverflow.com/questions/31288085/how-to-append-a-value-to-list-attribute-on-aws-dynamodb

    let list_write = XXX{
        Key: {
            provided_entry_data.entry.entry_address
        },
        UpdateExpression: format!("SET aspects = list_append({}, :i)", ASPECT_LIST),
        ExpressionAttributeValues: {
            i: aspect_addresses,
        },
    };
    writes.append(list_write);

    client.transact_bulk_updates(TransactBulkUpdatesInput{
        table_name: provided_entry_data.space_address,
    }).sync()

}
