use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DesoError {
    #[error("Problem Sending `{0}` Transaction: `{1}`")]
    TransactionError(String, String),
    #[error("Reqwest parsing text error: `{0}`")]
    ReqwestError(String),
    #[error("Deso Transaction Error: `{0}`")]
    DesoError(String),
    #[error("Serde Json Error at `{0}`: `{1}`")]
    JsonError(String, String),
    #[error("Problem With Temp Path `{0}`")]
    TempFileError(String),
    #[error("Payment Error: `{0}`")]
    PaymentError(String),
    #[error("Get Profile Error: `{0}`")]
    ProfileRequestError(String),
    #[error("Problem Getting Index: `{0}`")]
    SigningError(String),
}
