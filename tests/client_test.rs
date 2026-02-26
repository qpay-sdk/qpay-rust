use mockito::{Matcher, Server};
use qpay::models::*;
use qpay::{QPayClient, QPayConfig};

fn test_config(server_url: &str) -> QPayConfig {
    QPayConfig::new(
        server_url,
        "test_user",
        "test_pass",
        "TEST_CODE",
        "https://example.com/callback",
    )
}

fn token_json(expires_in: i64, refresh_expires_in: i64) -> String {
    serde_json::json!({
        "token_type": "Bearer",
        "refresh_expires_in": refresh_expires_in,
        "refresh_token": "mock_refresh_token",
        "access_token": "mock_access_token",
        "expires_in": expires_in,
        "scope": "default",
        "not-before-policy": "0",
        "session_state": "mock_session"
    })
    .to_string()
}

fn future_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        + 3600
}

// --- Auth: get_token ---

#[tokio::test]
async fn test_get_token_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    let mock = server
        .mock("POST", "/v2/auth/token")
        .match_header("authorization", Matcher::Regex("Basic .+".to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.get_token().await;
    assert!(result.is_ok());

    let token = result.unwrap();
    assert_eq!(token.access_token, "mock_access_token");
    assert_eq!(token.refresh_token, "mock_refresh_token");
    assert_eq!(token.token_type, "Bearer");

    mock.assert_async().await;
}

#[tokio::test]
async fn test_get_token_unauthorized() {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("POST", "/v2/auth/token")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"AUTHENTICATION_FAILED","message":"Invalid credentials"}"#)
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.get_token().await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    let (status, code, message) = qpay::is_qpay_error(&err).unwrap();
    assert_eq!(status, 401);
    assert_eq!(code, "AUTHENTICATION_FAILED");
    assert_eq!(message, "Invalid credentials");

    mock.assert_async().await;
}

// --- Auth: refresh_token ---

#[tokio::test]
async fn test_refresh_token_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    // First get a token
    let token_mock = server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    // Then refresh
    let refresh_json = serde_json::json!({
        "token_type": "Bearer",
        "refresh_expires_in": ts + 3600,
        "refresh_token": "new_refresh_token",
        "access_token": "new_access_token",
        "expires_in": ts + 600,
        "scope": "default",
        "not-before-policy": "0",
        "session_state": "new_session"
    })
    .to_string();

    let refresh_mock = server
        .mock("POST", "/v2/auth/refresh")
        .match_header("authorization", "Bearer mock_refresh_token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(refresh_json)
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    // Get initial token
    client.get_token().await.unwrap();
    token_mock.assert_async().await;

    // Refresh
    let result = client.refresh_token().await;
    assert!(result.is_ok());

    let token = result.unwrap();
    assert_eq!(token.access_token, "new_access_token");
    assert_eq!(token.refresh_token, "new_refresh_token");

    refresh_mock.assert_async().await;
}

// --- Invoice: create_simple_invoice ---

#[tokio::test]
async fn test_create_simple_invoice_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    let token_mock = server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let invoice_response = serde_json::json!({
        "invoice_id": "inv_abc123",
        "qr_text": "encoded_qr_data",
        "qr_image": "base64_qr_image",
        "qPay_shortUrl": "https://qpay.mn/q/abc",
        "urls": [
            {
                "name": "Khan Bank",
                "description": "Khan Bank app",
                "logo": "https://cdn.qpay.mn/khan.png",
                "link": "khanbank://payment?data=abc"
            }
        ]
    });

    let invoice_mock = server
        .mock("POST", "/v2/invoice")
        .match_header("authorization", "Bearer mock_access_token")
        .match_header("content-type", "application/json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(invoice_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = CreateSimpleInvoiceRequest {
        invoice_code: "TEST_CODE".to_string(),
        sender_invoice_no: "INV-001".to_string(),
        invoice_receiver_code: "terminal".to_string(),
        invoice_description: "Test payment".to_string(),
        sender_branch_code: None,
        amount: 5000.0,
        callback_url: "https://example.com/cb".to_string(),
    };

    let result = client.create_simple_invoice(&req).await;
    assert!(result.is_ok());

    let invoice = result.unwrap();
    assert_eq!(invoice.invoice_id, "inv_abc123");
    assert_eq!(invoice.qpay_short_url, "https://qpay.mn/q/abc");
    assert_eq!(invoice.urls.len(), 1);
    assert_eq!(invoice.urls[0].name, "Khan Bank");

    token_mock.assert_async().await;
    invoice_mock.assert_async().await;
}

