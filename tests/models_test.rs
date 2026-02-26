use qpay::models::*;

// --- TokenResponse ---

#[test]
fn test_token_response_deserialize() {
    let json = r#"{
        "token_type": "Bearer",
        "refresh_expires_in": 1800,
        "refresh_token": "refresh_abc123",
        "access_token": "access_xyz789",
        "expires_in": 300,
        "scope": "default",
        "not-before-policy": "0",
        "session_state": "session_123"
    }"#;

    let token: TokenResponse = serde_json::from_str(json).unwrap();
    assert_eq!(token.token_type, "Bearer");
    assert_eq!(token.refresh_expires_in, 1800);
    assert_eq!(token.refresh_token, "refresh_abc123");
    assert_eq!(token.access_token, "access_xyz789");
    assert_eq!(token.expires_in, 300);
    assert_eq!(token.scope, "default");
    assert_eq!(token.not_before_policy, "0");
    assert_eq!(token.session_state, "session_123");
}

#[test]
fn test_token_response_serialize() {
    let token = TokenResponse {
        token_type: "Bearer".to_string(),
        refresh_expires_in: 1800,
        refresh_token: "refresh_tok".to_string(),
        access_token: "access_tok".to_string(),
        expires_in: 300,
        scope: "default".to_string(),
        not_before_policy: "0".to_string(),
        session_state: "sess".to_string(),
    };

    let json = serde_json::to_string(&token).unwrap();
    assert!(json.contains("\"not-before-policy\":\"0\""));
    assert!(json.contains("\"access_token\":\"access_tok\""));
}

#[test]
fn test_token_response_roundtrip() {
    let original = TokenResponse {
        token_type: "Bearer".to_string(),
        refresh_expires_in: 3600,
        refresh_token: "rt".to_string(),
        access_token: "at".to_string(),
        expires_in: 600,
        scope: "openid".to_string(),
        not_before_policy: "0".to_string(),
        session_state: "s1".to_string(),
    };

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: TokenResponse = serde_json::from_str(&json).unwrap();

    assert_eq!(original.access_token, deserialized.access_token);
    assert_eq!(original.refresh_token, deserialized.refresh_token);
    assert_eq!(original.expires_in, deserialized.expires_in);
    assert_eq!(original.refresh_expires_in, deserialized.refresh_expires_in);
}

// --- Address ---

#[test]
fn test_address_default() {
    let addr = Address::default();
    assert!(addr.city.is_none());
    assert!(addr.district.is_none());
    assert!(addr.street.is_none());
    assert!(addr.building.is_none());
    assert!(addr.address.is_none());
    assert!(addr.zipcode.is_none());
    assert!(addr.longitude.is_none());
    assert!(addr.latitude.is_none());
}

#[test]
fn test_address_serialize_skip_none() {
    let addr = Address {
        city: Some("Ulaanbaatar".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&addr).unwrap();
    assert!(json.contains("Ulaanbaatar"));
    assert!(!json.contains("district"));
    assert!(!json.contains("street"));
}

#[test]
fn test_address_deserialize_full() {
    let json = r#"{
        "city": "UB",
        "district": "SBD",
        "street": "Peace Ave",
        "building": "101",
        "address": "Full address",
        "zipcode": "14200",
        "longitude": "106.9",
        "latitude": "47.9"
    }"#;

    let addr: Address = serde_json::from_str(json).unwrap();
    assert_eq!(addr.city.unwrap(), "UB");
    assert_eq!(addr.district.unwrap(), "SBD");
    assert_eq!(addr.zipcode.unwrap(), "14200");
}

// --- CreateSimpleInvoiceRequest ---

