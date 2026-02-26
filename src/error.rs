use serde::Deserialize;

/// QPay API error.
#[derive(Debug, thiserror::Error)]
pub enum QPayError {
    /// HTTP/network error from reqwest.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Configuration error (missing environment variable, etc.).
    #[error("config error: {0}")]
    Config(String),

    /// QPay API returned an error response.
    #[error("qpay: {code} - {message} (status {status_code})")]
    Api {
        status_code: u16,
        code: String,
        message: String,
        raw_body: String,
    },

    /// Token acquisition failed.
    #[error("failed to get token: {0}")]
    Token(String),
}

/// Helper struct for deserializing QPay error JSON responses.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct ApiErrorBody {
    #[serde(default, alias = "error")]
    pub code: String,
    #[serde(default)]
    pub message: String,
}

/// Check if an error is a QPay API error and extract its fields.
pub fn is_qpay_error(err: &QPayError) -> Option<(u16, &str, &str)> {
    match err {
        QPayError::Api {
            status_code,
            code,
            message,
            ..
        } => Some((*status_code, code.as_str(), message.as_str())),
        _ => None,
    }
}

// Error code constants
pub const ERR_ACCOUNT_BANK_DUPLICATED: &str = "ACCOUNT_BANK_DUPLICATED";
pub const ERR_ACCOUNT_SELECTION_INVALID: &str = "ACCOUNT_SELECTION_INVALID";
pub const ERR_AUTHENTICATION_FAILED: &str = "AUTHENTICATION_FAILED";
pub const ERR_BANK_ACCOUNT_NOT_FOUND: &str = "BANK_ACCOUNT_NOTFOUND";
pub const ERR_BANK_MCC_ALREADY_ADDED: &str = "BANK_MCC_ALREADY_ADDED";
pub const ERR_BANK_MCC_NOT_FOUND: &str = "BANK_MCC_NOT_FOUND";
pub const ERR_CARD_TERMINAL_NOT_FOUND: &str = "CARD_TERMINAL_NOTFOUND";
pub const ERR_CLIENT_NOT_FOUND: &str = "CLIENT_NOTFOUND";
pub const ERR_CLIENT_USERNAME_DUPLICATED: &str = "CLIENT_USERNAME_DUPLICATED";
pub const ERR_CUSTOMER_DUPLICATE: &str = "CUSTOMER_DUPLICATE";
pub const ERR_CUSTOMER_NOT_FOUND: &str = "CUSTOMER_NOTFOUND";
pub const ERR_CUSTOMER_REGISTER_INVALID: &str = "CUSTOMER_REGISTER_INVALID";
pub const ERR_EBARIMT_CANCEL_NOT_SUPPORTED: &str = "EBARIMT_CANCEL_NOTSUPPERDED";
pub const ERR_EBARIMT_NOT_REGISTERED: &str = "EBARIMT_NOT_REGISTERED";
pub const ERR_EBARIMT_QR_CODE_INVALID: &str = "EBARIMT_QR_CODE_INVALID";
pub const ERR_INFORM_NOT_FOUND: &str = "INFORM_NOTFOUND";
pub const ERR_INPUT_CODE_REGISTERED: &str = "INPUT_CODE_REGISTERED";
pub const ERR_INPUT_NOT_FOUND: &str = "INPUT_NOTFOUND";
pub const ERR_INVALID_AMOUNT: &str = "INVALID_AMOUNT";
pub const ERR_INVALID_OBJECT_TYPE: &str = "INVALID_OBJECT_TYPE";
pub const ERR_INVOICE_ALREADY_CANCELED: &str = "INVOICE_ALREADY_CANCELED";
pub const ERR_INVOICE_CODE_INVALID: &str = "INVOICE_CODE_INVALID";
pub const ERR_INVOICE_CODE_REGISTERED: &str = "INVOICE_CODE_REGISTERED";
pub const ERR_INVOICE_LINE_REQUIRED: &str = "INVOICE_LINE_REQUIRED";
pub const ERR_INVOICE_NOT_FOUND: &str = "INVOICE_NOTFOUND";
pub const ERR_INVOICE_PAID: &str = "INVOICE_PAID";
pub const ERR_INVOICE_RECEIVER_DATA_ADDR_REQ: &str = "INVOICE_RECEIVER_DATA_ADDRESS_REQUIRED";
pub const ERR_INVOICE_RECEIVER_DATA_EMAIL_REQ: &str = "INVOICE_RECEIVER_DATA_EMAIL_REQUIRED";
pub const ERR_INVOICE_RECEIVER_DATA_PHONE_REQ: &str = "INVOICE_RECEIVER_DATA_PHONE_REQUIRED";
pub const ERR_INVOICE_RECEIVER_DATA_REQUIRED: &str = "INVOICE_RECEIVER_DATA_REQUIRED";
pub const ERR_MAX_AMOUNT_ERR: &str = "MAX_AMOUNT_ERR";
pub const ERR_MCC_NOT_FOUND: &str = "MCC_NOTFOUND";
pub const ERR_MERCHANT_ALREADY_REGISTERED: &str = "MERCHANT_ALREADY_REGISTERED";
pub const ERR_MERCHANT_INACTIVE: &str = "MERCHANT_INACTIVE";
pub const ERR_MERCHANT_NOT_FOUND: &str = "MERCHANT_NOTFOUND";
pub const ERR_MIN_AMOUNT_ERR: &str = "MIN_AMOUNT_ERR";
pub const ERR_NO_CREDENTIALS: &str = "NO_CREDENDIALS";
pub const ERR_OBJECT_DATA_ERROR: &str = "OBJECT_DATA_ERROR";
pub const ERR_P2P_TERMINAL_NOT_FOUND: &str = "P2P_TERMINAL_NOTFOUND";
pub const ERR_PAYMENT_ALREADY_CANCELED: &str = "PAYMENT_ALREADY_CANCELED";
pub const ERR_PAYMENT_NOT_PAID: &str = "PAYMENT_NOT_PAID";
pub const ERR_PAYMENT_NOT_FOUND: &str = "PAYMENT_NOTFOUND";
pub const ERR_PERMISSION_DENIED: &str = "PERMISSION_DENIED";
pub const ERR_QR_ACCOUNT_INACTIVE: &str = "QRACCOUNT_INACTIVE";
pub const ERR_QR_ACCOUNT_NOT_FOUND: &str = "QRACCOUNT_NOTFOUND";
pub const ERR_QR_CODE_NOT_FOUND: &str = "QRCODE_NOTFOUND";
pub const ERR_QR_CODE_USED: &str = "QRCODE_USED";
pub const ERR_SENDER_BRANCH_DATA_REQUIRED: &str = "SENDER_BRANCH_DATA_REQUIRED";
pub const ERR_TAX_LINE_REQUIRED: &str = "TAX_LINE_REQUIRED";
pub const ERR_TAX_PRODUCT_CODE_REQUIRED: &str = "TAX_PRODUCT_CODE_REQUIRED";
pub const ERR_TRANSACTION_NOT_APPROVED: &str = "TRANSACTION_NOT_APPROVED";
pub const ERR_TRANSACTION_REQUIRED: &str = "TRANSACTION_REQUIRED";