// --- Invoice: create_invoice (full) ---

#[tokio::test]
async fn test_create_invoice_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    let token_mock = server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let invoice_response = serde_json::json!({
        "invoice_id": "inv_full_001",
        "qr_text": "qr",
        "qr_image": "img",
        "qPay_shortUrl": "https://qpay.mn/q/full",
        "urls": []
    });

    let invoice_mock = server
        .mock("POST", "/v2/invoice")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body(invoice_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = CreateInvoiceRequest {
        invoice_code: "CODE".to_string(),
        sender_invoice_no: "INV-FULL-001".to_string(),
        sender_branch_code: None,
        sender_branch_data: None,
        sender_staff_data: None,
        sender_staff_code: None,
        invoice_receiver_code: "terminal".to_string(),
        invoice_receiver_data: None,
        invoice_description: "Full invoice".to_string(),
        enable_expiry: None,
        allow_partial: None,
        minimum_amount: None,
        allow_exceed: None,
        maximum_amount: None,
        amount: 10000.0,
        callback_url: "https://cb.example.com".to_string(),
        sender_terminal_code: None,
        sender_terminal_data: None,
        allow_subscribe: None,
        subscription_interval: None,
        subscription_webhook: None,
        note: None,
        transactions: None,
        lines: None,
    };

    let result = client.create_invoice(&req).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().invoice_id, "inv_full_001");

    token_mock.assert_async().await;
    invoice_mock.assert_async().await;
}

// --- Invoice: create_ebarimt_invoice ---

#[tokio::test]
async fn test_create_ebarimt_invoice_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    let token_mock = server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let invoice_response = serde_json::json!({
        "invoice_id": "inv_ebarimt_001",
        "qr_text": "qr",
        "qr_image": "img",
        "qPay_shortUrl": "https://qpay.mn/q/eb",
        "urls": []
    });

    let invoice_mock = server
        .mock("POST", "/v2/invoice")
        .with_status(200)
        .with_body(invoice_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = CreateEbarimtInvoiceRequest {
        invoice_code: "CODE".to_string(),
        sender_invoice_no: "INV-EB-001".to_string(),
        sender_branch_code: None,
        sender_staff_data: None,
        sender_staff_code: None,
        invoice_receiver_code: "terminal".to_string(),
        invoice_receiver_data: None,
        invoice_description: "Ebarimt invoice".to_string(),
        tax_type: "1".to_string(),
        district_code: "23".to_string(),
        callback_url: "https://cb.example.com".to_string(),
        lines: vec![EbarimtInvoiceLine {
            tax_product_code: Some("TAX001".to_string()),
            line_description: "Product".to_string(),
            barcode: None,
            line_quantity: "1".to_string(),
            line_unit_price: "1000".to_string(),
            note: None,
            classification_code: None,
            taxes: None,
        }],
    };

    let result = client.create_ebarimt_invoice(&req).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().invoice_id, "inv_ebarimt_001");

    token_mock.assert_async().await;
    invoice_mock.assert_async().await;
}

// --- Invoice: cancel_invoice ---

#[tokio::test]
async fn test_cancel_invoice_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    let token_mock = server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let cancel_mock = server
        .mock("DELETE", "/v2/invoice/inv_123")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.cancel_invoice("inv_123").await;
    assert!(result.is_ok());

    token_mock.assert_async().await;
    cancel_mock.assert_async().await;
}

#[tokio::test]
async fn test_cancel_invoice_not_found() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("DELETE", "/v2/invoice/inv_nonexist")
        .with_status(404)
        .with_body(r#"{"code":"INVOICE_NOTFOUND","message":"Invoice not found"}"#)
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.cancel_invoice("inv_nonexist").await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    let (status, code, _) = qpay::is_qpay_error(&err).unwrap();
    assert_eq!(status, 404);
    assert_eq!(code, "INVOICE_NOTFOUND");
}

// --- Invoice: API error ---

