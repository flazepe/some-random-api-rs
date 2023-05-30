use anyhow::{bail, Result};
use reqwest::{Client as ReqwestClient, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::from_str;

static BASE_URL: &str = "https://some-random-api.com";

#[derive(Clone)]
pub struct Requester {
    pub reqwest: ReqwestClient,
    pub api_key: Option<String>,
}

impl Requester {
    pub fn new<T: ToString>(api_key: Option<T>) -> Self {
        Self {
            reqwest: ReqwestClient::new(),
            api_key: api_key.map(|api_key| api_key.to_string()),
        }
    }

    /// Parses error or response from a JSON string
    fn parse_json<T: DeserializeOwned>(string: String) -> Result<T> {
        if let Ok(error) = from_str::<Error>(string.as_str()) {
            bail!(error.message);
        }

        match from_str(string.as_str()) {
            Ok(json) => Ok(json),
            Err(_) => bail!(format!("Unexpected response: {string}")),
        }
    }

    /// Creates a request with an optional API key
    fn create_request<T: ToString>(&self, endpoint: T) -> Result<RequestBuilder> {
        let endpoint = endpoint.to_string();
        let mut request = self.reqwest.get(format!("{BASE_URL}/{endpoint}"));

        match self.api_key.as_ref() {
            Some(api_key) => request = request.query(&[("key", api_key)]),
            None => {
                if endpoint.contains("premium") || endpoint.contains("chatbot") {
                    bail!("An API key is required for this endpoint");
                }
            }
        }

        Ok(request)
    }

    /// Sends a request with optional query
    pub async fn request<T: ToString, U: Serialize + ?Sized, V: DeserializeOwned>(
        &self,
        endpoint: T,
        query: Option<&U>,
    ) -> Result<V> {
        let mut request = self.create_request(endpoint)?;

        if let Some(query) = query {
            request = request.query(query);
        }

        Self::parse_json(request.send().await?.text().await?)
    }

    /// Sends an image request with query
    pub async fn request_image<T: ToString, U: Serialize + ?Sized>(
        &self,
        endpoint: T,
        query: &U,
    ) -> Result<Vec<u8>> {
        let response = self.create_request(endpoint)?.query(query).send().await?;
        let status_code = response.status();

        if status_code != 200 {
            let string = response.text().await?;

            match from_str::<Error>(string.as_str()) {
                Ok(error) => bail!(error.message),
                Err(_) => bail!(format!("Unexpected response: {string}")),
            };
        }

        Ok(response.bytes().await?.to_vec())
    }
}

#[derive(Deserialize)]
struct Error {
    #[serde(rename = "error")]
    message: String,
}
