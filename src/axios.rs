use crate::config::{AxiosConfig, AxiosRequestConfig};
use crate::error::{AxiosError, AxiosErrorValue};
use crate::interceptor::InterceptorManager;
use crate::method::HttpMethod;
use crate::response::AxiosResponse;

#[derive(Clone)]
pub struct Axios {
    client: reqwest::Client,
    pub config: AxiosConfig,
    pub interceptors: InterceptorManager,
}

impl Axios {

    /// Creates a new Ruxios client with default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generalized method to perform an HTTP request.
    pub async fn request<TReq, TRes, TErr>(
        &self,
        cfg: AxiosRequestConfig,
        data: Option<&TReq>,
    ) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TReq: serde::Serialize,
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        let full_url = format!("{}{}", self.config.base_url, cfg.url);

        let client = self.client.clone();

        let mut req_builder = client
            .request(
                HttpMethod::string_to_method(cfg.method.unwrap_or("GET".to_owned())),
                full_url,
            );

        for (key, value) in self.config.headers.clone() {
            req_builder = req_builder.header(key, value);
        }

        if let Some(headers) = cfg.headers {
            for (key, value) in headers {
                req_builder = req_builder.header(key, value);
            }
        };

        let req_builder = if let Some(data) = data {
            req_builder.body(serde_json::to_string(data)?)
        } else {
            req_builder
        };

        let mut req = req_builder.build()?;

        // Выполнение перехватчиков перед отправкой запроса
        for handler in  self.interceptors.request_handlers.iter() {
            req = handler.execute(req).await;
        }

        let mut res = client.execute(req).await?;

        // Выполнение перехватчиков после получения ответа
        for handler in  self.interceptors.response_handlers.iter() {
            res = handler.execute(res).await;
        }

        let status = res.status();

        if status.is_success() {

            let response_data = res.json::<TRes>().await?;
            Ok(AxiosResponse {
                status: status.as_u16(),
                data: response_data,
            })
        } else {
            let body = res.text().await?;
            let response_data: AxiosErrorValue<TErr> = serde_json::from_str(&body)?;
            Err(AxiosError::ResponseError(response_data))
        }
    }

    fn type_of<T>(_: &T) -> String {
        format!("{:?}", std::any::type_name::<T>())
    }

    /// Performs a GET request to the specified URL.
    pub async fn get<TRes, TErr>(&self, url: &str) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<serde_json::Value, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("GET")),
                    ..Default::default()
                },
                None,
            )
            .await
    }

    /// Performs a POST request to the specified URL.
    pub async fn post<TReq, TRes, TErr>(
        &self,
        url: &str,
        data: &TReq
    ) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TReq: serde::Serialize,
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<TReq, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("POST")),
                    ..Default::default()
                },
                Some(data),
            )
            .await
    }

    /// Performs a PUT request to the specified URL.
    pub async fn put<TReq, TRes, TErr>(
        &self,
        url: &str,
        data: &TReq,
    ) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TReq: serde::Serialize,
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<TReq, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("PUT")),
                    ..Default::default()
                },
                Some(data),
            )
            .await
    }

    /// Performs a PATCH request to the specified URL.
    pub async fn patch<TReq, TRes, TErr>(
        &self,
        url: &str,
        data: &TReq,
    ) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TReq: serde::Serialize,
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<TReq, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("PATCH")),
                    ..Default::default()
                },
                Some(data),
            )
            .await
    }

    /// Performs a DELETE request to the specified URL.
    pub async fn delete<TRes, TErr>(
        &self,
        url: &str,
    ) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<serde_json::Value, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("DELETE")),
                    ..Default::default()
                },
                None,
            )
            .await
    }

    /// Performs an OPTIONS request to the specified URL.
    pub async fn options<TRes, TErr>(
        &self,
        url: &str,
    ) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<serde_json::Value, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("OPTIONS")),
                    ..Default::default()
                },
                None,
            )
            .await
    }

    /// Performs a HEAD request to the specified URL.
    pub async fn head<TRes, TErr>(&self, url: &str) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<serde_json::Value, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("HEAD")),
                    ..Default::default()
                },
                None,
            )
            .await
    }

    /// Performs a TRACE request to the specified URL.
    pub async fn trace<TRes, TErr>(
        &self,
        url: &str,
    ) -> Result<AxiosResponse<TRes>, AxiosError<TErr>>
    where
        TRes: serde::de::DeserializeOwned,
        TErr: serde::de::DeserializeOwned,
    {
        self
            .request::<serde_json::Value, TRes, TErr>(
                AxiosRequestConfig {
                    url: String::from(url),
                    method: Some(String::from("TRACE")),
                    ..Default::default()
                },
                None,
            )
            .await
    }
}

impl Default for Axios {
    /// Implements a default configuration for a Ruxios client.
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            config: AxiosConfig::new(),
            interceptors: InterceptorManager::new(),
        }
    }
}

impl From<AxiosConfig> for Axios {
    fn from(config: AxiosConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }
}