#[tokio::test]
async fn test_create_invoice_api_error() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("POST", "/v2/invoice")
        .with_status(400)
        .with_body(r#"{"code":"INVALID_AMOUNT","message":"Amount must be positive"}"#)
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = CreateSimpleInvoiceRequest {
        invoice_code: "CODE".to_string(),
        sender_invoice_no: "INV-001".to_string(),
        invoice_receiver_code: "terminal".to_string(),
        invoice_description: "Test".to_string(),
        sender_branch_code: None,
        amount: -100.0,
        callback_url: "https://cb.example.com".to_string(),
    };

    let result = client.create_simple_invoice(&req).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    let (status, code, msg) = qpay::is_qpay_error(&err).unwrap();
    assert_eq!(status, 400);
    assert_eq!(code, "INVALID_AMOUNT");
    assert_eq!(msg, "Amount must be positive");
}

// --- Payment: get_payment ---

#[tokio::test]
async fn test_get_payment_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let payment_json = serde_json::json!({
        "payment_id": "pay_001",
        "payment_status": "PAID",
        "payment_fee": "100",
        "payment_amount": "5000",
        "payment_currency": "MNT",
        "payment_date": "2026-01-15",
        "payment_wallet": "qPay",
        "transaction_type": "P2P",
        "object_type": "INVOICE",
        "object_id": "inv_001",
        "next_payment_date": null,
        "next_payment_datetime": null,
        "card_transactions": [],
        "p2p_transactions": []
    });

    server
        .mock("GET", "/v2/payment/pay_001")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body(payment_json.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.get_payment("pay_001").await;
    assert!(result.is_ok());

    let detail = result.unwrap();
    assert_eq!(detail.payment_id, "pay_001");
    assert_eq!(detail.payment_status, "PAID");
    assert_eq!(detail.payment_amount, "5000");
}

#[tokio::test]
async fn test_get_payment_not_found() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("GET", "/v2/payment/pay_nonexist")
        .with_status(404)
        .with_body(r#"{"code":"PAYMENT_NOTFOUND","message":"Payment not found"}"#)
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.get_payment("pay_nonexist").await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    let (status, code, _) = qpay::is_qpay_error(&err).unwrap();
    assert_eq!(status, 404);
    assert_eq!(code, "PAYMENT_NOTFOUND");
}

// --- Payment: check_payment ---

#[tokio::test]
async fn test_check_payment_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let check_response = serde_json::json!({
        "count": 1,
        "paid_amount": 5000.0,
        "rows": [
            {
                "payment_id": "pay_001",
                "payment_status": "PAID",
                "payment_amount": "5000",
                "trx_fee": "50",
                "payment_currency": "MNT",
                "payment_wallet": "qPay",
                "payment_type": "P2P",
                "next_payment_date": null,
                "next_payment_datetime": null,
                "card_transactions": [],
                "p2p_transactions": []
            }
        ]
    });

    server
        .mock("POST", "/v2/payment/check")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body(check_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = PaymentCheckRequest {
        object_type: "INVOICE".to_string(),
        object_id: "inv_001".to_string(),
        offset: None,
    };

    let result = client.check_payment(&req).await;
    assert!(result.is_ok());

    let resp = result.unwrap();
    assert_eq!(resp.count, 1);
    assert_eq!(resp.paid_amount, Some(5000.0));
    assert_eq!(resp.rows[0].payment_status, "PAID");
}

#[tokio::test]
async fn test_check_payment_no_payments() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("POST", "/v2/payment/check")
        .with_status(200)
        .with_body(r#"{"count": 0, "rows": []}"#)
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = PaymentCheckRequest {
        object_type: "INVOICE".to_string(),
        object_id: "inv_empty".to_string(),
        offset: None,
    };

    let result = client.check_payment(&req).await;
    assert!(result.is_ok());

    let resp = result.unwrap();
    assert_eq!(resp.count, 0);
    assert!(resp.rows.is_empty());
}

// --- Payment: list_payments ---

