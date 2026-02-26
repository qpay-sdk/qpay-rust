use qpay::{QPayConfig, QPayError};
use serial_test::serial;

#[test]
fn test_config_new() {
    let config = QPayConfig::new(
        "https://merchant.qpay.mn",
        "test_user",
        "test_pass",
        "INV_CODE",
        "https://example.com/callback",
    );

    assert_eq!(config.base_url, "https://merchant.qpay.mn");
    assert_eq!(config.username, "test_user");
    assert_eq!(config.password, "test_pass");
    assert_eq!(config.invoice_code, "INV_CODE");
    assert_eq!(config.callback_url, "https://example.com/callback");
}

#[test]
fn test_config_new_with_string_types() {
    let config = QPayConfig::new(
        String::from("https://merchant.qpay.mn"),
        String::from("user"),
        "pass",
        "CODE",
        String::from("https://cb.example.com"),
    );

    assert_eq!(config.base_url, "https://merchant.qpay.mn");
    assert_eq!(config.username, "user");
    assert_eq!(config.password, "pass");
    assert_eq!(config.invoice_code, "CODE");
    assert_eq!(config.callback_url, "https://cb.example.com");
}

#[test]
fn test_config_clone() {
    let config = QPayConfig::new(
        "https://merchant.qpay.mn",
        "user",
        "pass",
        "CODE",
        "https://cb.example.com",
    );
    let cloned = config.clone();

    assert_eq!(config.base_url, cloned.base_url);
    assert_eq!(config.username, cloned.username);
    assert_eq!(config.password, cloned.password);
    assert_eq!(config.invoice_code, cloned.invoice_code);
    assert_eq!(config.callback_url, cloned.callback_url);
}

#[test]
fn test_config_debug() {
    let config = QPayConfig::new("url", "user", "pass", "code", "callback");
    let debug = format!("{:?}", config);
    assert!(debug.contains("QPayConfig"));
    assert!(debug.contains("url"));
}

#[test]
#[serial]
fn test_config_from_env_success() {
    // Set all required environment variables
    std::env::set_var("QPAY_BASE_URL", "https://merchant.qpay.mn");
    std::env::set_var("QPAY_USERNAME", "env_user");
    std::env::set_var("QPAY_PASSWORD", "env_pass");
    std::env::set_var("QPAY_INVOICE_CODE", "ENV_CODE");
    std::env::set_var("QPAY_CALLBACK_URL", "https://env.example.com/callback");

    let config = QPayConfig::from_env().expect("should load from env");

    assert_eq!(config.base_url, "https://merchant.qpay.mn");
    assert_eq!(config.username, "env_user");
    assert_eq!(config.password, "env_pass");
    assert_eq!(config.invoice_code, "ENV_CODE");
    assert_eq!(config.callback_url, "https://env.example.com/callback");

    // Clean up
    std::env::remove_var("QPAY_BASE_URL");
    std::env::remove_var("QPAY_USERNAME");
    std::env::remove_var("QPAY_PASSWORD");
    std::env::remove_var("QPAY_INVOICE_CODE");
    std::env::remove_var("QPAY_CALLBACK_URL");
}

#[test]
#[serial]
fn test_config_from_env_missing_base_url() {
    std::env::remove_var("QPAY_BASE_URL");
    std::env::set_var("QPAY_USERNAME", "user");
    std::env::set_var("QPAY_PASSWORD", "pass");
    std::env::set_var("QPAY_INVOICE_CODE", "code");
    std::env::set_var("QPAY_CALLBACK_URL", "cb");

    let result = QPayConfig::from_env();
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("QPAY_BASE_URL"),
        "error should mention QPAY_BASE_URL: {}",
        msg
    );

    // Clean up
    std::env::remove_var("QPAY_USERNAME");
    std::env::remove_var("QPAY_PASSWORD");
    std::env::remove_var("QPAY_INVOICE_CODE");
    std::env::remove_var("QPAY_CALLBACK_URL");
}

#[test]
#[serial]
fn test_config_from_env_missing_username() {
    std::env::set_var("QPAY_BASE_URL", "url");
    std::env::remove_var("QPAY_USERNAME");
    std::env::set_var("QPAY_PASSWORD", "pass");
    std::env::set_var("QPAY_INVOICE_CODE", "code");
    std::env::set_var("QPAY_CALLBACK_URL", "cb");

    let result = QPayConfig::from_env();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("QPAY_USERNAME"));

    // Clean up
    std::env::remove_var("QPAY_BASE_URL");
    std::env::remove_var("QPAY_PASSWORD");
    std::env::remove_var("QPAY_INVOICE_CODE");
    std::env::remove_var("QPAY_CALLBACK_URL");
}

#[test]
#[serial]
fn test_config_from_env_missing_password() {
    std::env::set_var("QPAY_BASE_URL", "url");
    std::env::set_var("QPAY_USERNAME", "user");
    std::env::remove_var("QPAY_PASSWORD");
    std::env::set_var("QPAY_INVOICE_CODE", "code");
    std::env::set_var("QPAY_CALLBACK_URL", "cb");

    let result = QPayConfig::from_env();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("QPAY_PASSWORD"));

    // Clean up
    std::env::remove_var("QPAY_BASE_URL");
    std::env::remove_var("QPAY_USERNAME");
    std::env::remove_var("QPAY_INVOICE_CODE");
    std::env::remove_var("QPAY_CALLBACK_URL");
}

#[test]
#[serial]
fn test_config_from_env_missing_invoice_code() {
    std::env::set_var("QPAY_BASE_URL", "url");
    std::env::set_var("QPAY_USERNAME", "user");
    std::env::set_var("QPAY_PASSWORD", "pass");
    std::env::remove_var("QPAY_INVOICE_CODE");
    std::env::set_var("QPAY_CALLBACK_URL", "cb");

    let result = QPayConfig::from_env();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("QPAY_INVOICE_CODE"));

    // Clean up
    std::env::remove_var("QPAY_BASE_URL");
    std::env::remove_var("QPAY_USERNAME");
    std::env::remove_var("QPAY_PASSWORD");
    std::env::remove_var("QPAY_CALLBACK_URL");
}

#[test]
#[serial]
fn test_config_from_env_missing_callback_url() {
    std::env::set_var("QPAY_BASE_URL", "url");
    std::env::set_var("QPAY_USERNAME", "user");
    std::env::set_var("QPAY_PASSWORD", "pass");
    std::env::set_var("QPAY_INVOICE_CODE", "code");
    std::env::remove_var("QPAY_CALLBACK_URL");

    let result = QPayConfig::from_env();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("QPAY_CALLBACK_URL"));

    // Clean up
    std::env::remove_var("QPAY_BASE_URL");
    std::env::remove_var("QPAY_USERNAME");
    std::env::remove_var("QPAY_PASSWORD");
    std::env::remove_var("QPAY_INVOICE_CODE");
}

#[test]
#[serial]
fn test_config_from_env_error_is_config_variant() {
    std::env::remove_var("QPAY_BASE_URL");
    std::env::remove_var("QPAY_USERNAME");
    std::env::remove_var("QPAY_PASSWORD");
    std::env::remove_var("QPAY_INVOICE_CODE");
    std::env::remove_var("QPAY_CALLBACK_URL");

    let result = QPayConfig::from_env();
    assert!(result.is_err());

    let err = result.unwrap_err();
    match err {
        QPayError::Config(msg) => {
            assert!(msg.contains("QPAY_BASE_URL"));
        }
        _ => panic!("expected QPayError::Config, got: {:?}", err),
    }
}
