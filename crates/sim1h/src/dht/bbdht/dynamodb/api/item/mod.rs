use std::collections::HashMap;
use rusoto_dynamodb::AttributeValue;

pub mod fixture;
pub mod read;
pub mod write;

type Item = HashMap<String, AttributeValue>;
