pub fn query_entry(
    log_context: &LogContext,
    client: &Client,
    provided_entry_data: &ProvidedEntryData,
) -> Lib3hResult<ClientToLib3hResponse> {
    tracer(&log_context, "publish_entry");

}
