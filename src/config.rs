use crate::error::QPayError;

/// QPay client configuration.
#[derive(Debug, Clone)]
pub struct QPayConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub invoice_code: String,
    pub callback_url: String,
}

impl QPayConfig {
    /// Create a new QPayConfig with the given values.
    pub fn new(
        base_url: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
        invoice_code: impl Into<String>,
        callback_url: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            username: username.into(),
            password: password.into(),
            invoice_code: invoice_code.into(),
            callback_url: callback_url.into(),
        }
    }

    /// Load configuration from environment variables.
    ///
    /// Required variables:
    /// - `QPAY_BASE_URL`
    /// - `QPAY_USERNAME`
    /// - `QPAY_PASSWORD`
    /// - `QPAY_INVOICE_CODE`
    /// - `QPAY_CALLBACK_URL`
    pub fn from_env() -> Result<Self, QPayError> {
        let base_url = require_env("QPAY_BASE_URL")?;
        let username = require_env("QPAY_USERNAME")?;
        let password = require_env("QPAY_PASSWORD")?;
        let invoice_code = require_env("QPAY_INVOICE_CODE")?;
        let callback_url = require_env("QPAY_CALLBACK_URL")?;

        Ok(Self {
            base_url,
            username,
            password,
            invoice_code,
            callback_url,
        })
    }
}

fn require_env(name: &str) -> Result<String, QPayError> {
    std::env::var(name).map_err(|_| {
        QPayError::Config(format!(
            "required environment variable {} is not set",
            name
        ))
    })
}
