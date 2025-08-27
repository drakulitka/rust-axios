use std::collections::HashMap;

#[derive(Clone)]
pub struct AxiosConfig {
    pub base_url: String,
    pub headers: HashMap<String, String>,
    pub timeout: u32,
}

impl Default for AxiosConfig {
    fn default() -> Self {
        Self {
            base_url: String::from(""),
            headers: HashMap::new(),
            timeout: 10000,
        }
    }
}

impl AxiosConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct AxiosRequestConfig {
    pub url: String,
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub _nop: (),
}

impl Default for AxiosRequestConfig {
    fn default() -> Self {
        AxiosRequestConfig {
            url: String::new(),
            method: Some(String::from("GET")),
            headers: None,
            _nop: (),
        }
    }
}