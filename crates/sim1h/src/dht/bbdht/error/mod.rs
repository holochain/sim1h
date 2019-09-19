use rusoto_core::RusotoError;
use rusoto_dynamodb::DescribeTableError;
use rusoto_dynamodb::GetItemError;
use std::num::ParseIntError;
use lib3h::error::Lib3hError;

#[derive(Debug)]
pub enum BbDhtError {
    // Rusoto mappings
    InternalServerError(String),
    ProvisionedThroughputExceeded(String),
    RequestLimitExceeded(String),
    ResourceNotFound(String),
    HttpDispatch(String),
    Credentials(String),
    Validation(String),
    ParseError(String),
    Unknown(String),
    // data handling
    MissingData(String),
    CorruptData(String),
}

impl ToString for BbDhtError {
    fn to_string(&self) -> String {
        match self {
            BbDhtError::InternalServerError(s) => s,
            BbDhtError::ProvisionedThroughputExceeded(s) => s,
            BbDhtError::RequestLimitExceeded(s) => s,
            BbDhtError::ResourceNotFound(s) => s,
            BbDhtError::HttpDispatch(s) => s,
            BbDhtError::Credentials(s) => s,
            BbDhtError::Validation(s) => s,
            BbDhtError::ParseError(s) => s,
            BbDhtError::Unknown(s) => s,
            BbDhtError::MissingData(s) => s,
            BbDhtError::CorruptData(s) => s,
        }.to_string()
    }
}

pub type BbDhtResult<T> = Result<T, BbDhtError>;

impl From<ParseIntError> for BbDhtError {
    fn from(int_error: ParseIntError) -> Self {
        BbDhtError::CorruptData(int_error.to_string())
    }
}

impl From<BbDhtError> for Lib3hError {
    fn from(bb_dht_error: BbDhtError) -> Self {
        Lib3hError::from(bb_dht_error.to_string())
    }
}

impl From<RusotoError<GetItemError>> for BbDhtError {
    fn from(rusoto_error: RusotoError<GetItemError>) -> Self {
        match rusoto_error {
            RusotoError::Service(service_error) => match service_error {
                GetItemError::InternalServerError(err) => BbDhtError::InternalServerError(err),
                GetItemError::RequestLimitExceeded(err) => BbDhtError::RequestLimitExceeded(err),
                GetItemError::ProvisionedThroughputExceeded(err) => {
                    BbDhtError::ProvisionedThroughputExceeded(err)
                }
                GetItemError::ResourceNotFound(err) => BbDhtError::ResourceNotFound(err),
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
            RusotoError::Service(service_error) => match service_error {
                DescribeTableError::InternalServerError(err) => {
                    BbDhtError::InternalServerError(err)
                }
                DescribeTableError::ResourceNotFound(err) => BbDhtError::ResourceNotFound(err),
            },
            RusotoError::HttpDispatch(err) => BbDhtError::HttpDispatch(err.to_string()),
            RusotoError::Credentials(err) => BbDhtError::Credentials(err.to_string()),
            RusotoError::Validation(err) => BbDhtError::Validation(err.to_string()),
            RusotoError::ParseError(err) => BbDhtError::ParseError(err.to_string()),
            RusotoError::Unknown(err) => BbDhtError::Unknown(format!("{:?}", err)),
        }
    }
}
