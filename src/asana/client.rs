use super::models::*;
use anyhow::Result;
use reqwest::{Method, Response};

/// Makes requests to Asana and tries to conform response data to given model.
///
pub struct Client {
    access_token: String,
    base_url: String,
    endpoint: String,
    http_client: reqwest::Client,
}

impl Client {
    /// Returns a new instance for the given access token and base URL.
    ///
    pub fn new(access_token: &str, base_url: &str) -> Self {
        Client {
            access_token: access_token.to_owned(),
            base_url: base_url.to_owned(),
            endpoint: String::from(""),
            http_client: reqwest::Client::builder().build().unwrap(),
        }
    }

    /// Return model data for entity with GID or error.
    ///
    pub async fn get<T: Model>(&mut self, gid: &str) -> Result<T> {
        let model: Wrapper<T> = self.call::<T>(Method::GET, Some(gid)).await?.json().await?;
        Ok(model.data)
    }

    /// Return vector of model data or error.
    ///
    #[allow(dead_code)]
    pub async fn list<T: Model>(&mut self) -> Result<Vec<T>> {
        let model: ListWrapper<T> = self.call::<T>(Method::GET, None).await?.json().await?;
        self.endpoint.clear();
        Ok(model.data)
    }

    /// Prepare endpoint for relational model data.
    ///
    #[allow(dead_code)]
    pub fn from<T: Model>(&mut self, relational_gid: &str) -> &mut Client {
        self.endpoint = format!("{}/{}/", T::endpoint(), relational_gid);
        self
    }

    /// Make request and return response with model data or error.
    ///
    async fn call<T: Model>(&mut self, method: Method, gid: Option<&str>) -> Result<Response> {
        // Add both relational and main endpoints, and entity gid if supplied
        let uri = format!("{}{}/", self.endpoint, T::endpoint());
        let uri = format!(
            "{}{}",
            uri,
            match gid {
                Some(gid) => gid.to_owned(),
                None => String::from(""),
            }
        );

        // Clear relational endpoint state
        self.endpoint.clear();

        // Add relational and root field inclusions as query parameters
        let opts = format!(
            "this.({}),{}",
            T::field_names().join("|"),
            T::opt_strings().join(",")
        );
        let uri = format!("{}?opt_fields={}", uri, opts);
        let request_url = format!("{}/{}", &self.base_url, uri);

        // Make request
        Ok(self
            .http_client
            .request(method, &request_url)
            .header("Authorization", format!("Bearer {}", &self.access_token))
            .send()
            .await?)
    }
}