#[tokio::test]
async fn test_list_payments_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let list_response = serde_json::json!({
        "count": 2,
        "rows": [
            {
                "payment_id": "pay_001",
                "payment_date": "2026-01-15",
                "payment_status": "PAID",
                "payment_fee": "100",
                "payment_amount": "5000",
                "payment_currency": "MNT",
                "payment_wallet": "qPay",
                "payment_name": "Payment 1",
                "payment_description": "Test 1",
                "qr_code": "qr1",
                "paid_by": "user1",
                "object_type": "INVOICE",
                "object_id": "inv_001"
            },
            {
                "payment_id": "pay_002",
                "payment_date": "2026-01-16",
                "payment_status": "PAID",
                "payment_fee": "200",
                "payment_amount": "10000",
                "payment_currency": "MNT",
                "payment_wallet": "qPay",
                "payment_name": "Payment 2",
                "payment_description": "Test 2",
                "qr_code": "qr2",
                "paid_by": "user2",
                "object_type": "INVOICE",
                "object_id": "inv_002"
            }
        ]
    });

    server
        .mock("POST", "/v2/payment/list")
        .with_status(200)
        .with_body(list_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = PaymentListRequest {
        object_type: "INVOICE".to_string(),
        object_id: "inv_001".to_string(),
        start_date: "2026-01-01".to_string(),
        end_date: "2026-01-31".to_string(),
        offset: Offset {
            page_number: 1,
            page_limit: 20,
        },
    };

    let result = client.list_payments(&req).await;
    assert!(result.is_ok());

    let resp = result.unwrap();
    assert_eq!(resp.count, 2);
    assert_eq!(resp.rows.len(), 2);
    assert_eq!(resp.rows[0].payment_id, "pay_001");
    assert_eq!(resp.rows[1].payment_amount, "10000");
}

// --- Payment: cancel_payment ---

#[tokio::test]
async fn test_cancel_payment_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("DELETE", "/v2/payment/cancel/pay_001")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = PaymentCancelRequest {
        callback_url: Some("https://cb.example.com".to_string()),
        note: Some("User cancelled".to_string()),
    };

    let result = client.cancel_payment("pay_001", &req).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_cancel_payment_already_canceled() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("DELETE", "/v2/payment/cancel/pay_cancelled")
        .with_status(400)
        .with_body(
            r#"{"code":"PAYMENT_ALREADY_CANCELED","message":"Payment is already canceled"}"#,
        )
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = PaymentCancelRequest::default();
    let result = client.cancel_payment("pay_cancelled", &req).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    let (_, code, _) = qpay::is_qpay_error(&err).unwrap();
    assert_eq!(code, "PAYMENT_ALREADY_CANCELED");
}

// --- Payment: refund_payment ---

#[tokio::test]
async fn test_refund_payment_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("DELETE", "/v2/payment/refund/pay_001")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body("")
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = PaymentRefundRequest {
        callback_url: Some("https://cb.example.com".to_string()),
        note: Some("Refund requested".to_string()),
    };

    let result = client.refund_payment("pay_001", &req).await;
    assert!(result.is_ok());
}

// --- Ebarimt: create_ebarimt ---

#[tokio::test]
async fn test_create_ebarimt_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let ebarimt_response = serde_json::json!({
        "id": "eb_001",
        "ebarimt_by": "system",
        "g_wallet_id": "w_001",
        "g_wallet_customer_id": "wc_001",
        "ebarimt_receiver_type": "83",
        "ebarimt_receiver": "",
        "ebarimt_district_code": "23",
        "ebarimt_bill_type": "1",
        "g_merchant_id": "m_001",
        "merchant_branch_code": "br_001",
        "merchant_terminal_code": null,
        "merchant_staff_code": null,
        "merchant_register_no": "1234567",
        "g_payment_id": "pay_001",
        "paid_by": "user1",
        "object_type": "INVOICE",
        "object_id": "inv_001",
        "amount": "5000",
        "vat_amount": "500",
        "city_tax_amount": "50",
        "ebarimt_qr_data": "qr_data",
        "ebarimt_lottery": "AB12345678",
        "note": null,
        "barimt_status": "SUCCESS",
        "barimt_status_date": "2026-01-15",
        "ebarimt_sent_email": null,
        "ebarimt_receiver_phone": "99001122",
        "tax_type": "1",
        "merchant_tin": null,
        "ebarimt_receipt_id": null,
        "created_by": "system",
        "created_date": "2026-01-15",
        "updated_by": "system",
        "updated_date": "2026-01-15",
        "status": true,
        "barimt_items": [],
        "barimt_transactions": [],
        "barimt_histories": []
    });

    server
        .mock("POST", "/v2/ebarimt_v3/create")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body(ebarimt_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = CreateEbarimtRequest {
        payment_id: "pay_001".to_string(),
        ebarimt_receiver_type: "83".to_string(),
        ebarimt_receiver: None,
        district_code: Some("23".to_string()),
        classification_code: None,
    };

    let result = client.create_ebarimt(&req).await;
    assert!(result.is_ok());

    let ebarimt = result.unwrap();
    assert_eq!(ebarimt.id, "eb_001");
    assert_eq!(ebarimt.barimt_status, "SUCCESS");
    assert_eq!(ebarimt.ebarimt_lottery, "AB12345678");
    assert!(ebarimt.status);
}

