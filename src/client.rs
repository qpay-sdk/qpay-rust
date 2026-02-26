use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use tokio::sync::Mutex;

use crate::config::QPayConfig;
use crate::error::{ApiErrorBody, QPayError};
use crate::models::TokenResponse;

const TOKEN_BUFFER_SECONDS: i64 = 30;

pub(crate) struct TokenState {
    pub(crate) access_token: String,
    pub(crate) refresh_token: String,
    pub(crate) expires_at: i64,
    pub(crate) refresh_expires_at: i64,
}

impl Default for TokenState {
    fn default() -> Self {
        Self {
            access_token: String::new(),
            refresh_token: String::new(),
            expires_at: 0,
            refresh_expires_at: 0,
        }
    }
}

/// QPay API client with automatic token management.
pub struct QPayClient {
    pub(crate) config: QPayConfig,
    pub(crate) http: reqwest::Client,
    pub(crate) token_state: Mutex<TokenState>,
}

impl QPayClient {
    /// Create a new QPayClient with the given configuration.
    pub fn new(config: QPayConfig) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("failed to build reqwest client");

        Self {
            config,
            http,
            token_state: Mutex::new(TokenState::default()),
        }
    }

    /// Create a new QPayClient with a custom reqwest::Client.
    pub fn with_http_client(config: QPayConfig, http: reqwest::Client) -> Self {
        Self {
            config,
            http,
            token_state: Mutex::new(TokenState::default()),
        }
    }

    /// Ensure a valid access token is available, refreshing or re-authenticating as needed.
    pub(crate) async fn ensure_token(&self) -> Result<(), QPayError> {
        let now = chrono_now();

        let (needs_refresh, can_refresh, refresh_tok) = {
            let state = self.token_state.lock().await;
            let token_valid =
                !state.access_token.is_empty() && now < state.expires_at - TOKEN_BUFFER_SECONDS;
            if token_valid {
                return Ok(());
            }
            let can_refresh = !state.refresh_token.is_empty()
                && now < state.refresh_expires_at - TOKEN_BUFFER_SECONDS;
            (true, can_refresh, state.refresh_token.clone())
        };

        if needs_refresh && can_refresh {
            if let Ok(token) = self.do_refresh_token_http(&refresh_tok).await {
                let mut state = self.token_state.lock().await;
                store_token(&mut state, &token);
                return Ok(());
            }
            // Refresh failed, fall through to full auth
        }

        // Get a new token via basic auth
        let token = self
            .get_token_request()
            .await
            .map_err(|e| QPayError::Token(e.to_string()))?;

        let mut state = self.token_state.lock().await;
        store_token(&mut state, &token);
        Ok(())
    }

    /// Perform token refresh via HTTP (without holding lock).
    pub(crate) async fn do_refresh_token_http(
        &self,
        refresh_tok: &str,
    ) -> Result<TokenResponse, QPayError> {
        let url = format!("{}/v2/auth/refresh", self.config.base_url);

        let resp = self
            .http
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", refresh_tok))
            .send()
            .await?;

        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            let api_err = serde_json::from_str::<ApiErrorBody>(&body).unwrap_or_default();
            return Err(QPayError::Api {
                status_code: status.as_u16(),
                code: api_err.code,
                message: api_err.message,
                raw_body: body,
            });
        }

        let token: TokenResponse = serde_json::from_str(&body)?;
        Ok(token)
    }

    /// Get a new token using basic auth credentials.
    pub(crate) async fn get_token_request(&self) -> Result<TokenResponse, QPayError> {
        let url = format!("{}/v2/auth/token", self.config.base_url);

        let resp = self
            .http
            .post(&url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .send()
            .await?;

        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            let api_err = serde_json::from_str::<ApiErrorBody>(&body).unwrap_or_default();
            let code = if api_err.code.is_empty() {
                status
                    .canonical_reason()
                    .unwrap_or("Unknown")
                    .to_string()
            } else {
                api_err.code
            };
            let message = if api_err.message.is_empty() {
                body.clone()
            } else {
                api_err.message
            };
            return Err(QPayError::Api {
                status_code: status.as_u16(),
                code,
                message,
                raw_body: body,
            });
        }

        let token: TokenResponse = serde_json::from_str(&body)?;
        Ok(token)
    }

    /// Store the token response in the client state.
    pub(crate) async fn store_token_response(&self, token: &TokenResponse) {
        let mut state = self.token_state.lock().await;
        store_token(&mut state, token);
    }

    /// Make an authenticated JSON request to the QPay API.
    pub(crate) async fn do_request<B: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<R, QPayError> {
        self.ensure_token().await?;

        let url = format!("{}{}", self.config.base_url, path);

        let access_token = {
            let state = self.token_state.lock().await;
            state.access_token.clone()
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token))
                .map_err(|e| QPayError::Config(e.to_string()))?,
        );

        let mut request = self.http.request(method, &url).headers(headers);

        if let Some(b) = body {
            request = request.json(b);
        }

        let resp = request.send().await?;
        let status = resp.status();
        let resp_body = resp.text().await?;

        if !status.is_success() {
            let api_err = serde_json::from_str::<ApiErrorBody>(&resp_body).unwrap_or_default();
            let code = if api_err.code.is_empty() {
                status
                    .canonical_reason()
                    .unwrap_or("Unknown")
                    .to_string()
            } else {
                api_err.code
            };
            let message = if api_err.message.is_empty() {
                resp_body.clone()
            } else {
                api_err.message
            };
            return Err(QPayError::Api {
                status_code: status.as_u16(),
                code,
                message,
                raw_body: resp_body,
            });
        }

        let result: R = serde_json::from_str(&resp_body)?;
        Ok(result)
    }

    /// Make an authenticated request that returns no body (e.g., DELETE).
    pub(crate) async fn do_request_no_response<B: serde::Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<(), QPayError> {
        self.ensure_token().await?;

        let url = format!("{}{}", self.config.base_url, path);

        let access_token = {
            let state = self.token_state.lock().await;
            state.access_token.clone()
        };

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", access_token))
                .map_err(|e| QPayError::Config(e.to_string()))?,
        );

        let mut request = self.http.request(method, &url).headers(headers);

        if let Some(b) = body {
            request = request.json(b);
        }

        let resp = request.send().await?;
        let status = resp.status();
        let resp_body = resp.text().await?;

        if !status.is_success() {
            let api_err = serde_json::from_str::<ApiErrorBody>(&resp_body).unwrap_or_default();
            let code = if api_err.code.is_empty() {
                status
                    .canonical_reason()
                    .unwrap_or("Unknown")
                    .to_string()
            } else {
                api_err.code
            };
            let message = if api_err.message.is_empty() {
                resp_body.clone()
            } else {
                api_err.message
            };
            return Err(QPayError::Api {
                status_code: status.as_u16(),
                code,
                message,
                raw_body: resp_body,
            });
        }

        Ok(())
    }
}

fn store_token(state: &mut TokenState, token: &TokenResponse) {
    state.access_token = token.access_token.clone();
    state.refresh_token = token.refresh_token.clone();
    state.expires_at = token.expires_in;
    state.refresh_expires_at = token.refresh_expires_in;
}

fn chrono_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
