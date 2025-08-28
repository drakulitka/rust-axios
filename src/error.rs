use thiserror::Error;

#[derive(Error, Debug, serde::Deserialize)]
pub struct AxiosErrorValue<T> {
    pub status: String,
    pub message: T,
}

#[derive(Error, Debug)]
pub enum AxiosError<T> {
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Request failed: {0}")]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),

    #[error("Deserialization failed: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Failed to get env: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("{0:?}")]
    ResponseError(AxiosErrorValue<T>),
}

impl AxiosError<serde_json::Value> {
    pub fn from_string<T>(body: T) -> AxiosError<serde_json::Value>
    where
        T: AsRef<str>,
    {
        let body = serde_json::json!({
      "message": body.as_ref(),
    });

        Self::ResponseError(AxiosErrorValue {
            status: "".to_string(),
            message: body,
        })
    }

    pub fn from_value<T>(body: T) -> AxiosError<serde_json::Value>
    where
        T: Into<serde_json::Value>,
    {
        let body = body.into();

        Self::ResponseError(AxiosErrorValue {
            status: "".to_string(),
            message: body,
        })
    }
}