// --- Ebarimt: cancel_ebarimt ---

#[tokio::test]
async fn test_cancel_ebarimt_success() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let ebarimt_response = serde_json::json!({
        "id": "eb_001",
        "ebarimt_by": "system",
        "g_wallet_id": "w_001",
        "g_wallet_customer_id": "wc_001",
        "ebarimt_receiver_type": "83",
        "ebarimt_receiver": "",
        "ebarimt_district_code": "23",
        "ebarimt_bill_type": "1",
        "g_merchant_id": "m_001",
        "merchant_branch_code": "br_001",
        "merchant_terminal_code": null,
        "merchant_staff_code": null,
        "merchant_register_no": "1234567",
        "g_payment_id": "pay_001",
        "paid_by": "user1",
        "object_type": "INVOICE",
        "object_id": "inv_001",
        "amount": "5000",
        "vat_amount": "500",
        "city_tax_amount": "50",
        "ebarimt_qr_data": "qr_data",
        "ebarimt_lottery": "AB12345678",
        "note": null,
        "barimt_status": "CANCELLED",
        "barimt_status_date": "2026-01-16",
        "ebarimt_sent_email": null,
        "ebarimt_receiver_phone": "99001122",
        "tax_type": "1",
        "merchant_tin": null,
        "ebarimt_receipt_id": null,
        "created_by": "system",
        "created_date": "2026-01-15",
        "updated_by": "system",
        "updated_date": "2026-01-16",
        "status": false,
        "barimt_items": [],
        "barimt_transactions": [],
        "barimt_histories": []
    });

    server
        .mock("DELETE", "/v2/ebarimt_v3/pay_001")
        .match_header("authorization", "Bearer mock_access_token")
        .with_status(200)
        .with_body(ebarimt_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.cancel_ebarimt("pay_001").await;
    assert!(result.is_ok());

    let ebarimt = result.unwrap();
    assert_eq!(ebarimt.barimt_status, "CANCELLED");
    assert!(!ebarimt.status);
}

// --- Token auto-refresh ---

#[tokio::test]
async fn test_auto_token_refresh_on_expired() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    // Token endpoint called first for initial auth, then later when refresh fails
    let _token_mock = server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .expect_at_least(1)
        .create_async()
        .await;

    let check_response = serde_json::json!({
        "count": 0,
        "rows": []
    });

    server
        .mock("POST", "/v2/payment/check")
        .with_status(200)
        .with_body(check_response.to_string())
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    // Make a request -- ensure_token should get a new token automatically
    let req = PaymentCheckRequest {
        object_type: "INVOICE".to_string(),
        object_id: "inv_001".to_string(),
        offset: None,
    };

    let result = client.check_payment(&req).await;
    assert!(result.is_ok());
}

// --- with_http_client ---

#[tokio::test]
async fn test_with_http_client() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    let config = test_config(&server.url());
    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();
    let client = QPayClient::with_http_client(config, http);

    let result = client.get_token().await;
    assert!(result.is_ok());
}

// --- Server error (500) ---

#[tokio::test]
async fn test_server_error_500() {
    let mut server = Server::new_async().await;
    let ts = future_timestamp();

    server
        .mock("POST", "/v2/auth/token")
        .with_status(200)
        .with_body(token_json(ts, ts + 1800))
        .create_async()
        .await;

    server
        .mock("POST", "/v2/invoice")
        .with_status(500)
        .with_body("Internal Server Error")
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let req = CreateSimpleInvoiceRequest {
        invoice_code: "CODE".to_string(),
        sender_invoice_no: "INV-001".to_string(),
        invoice_receiver_code: "terminal".to_string(),
        invoice_description: "Test".to_string(),
        sender_branch_code: None,
        amount: 1000.0,
        callback_url: "https://cb.example.com".to_string(),
    };

    let result = client.create_simple_invoice(&req).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    let (status, _, _) = qpay::is_qpay_error(&err).unwrap();
    assert_eq!(status, 500);
}

// --- Token error (non-JSON response from auth) ---

#[tokio::test]
async fn test_get_token_non_json_error() {
    let mut server = Server::new_async().await;

    server
        .mock("POST", "/v2/auth/token")
        .with_status(503)
        .with_body("Service Unavailable")
        .create_async()
        .await;

    let config = test_config(&server.url());
    let client = QPayClient::new(config);

    let result = client.get_token().await;
    assert!(result.is_err());
}
