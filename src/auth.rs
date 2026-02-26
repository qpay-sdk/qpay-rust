use crate::client::QPayClient;
use crate::error::QPayError;
use crate::models::TokenResponse;

impl QPayClient {
    /// Authenticate with QPay using Basic Auth and return a new token pair.
    /// The token is also stored in the client for subsequent requests.
    pub async fn get_token(&self) -> Result<TokenResponse, QPayError> {
        let token = self.get_token_request().await?;
        self.store_token_response(&token).await;
        Ok(token)
    }

    /// Use the current refresh token to obtain a new access token.
    /// The new token is stored in the client for subsequent requests.
    pub async fn refresh_token(&self) -> Result<TokenResponse, QPayError> {
        let refresh_tok = {
            // We need to read the current refresh token; store_token_response uses lock internally
            // but we access token_state through ensure_token path. Instead, do a full refresh cycle.
            // We'll call do_refresh_token_http which doesn't hold the lock.
            let state = self.token_state.lock().await;
            state.refresh_token.clone()
        };

        let token = self.do_refresh_token_http(&refresh_tok).await?;
        self.store_token_response(&token).await;
        Ok(token)
    }
}