#[test]
fn test_simple_invoice_request_serialize() {
    let req = CreateSimpleInvoiceRequest {
        invoice_code: "TEST_CODE".to_string(),
        sender_invoice_no: "INV-001".to_string(),
        invoice_receiver_code: "terminal".to_string(),
        invoice_description: "Test payment".to_string(),
        sender_branch_code: None,
        amount: 5000.0,
        callback_url: "https://example.com/cb".to_string(),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"invoice_code\":\"TEST_CODE\""));
    assert!(json.contains("\"amount\":5000.0"));
    assert!(!json.contains("sender_branch_code"));
}

#[test]
fn test_simple_invoice_request_with_branch_code() {
    let req = CreateSimpleInvoiceRequest {
        invoice_code: "CODE".to_string(),
        sender_invoice_no: "INV-002".to_string(),
        invoice_receiver_code: "terminal".to_string(),
        invoice_description: "With branch".to_string(),
        sender_branch_code: Some("BRANCH_01".to_string()),
        amount: 1000.0,
        callback_url: "https://example.com/cb".to_string(),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"sender_branch_code\":\"BRANCH_01\""));
}

#[test]
fn test_simple_invoice_request_deserialize() {
    let json = r#"{
        "invoice_code": "CODE",
        "sender_invoice_no": "INV-001",
        "invoice_receiver_code": "terminal",
        "invoice_description": "Test",
        "amount": 2500.5,
        "callback_url": "https://cb.example.com"
    }"#;

    let req: CreateSimpleInvoiceRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.invoice_code, "CODE");
    assert_eq!(req.amount, 2500.5);
    assert!(req.sender_branch_code.is_none());
}

// --- InvoiceResponse ---

#[test]
fn test_invoice_response_deserialize() {
    let json = r#"{
        "invoice_id": "inv_123",
        "qr_text": "qr_data_here",
        "qr_image": "base64_image",
        "qPay_shortUrl": "https://qpay.mn/q/abc",
        "urls": [
            {
                "name": "Khan Bank",
                "description": "Khan Bank app",
                "logo": "https://logo.png",
                "link": "khanbank://payment?data=abc"
            }
        ]
    }"#;

    let resp: InvoiceResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.invoice_id, "inv_123");
    assert_eq!(resp.qr_text, "qr_data_here");
    assert_eq!(resp.qpay_short_url, "https://qpay.mn/q/abc");
    assert_eq!(resp.urls.len(), 1);
    assert_eq!(resp.urls[0].name, "Khan Bank");
}

#[test]
fn test_invoice_response_serialize_preserves_qpay_short_url_field() {
    let resp = InvoiceResponse {
        invoice_id: "id".to_string(),
        qr_text: "qr".to_string(),
        qr_image: "img".to_string(),
        qpay_short_url: "https://short.url".to_string(),
        urls: vec![],
    };

    let json = serde_json::to_string(&resp).unwrap();
    assert!(json.contains("qPay_shortUrl"));
}

// --- PaymentCheckRequest ---

#[test]
fn test_payment_check_request_serialize() {
    let req = PaymentCheckRequest {
        object_type: "INVOICE".to_string(),
        object_id: "inv_123".to_string(),
        offset: Some(Offset {
            page_number: 1,
            page_limit: 10,
        }),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"object_type\":\"INVOICE\""));
    assert!(json.contains("\"page_number\":1"));
}

#[test]
fn test_payment_check_request_without_offset() {
    let req = PaymentCheckRequest {
        object_type: "INVOICE".to_string(),
        object_id: "inv_456".to_string(),
        offset: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(!json.contains("offset"));
}

// --- PaymentCheckResponse ---

#[test]
fn test_payment_check_response_deserialize() {
    let json = r#"{
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
    }"#;

    let resp: PaymentCheckResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.count, 1);
    assert_eq!(resp.paid_amount, Some(5000.0));
    assert_eq!(resp.rows.len(), 1);
    assert_eq!(resp.rows[0].payment_id, "pay_001");
    assert_eq!(resp.rows[0].payment_status, "PAID");
}

