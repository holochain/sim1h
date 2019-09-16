pub fn client (region: Region) -> DynamoDbClient {
    DynamoDbClient::new(region).with_retries(Policy::default())
}
