
/// Represents the response structure returned by the Ruxios client.
#[derive(Debug, serde::Deserialize)]
pub struct AxiosResponse<T> {
    /// HTTP status code of the response.
    pub status: u16,
    /// Actual data content of the response.
    pub data: T,
}

impl<T> AxiosResponse<T> {
    /// Checks if the response status is a success (2xx range).
    pub fn ok(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

impl AxiosResponse<serde_json::Value> {
    /// Converts the response data from JSON to the given type.
    pub fn json<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.data.clone())
    }
}

impl<T> TryConvert<T> for AxiosResponse<serde_json::Value>
where
    T: serde::de::DeserializeOwned,
{
    fn try_convert(self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.data)
    }
}


pub trait TryConvert<T: serde::de::DeserializeOwned> {
    fn try_convert(self) -> Result<T, serde_json::Error>;
}