#[test]
fn test_payment_check_response_no_paid_amount() {
    let json = r#"{
        "count": 0,
        "rows": []
    }"#;

    let resp: PaymentCheckResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.count, 0);
    assert!(resp.paid_amount.is_none());
    assert!(resp.rows.is_empty());
}

// --- PaymentDetail ---

#[test]
fn test_payment_detail_deserialize() {
    let json = r#"{
        "payment_id": "pay_789",
        "payment_status": "PAID",
        "payment_fee": "100",
        "payment_amount": "10000",
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
    }"#;

    let detail: PaymentDetail = serde_json::from_str(json).unwrap();
    assert_eq!(detail.payment_id, "pay_789");
    assert_eq!(detail.payment_amount, "10000");
    assert_eq!(detail.object_type, "INVOICE");
}

// --- PaymentListRequest ---

#[test]
fn test_payment_list_request_serialize() {
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

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"start_date\":\"2026-01-01\""));
    assert!(json.contains("\"page_limit\":20"));
}

// --- PaymentCancelRequest / PaymentRefundRequest ---

#[test]
fn test_payment_cancel_request_default() {
    let req = PaymentCancelRequest::default();
    assert!(req.callback_url.is_none());
    assert!(req.note.is_none());
}

#[test]
fn test_payment_cancel_request_skip_none() {
    let req = PaymentCancelRequest::default();
    let json = serde_json::to_string(&req).unwrap();
    assert_eq!(json, "{}");
}

#[test]
fn test_payment_cancel_request_with_values() {
    let req = PaymentCancelRequest {
        callback_url: Some("https://cb.example.com".to_string()),
        note: Some("Cancelled by user".to_string()),
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("callback_url"));
    assert!(json.contains("Cancelled by user"));
}

#[test]
fn test_payment_refund_request_default() {
    let req = PaymentRefundRequest::default();
    assert!(req.callback_url.is_none());
    assert!(req.note.is_none());
}

#[test]
fn test_payment_refund_request_skip_none() {
    let req = PaymentRefundRequest::default();
    let json = serde_json::to_string(&req).unwrap();
    assert_eq!(json, "{}");
}

// --- CreateEbarimtRequest ---

#[test]
fn test_create_ebarimt_request_serialize() {
    let req = CreateEbarimtRequest {
        payment_id: "pay_001".to_string(),
        ebarimt_receiver_type: "83".to_string(),
        ebarimt_receiver: None,
        district_code: Some("23".to_string()),
        classification_code: None,
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"payment_id\":\"pay_001\""));
    assert!(json.contains("\"ebarimt_receiver_type\":\"83\""));
    assert!(json.contains("\"district_code\":\"23\""));
    assert!(!json.contains("\"ebarimt_receiver\":"));
    assert!(!json.contains("\"classification_code\":"));
}

#[test]
fn test_create_ebarimt_request_deserialize() {
    let json = r#"{
        "payment_id": "pay_002",
        "ebarimt_receiver_type": "80",
        "ebarimt_receiver": "1234567",
        "district_code": "01"
    }"#;

    let req: CreateEbarimtRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.payment_id, "pay_002");
    assert_eq!(req.ebarimt_receiver.unwrap(), "1234567");
}

// --- Deeplink ---

#[test]
fn test_deeplink_deserialize() {
    let json = r#"{
        "name": "Khan Bank",
        "description": "Khan Bank payment",
        "logo": "https://cdn.qpay.mn/khan.png",
        "link": "khanbank://qpay?data=abc123"
    }"#;

    let dl: Deeplink = serde_json::from_str(json).unwrap();
    assert_eq!(dl.name, "Khan Bank");
    assert_eq!(dl.link, "khanbank://qpay?data=abc123");
}

// --- InvoiceLine ---

