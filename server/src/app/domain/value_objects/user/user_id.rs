use ddd_macros::ValueObject;
use serde::{Deserialize, Serialize};

#[derive(ValueObject, Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[Id]
pub struct UserId {
    value: Option<String>,
}
