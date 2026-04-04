pub mod models;

use crate::auth::token_store::{self, Credentials};
use crate::error::CliError;
use anyhow::Result;
use models::*;
use reqwest::blocking::{Client, RequestBuilder, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const APIM_KEY_HEADER: &str = "Ocp-Apim-Subscription-Key";

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .user_agent(concat!("todomate-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client");
        ApiClient { client, base_url }
    }

    // --- Auth endpoints (no Bearer token required) ---

    pub fn exchange_github_token(
        &self,
        github_token: &str,
        subscription_key: &str,
    ) -> Result<AuthResponse> {
        let resp = self
            .client
            .post(format!("{}/v1/auth/token", self.base_url))
            .header(CONTENT_TYPE, "application/json")
            .header(APIM_KEY_HEADER, subscription_key)
            .json(&AuthTokenRequest {
                github_token: github_token.to_string(),
            })
            .send()?;
        parse_response(resp)
    }

    fn refresh_jwt(&self, refresh_token: &str, subscription_key: &str) -> Result<AuthResponse> {
        let resp = self
            .client
            .post(format!("{}/v1/auth/refresh", self.base_url))
            .header(CONTENT_TYPE, "application/json")
            .header(APIM_KEY_HEADER, subscription_key)
            .json(&RefreshRequest {
                refresh_token: refresh_token.to_string(),
            })
            .send()?;
        parse_response(resp)
    }

    // --- Authenticated request dispatch ---

    fn execute<F>(&self, build: F) -> Result<Response>
    where
        F: Fn(&str, &str) -> RequestBuilder,
    {
        let creds = token_store::load()?;
        let resp = build(&creds.jwt, &creds.subscription_key).send()?;

        if resp.status().as_u16() == 401 {
            // Attempt silent token refresh
            let refreshed = self
                .refresh_jwt(&creds.refresh_token, &creds.subscription_key)
                .map_err(|_| CliError::RefreshFailed)?;

            token_store::save(&Credentials {
                jwt: refreshed.access_token.clone(),
                refresh_token: refreshed.refresh_token.clone(),
                subscription_key: creds.subscription_key.clone(),
            })?;

            let retry = build(&refreshed.access_token, &creds.subscription_key).send()?;
            return Ok(retry);
        }

        Ok(resp)
    }

    fn auth_headers(&self, jwt: &str, subscription_key: &str) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {jwt}")).expect("invalid JWT characters"),
        );
        map.insert(
            APIM_KEY_HEADER,
            HeaderValue::from_str(subscription_key).expect("invalid subscription key characters"),
        );
        map
    }

    // --- Todos ---

    pub fn list_todos(
        &self,
        completed: Option<bool>,
        priority: Option<&str>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Todo>> {
        let resp = self.execute(|jwt, key| {
            let mut req = self
                .client
                .get(format!("{}/v1/todos", self.base_url))
                .headers(self.auth_headers(jwt, key));
            if let Some(c) = completed {
                req = req.query(&[("completed", c.to_string())]);
            }
            if let Some(p) = priority {
                req = req.query(&[("priority", p)]);
            }
            if let Some(l) = limit {
                req = req.query(&[("limit", l.to_string())]);
            }
            if let Some(o) = offset {
                req = req.query(&[("offset", o.to_string())]);
            }
            req
        })?;
        parse_response(resp)
    }

    pub fn create_todo(&self, body: CreateTodoRequest) -> Result<Todo> {
        let resp = self.execute(|jwt, key| {
            self.client
                .post(format!("{}/v1/todos", self.base_url))
                .headers(self.auth_headers(jwt, key))
                .header(CONTENT_TYPE, "application/json")
                .json(&body)
        })?;
        parse_response(resp)
    }

    pub fn update_todo(&self, id: &str, body: UpdateTodoRequest) -> Result<Todo> {
        let resp = self.execute(|jwt, key| {
            self.client
                .put(format!("{}/v1/todos/{id}", self.base_url))
                .headers(self.auth_headers(jwt, key))
                .header(CONTENT_TYPE, "application/json")
                .json(&body)
        })?;
        parse_response(resp)
    }

    pub fn delete_todo(&self, id: &str) -> Result<DeleteResponse> {
        let resp = self.execute(|jwt, key| {
            self.client
                .delete(format!("{}/v1/todos/{id}", self.base_url))
                .headers(self.auth_headers(jwt, key))
        })?;
        parse_response(resp)
    }

    // --- Goals ---

    pub fn list_goals(&self) -> Result<Vec<Goal>> {
        let resp = self.execute(|jwt, key| {
            self.client
                .get(format!("{}/v1/goals", self.base_url))
                .headers(self.auth_headers(jwt, key))
        })?;
        parse_response(resp)
    }

    pub fn create_goal(&self, body: CreateGoalRequest) -> Result<Goal> {
        let resp = self.execute(|jwt, key| {
            self.client
                .post(format!("{}/v1/goals", self.base_url))
                .headers(self.auth_headers(jwt, key))
                .header(CONTENT_TYPE, "application/json")
                .json(&body)
        })?;
        parse_response(resp)
    }

    pub fn update_goal(&self, id: &str, body: UpdateGoalRequest) -> Result<Goal> {
        let resp = self.execute(|jwt, key| {
            self.client
                .put(format!("{}/v1/goals/{id}", self.base_url))
                .headers(self.auth_headers(jwt, key))
                .header(CONTENT_TYPE, "application/json")
                .json(&body)
        })?;
        parse_response(resp)
    }

    pub fn delete_goal(&self, id: &str) -> Result<DeleteResponse> {
        let resp = self.execute(|jwt, key| {
            self.client
                .delete(format!("{}/v1/goals/{id}", self.base_url))
                .headers(self.auth_headers(jwt, key))
        })?;
        parse_response(resp)
    }

    // --- Vision ---

    pub fn get_vision(&self) -> Result<Vision> {
        let resp = self.execute(|jwt, key| {
            self.client
                .get(format!("{}/v1/vision", self.base_url))
                .headers(self.auth_headers(jwt, key))
        })?;
        parse_response(resp)
    }

    pub fn update_vision(&self, description: &str) -> Result<Vision> {
        let resp = self.execute(|jwt, key| {
            self.client
                .put(format!("{}/v1/vision", self.base_url))
                .headers(self.auth_headers(jwt, key))
                .header(CONTENT_TYPE, "application/json")
                .json(&UpdateVisionRequest {
                    description: description.to_string(),
                })
        })?;
        parse_response(resp)
    }
}

fn parse_response<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T> {
    let status = resp.status();
    if status.is_success() {
        let body: T = resp.json()?;
        return Ok(body);
    }
    // Try to extract a message from the JSON body
    let text = resp.text().unwrap_or_default();
    let message = serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|v| v.get("message").and_then(|m| m.as_str()).map(String::from))
        .unwrap_or(text);
    Err(CliError::ApiError {
        status: status.as_u16(),
        message,
    }
    .into())
}