#[test]
fn test_invoice_line_serialize() {
    let line = InvoiceLine {
        tax_product_code: Some("TAX001".to_string()),
        line_description: "Product A".to_string(),
        line_quantity: "2".to_string(),
        line_unit_price: "500".to_string(),
        note: None,
        discounts: None,
        surcharges: None,
        taxes: None,
    };

    let json = serde_json::to_string(&line).unwrap();
    assert!(json.contains("\"line_description\":\"Product A\""));
    assert!(json.contains("\"tax_product_code\":\"TAX001\""));
    assert!(!json.contains("\"note\""));
    assert!(!json.contains("\"discounts\""));
}

// --- TaxEntry ---

#[test]
fn test_tax_entry_serialize() {
    let entry = TaxEntry {
        tax_code: Some("VAT".to_string()),
        discount_code: None,
        surcharge_code: None,
        description: "Value Added Tax".to_string(),
        amount: 100.0,
        note: None,
    };

    let json = serde_json::to_string(&entry).unwrap();
    assert!(json.contains("\"tax_code\":\"VAT\""));
    assert!(json.contains("\"amount\":100.0"));
    assert!(!json.contains("discount_code"));
}

// --- CreateInvoiceRequest ---

#[test]
fn test_create_invoice_request_minimal() {
    let req = CreateInvoiceRequest {
        invoice_code: "CODE".to_string(),
        sender_invoice_no: "INV-001".to_string(),
        sender_branch_code: None,
        sender_branch_data: None,
        sender_staff_data: None,
        sender_staff_code: None,
        invoice_receiver_code: "terminal".to_string(),
        invoice_receiver_data: None,
        invoice_description: "Test".to_string(),
        enable_expiry: None,
        allow_partial: None,
        minimum_amount: None,
        allow_exceed: None,
        maximum_amount: None,
        amount: 1000.0,
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

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"invoice_code\":\"CODE\""));
    assert!(json.contains("\"amount\":1000.0"));
    // Optional None fields should be skipped
    assert!(!json.contains("sender_branch_code"));
    assert!(!json.contains("lines"));
}

// --- Offset ---

#[test]
fn test_offset_roundtrip() {
    let offset = Offset {
        page_number: 3,
        page_limit: 50,
    };

    let json = serde_json::to_string(&offset).unwrap();
    let deserialized: Offset = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.page_number, 3);
    assert_eq!(deserialized.page_limit, 50);
}

// --- SenderBranchData ---

#[test]
fn test_sender_branch_data_default() {
    let data = SenderBranchData::default();
    assert!(data.register.is_none());
    assert!(data.name.is_none());
    assert!(data.email.is_none());
    assert!(data.phone.is_none());
    assert!(data.address.is_none());
}

// --- SenderStaffData ---

#[test]
fn test_sender_staff_data_default() {
    let data = SenderStaffData::default();
    assert!(data.name.is_none());
    assert!(data.email.is_none());
    assert!(data.phone.is_none());
}

// --- InvoiceReceiverData ---

#[test]
fn test_invoice_receiver_data_default() {
    let data = InvoiceReceiverData::default();
    assert!(data.register.is_none());
    assert!(data.name.is_none());
    assert!(data.email.is_none());
    assert!(data.phone.is_none());
    assert!(data.address.is_none());
}

// --- Account ---

#[test]
fn test_account_deserialize() {
    let json = r#"{
        "account_bank_code": "050000",
        "account_number": "1234567890",
        "iban_number": "MN123456",
        "account_name": "Test Account",
        "account_currency": "MNT",
        "is_default": true
    }"#;

    let acc: Account = serde_json::from_str(json).unwrap();
    assert_eq!(acc.account_bank_code, "050000");
    assert_eq!(acc.account_number, "1234567890");
    assert!(acc.is_default);
}

// --- Transaction ---

#[test]
fn test_transaction_serialize() {
    let txn = Transaction {
        description: "Payment for order".to_string(),
        amount: "5000".to_string(),
        accounts: None,
    };

    let json = serde_json::to_string(&txn).unwrap();
    assert!(json.contains("\"description\":\"Payment for order\""));
    assert!(!json.contains("accounts"));
}

