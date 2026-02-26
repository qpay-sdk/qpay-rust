use qpay::{is_qpay_error, QPayError};

#[test]
fn test_api_error_display() {
    let err = QPayError::Api {
        status_code: 400,
        code: "INVOICE_NOTFOUND".to_string(),
        message: "Invoice not found".to_string(),
        raw_body: r#"{"code":"INVOICE_NOTFOUND","message":"Invoice not found"}"#.to_string(),
    };

    let display = err.to_string();
    assert!(
        display.contains("INVOICE_NOTFOUND"),
        "display should contain error code: {}",
        display
    );
    assert!(
        display.contains("Invoice not found"),
        "display should contain message: {}",
        display
    );
    assert!(
        display.contains("400"),
        "display should contain status code: {}",
        display
    );
}

#[test]
fn test_config_error_display() {
    let err = QPayError::Config("missing QPAY_BASE_URL".to_string());
    assert!(err.to_string().contains("missing QPAY_BASE_URL"));
}

#[test]
fn test_token_error_display() {
    let err = QPayError::Token("authentication failed".to_string());
    let display = err.to_string();
    assert!(
        display.contains("authentication failed"),
        "display: {}",
        display
    );
}

#[test]
fn test_json_error_display() {
    let json_err = serde_json::from_str::<serde_json::Value>("not valid json").unwrap_err();
    let err = QPayError::Json(json_err);
    let display = err.to_string();
    assert!(
        display.contains("json error"),
        "display should contain 'json error': {}",
        display
    );
}

#[test]
fn test_is_qpay_error_with_api_error() {
    let err = QPayError::Api {
        status_code: 404,
        code: "INVOICE_NOTFOUND".to_string(),
        message: "Invoice not found".to_string(),
        raw_body: "{}".to_string(),
    };

    let result = is_qpay_error(&err);
    assert!(result.is_some());

    let (status, code, message) = result.unwrap();
    assert_eq!(status, 404);
    assert_eq!(code, "INVOICE_NOTFOUND");
    assert_eq!(message, "Invoice not found");
}

#[test]
fn test_is_qpay_error_with_config_error() {
    let err = QPayError::Config("test".to_string());
    assert!(is_qpay_error(&err).is_none());
}

#[test]
fn test_is_qpay_error_with_token_error() {
    let err = QPayError::Token("test".to_string());
    assert!(is_qpay_error(&err).is_none());
}

#[test]
fn test_is_qpay_error_with_json_error() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
    let err = QPayError::Json(json_err);
    assert!(is_qpay_error(&err).is_none());
}

#[test]
fn test_error_debug_format() {
    let err = QPayError::Api {
        status_code: 500,
        code: "INTERNAL".to_string(),
        message: "Server error".to_string(),
        raw_body: "raw".to_string(),
    };

    let debug = format!("{:?}", err);
    assert!(debug.contains("Api"));
    assert!(debug.contains("500"));
    assert!(debug.contains("INTERNAL"));
}

#[test]
fn test_error_constants() {
    use qpay::error::*;

    assert_eq!(ERR_AUTHENTICATION_FAILED, "AUTHENTICATION_FAILED");
    assert_eq!(ERR_INVOICE_NOT_FOUND, "INVOICE_NOTFOUND");
    assert_eq!(ERR_INVOICE_PAID, "INVOICE_PAID");
    assert_eq!(ERR_INVOICE_ALREADY_CANCELED, "INVOICE_ALREADY_CANCELED");
    assert_eq!(ERR_PAYMENT_NOT_FOUND, "PAYMENT_NOTFOUND");
    assert_eq!(ERR_PAYMENT_ALREADY_CANCELED, "PAYMENT_ALREADY_CANCELED");
    assert_eq!(ERR_PAYMENT_NOT_PAID, "PAYMENT_NOT_PAID");
    assert_eq!(ERR_PERMISSION_DENIED, "PERMISSION_DENIED");
    assert_eq!(ERR_INVALID_AMOUNT, "INVALID_AMOUNT");
    assert_eq!(ERR_NO_CREDENTIALS, "NO_CREDENDIALS");
    assert_eq!(ERR_MERCHANT_NOT_FOUND, "MERCHANT_NOTFOUND");
    assert_eq!(ERR_MERCHANT_INACTIVE, "MERCHANT_INACTIVE");
    assert_eq!(ERR_CUSTOMER_NOT_FOUND, "CUSTOMER_NOTFOUND");
    assert_eq!(ERR_CUSTOMER_DUPLICATE, "CUSTOMER_DUPLICATE");
    assert_eq!(ERR_EBARIMT_NOT_REGISTERED, "EBARIMT_NOT_REGISTERED");
    assert_eq!(ERR_EBARIMT_CANCEL_NOT_SUPPORTED, "EBARIMT_CANCEL_NOTSUPPERDED");
    assert_eq!(ERR_INVOICE_CODE_INVALID, "INVOICE_CODE_INVALID");
    assert_eq!(ERR_INVOICE_LINE_REQUIRED, "INVOICE_LINE_REQUIRED");
    assert_eq!(ERR_TAX_LINE_REQUIRED, "TAX_LINE_REQUIRED");
    assert_eq!(ERR_TAX_PRODUCT_CODE_REQUIRED, "TAX_PRODUCT_CODE_REQUIRED");
    assert_eq!(ERR_TRANSACTION_REQUIRED, "TRANSACTION_REQUIRED");
}

#[test]
fn test_api_error_with_empty_fields() {
    let err = QPayError::Api {
        status_code: 0,
        code: String::new(),
        message: String::new(),
        raw_body: String::new(),
    };

    let result = is_qpay_error(&err);
    assert!(result.is_some());
    let (status, code, message) = result.unwrap();
    assert_eq!(status, 0);
    assert_eq!(code, "");
    assert_eq!(message, "");
}

#[test]
fn test_json_error_from_conversion() {
    let json_err = serde_json::from_str::<serde_json::Value>("{bad}").unwrap_err();
    let err: QPayError = json_err.into();
    match err {
        QPayError::Json(_) => {} // expected
        _ => panic!("expected QPayError::Json"),
    }
}
