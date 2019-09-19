use rusoto_core::RusotoError;
use rusoto_dynamodb::GetItemError;
use rusoto_dynamodb::DescribeTableError;

#[derive(Debug)]
pub enum BbDhtError {
    InternalServerError(String),
    ProvisionedThroughputExceeded(String),
    RequestLimitExceeded(String),
    ResourceNotFound(String),
    HttpDispatch(String),
    Credentials(String),
    Validation(String),
    ParseError(String),
    Unknown(String),
}

impl From<RusotoError<GetItemError>> for BbDhtError {

    fn from(rusoto_error: RusotoError<GetItemError>) -> Self {
        match rusoto_error {
            RusotoError::Service(service_error) => {
                match service_error {
                    GetItemError::InternalServerError(err) => BbDhtError::InternalServerError(err),
                    GetItemError::RequestLimitExceeded(err) => BbDhtError::RequestLimitExceeded(err),
                    GetItemError::ProvisionedThroughputExceeded(err) => BbDhtError::ProvisionedThroughputExceeded(err),
                    GetItemError::ResourceNotFound(err) => BbDhtError::ResourceNotFound(err),
                }
            },
            RusotoError::HttpDispatch(err) => BbDhtError::HttpDispatch(err.to_string()),
            RusotoError::Credentials(err) => BbDhtError::Credentials(err.to_string()),
            RusotoError::Validation(err) => BbDhtError::Validation(err.to_string()),
            RusotoError::ParseError(err) => BbDhtError::ParseError(err.to_string()),
            RusotoError::Unknown(err) => BbDhtError::Unknown(format!("{:?}", err)),
        }
    }
}

impl From<RusotoError<DescribeTableError>> for BbDhtError {

    fn from(rusoto_error: RusotoError<DescribeTableError>) -> Self {
        match rusoto_error {
            RusotoError::Service(service_error) => {
                match service_error {
                    DescribeTableError::InternalServerError(err) => BbDhtError::InternalServerError(err),
                    DescribeTableError::ResourceNotFound(err) => BbDhtError::ResourceNotFound(err),
                }
            },
            RusotoError::HttpDispatch(err) => BbDhtError::HttpDispatch(err.to_string()),
            RusotoError::Credentials(err) => BbDhtError::Credentials(err.to_string()),
            RusotoError::Validation(err) => BbDhtError::Validation(err.to_string()),
            RusotoError::ParseError(err) => BbDhtError::ParseError(err.to_string()),
            RusotoError::Unknown(err) => BbDhtError::Unknown(format!("{:?}", err)),
        }
    }
}