// --- P2PTransaction ---

#[test]
fn test_p2p_transaction_deserialize() {
    let json = r#"{
        "transaction_bank_code": "050000",
        "account_bank_code": "050000",
        "account_bank_name": "Khan Bank",
        "account_number": "1234567890",
        "status": "SUCCESS",
        "amount": "5000",
        "currency": "MNT",
        "settlement_status": "SETTLED"
    }"#;

    let txn: P2PTransaction = serde_json::from_str(json).unwrap();
    assert_eq!(txn.transaction_bank_code, "050000");
    assert_eq!(txn.status, "SUCCESS");
    assert_eq!(txn.amount, "5000");
}

// --- CardTransaction ---

#[test]
fn test_card_transaction_deserialize() {
    let json = r#"{
        "card_type": "VISA",
        "is_cross_border": false,
        "settlement_status": "SETTLED",
        "settlement_status_date": "2026-01-15"
    }"#;

    let txn: CardTransaction = serde_json::from_str(json).unwrap();
    assert_eq!(txn.card_type, "VISA");
    assert!(!txn.is_cross_border);
    assert!(txn.card_number.is_none());
}

// --- PaymentListResponse ---

#[test]
fn test_payment_list_response_deserialize() {
    let json = r#"{
        "count": 1,
        "rows": [
            {
                "payment_id": "pay_001",
                "payment_date": "2026-01-15",
                "payment_status": "PAID",
                "payment_fee": "100",
                "payment_amount": "5000",
                "payment_currency": "MNT",
                "payment_wallet": "qPay",
                "payment_name": "Test Payment",
                "payment_description": "Test",
                "qr_code": "qr_abc",
                "paid_by": "user123",
                "object_type": "INVOICE",
                "object_id": "inv_001"
            }
        ]
    }"#;

    let resp: PaymentListResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.count, 1);
    assert_eq!(resp.rows[0].payment_id, "pay_001");
    assert_eq!(resp.rows[0].payment_amount, "5000");
}

// --- EbarimtInvoiceLine ---

#[test]
fn test_ebarimt_invoice_line_serialize() {
    let line = EbarimtInvoiceLine {
        tax_product_code: Some("TAX001".to_string()),
        line_description: "Product".to_string(),
        barcode: None,
        line_quantity: "1".to_string(),
        line_unit_price: "1000".to_string(),
        note: None,
        classification_code: None,
        taxes: None,
    };

    let json = serde_json::to_string(&line).unwrap();
    assert!(json.contains("\"tax_product_code\":\"TAX001\""));
    assert!(!json.contains("barcode"));
    assert!(!json.contains("classification_code"));
}

// --- CreateEbarimtInvoiceRequest ---

#[test]
fn test_create_ebarimt_invoice_request_serialize() {
    let req = CreateEbarimtInvoiceRequest {
        invoice_code: "CODE".to_string(),
        sender_invoice_no: "INV-001".to_string(),
        sender_branch_code: None,
        sender_staff_data: None,
        sender_staff_code: None,
        invoice_receiver_code: "terminal".to_string(),
        invoice_receiver_data: None,
        invoice_description: "Tax invoice".to_string(),
        tax_type: "1".to_string(),
        district_code: "23".to_string(),
        callback_url: "https://cb.example.com".to_string(),
        lines: vec![EbarimtInvoiceLine {
            tax_product_code: Some("TAX001".to_string()),
            line_description: "Item".to_string(),
            barcode: None,
            line_quantity: "1".to_string(),
            line_unit_price: "500".to_string(),
            note: None,
            classification_code: None,
            taxes: None,
        }],
    };

    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("\"tax_type\":\"1\""));
    assert!(json.contains("\"district_code\":\"23\""));
    assert!(json.contains("TAX001"));
}
