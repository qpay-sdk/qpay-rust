# qpay

QPay V2 API SDK for Rust. Async client with automatic token management, invoice creation, payment operations, and ebarimt (electronic tax receipt) support.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
qpay = "1.0.0"
tokio = { version = "1", features = ["full"] }
```

Or using `cargo add`:

```bash
cargo add qpay
cargo add tokio --features full
```

## Quick Start

```rust
use qpay::{QPayClient, QPayConfig, models::CreateSimpleInvoiceRequest};

#[tokio::main]
async fn main() -> Result<(), qpay::QPayError> {
    // Configure from environment variables
    let config = QPayConfig::from_env()?;
    let client = QPayClient::new(config);

    // Create an invoice
    let req = CreateSimpleInvoiceRequest {
        invoice_code: "YOUR_INVOICE_CODE".to_string(),
        sender_invoice_no: "INV-001".to_string(),
        invoice_receiver_code: "terminal".to_string(),
        invoice_description: "Payment for order #001".to_string(),
        sender_branch_code: None,
        amount: 10000.0,
        callback_url: "https://example.com/callback".to_string(),
    };

    let invoice = client.create_simple_invoice(&req).await?;
    println!("Invoice ID: {}", invoice.invoice_id);
    println!("QR Image: {}", invoice.qr_image);
    println!("Short URL: {}", invoice.qpay_short_url);

    Ok(())
}
```

## Configuration

### From environment variables

```rust
let config = QPayConfig::from_env()?;
```

Required environment variables:

| Variable | Description |
|---|---|
| `QPAY_BASE_URL` | QPay API base URL (e.g., `https://merchant.qpay.mn`) |
| `QPAY_USERNAME` | QPay merchant username |
| `QPAY_PASSWORD` | QPay merchant password |
| `QPAY_INVOICE_CODE` | Default invoice code |
| `QPAY_CALLBACK_URL` | Payment callback URL |

### Manual configuration

```rust
let config = QPayConfig::new(
    "https://merchant.qpay.mn",
    "your_username",
    "your_password",
    "YOUR_INVOICE_CODE",
    "https://example.com/callback",
);
```

### Custom HTTP client

```rust
use std::time::Duration;

let http = reqwest::Client::builder()
    .timeout(Duration::from_secs(60))
    .build()
    .unwrap();

let client = QPayClient::with_http_client(config, http);
```

## Usage

### Authentication

Token management is fully automatic. The client obtains and refreshes tokens as needed before each request. You can also manage tokens manually:

```rust
// Get a new token
let token = client.get_token().await?;
println!("Access token: {}", token.access_token);

// Refresh the current token
let new_token = client.refresh_token().await?;
```

### Create an invoice (simple)

```rust
use qpay::models::CreateSimpleInvoiceRequest;

let req = CreateSimpleInvoiceRequest {
    invoice_code: "YOUR_INVOICE_CODE".to_string(),
    sender_invoice_no: "INV-001".to_string(),
    invoice_receiver_code: "terminal".to_string(),
    invoice_description: "Order payment".to_string(),
    sender_branch_code: None,
    amount: 50000.0,
    callback_url: "https://example.com/callback".to_string(),
};

let invoice = client.create_simple_invoice(&req).await?;
println!("Invoice ID: {}", invoice.invoice_id);
println!("QR text: {}", invoice.qr_text);

// Show deeplinks for bank apps
for url in &invoice.urls {
    println!("{}: {}", url.name, url.link);
}
```

### Create an invoice (full options)

```rust
use qpay::models::*;

let req = CreateInvoiceRequest {
    invoice_code: "YOUR_INVOICE_CODE".to_string(),
    sender_invoice_no: "INV-002".to_string(),
    sender_branch_code: Some("BRANCH_01".to_string()),
    sender_branch_data: Some(SenderBranchData {
        name: Some("Main Branch".to_string()),
        email: Some("branch@example.com".to_string()),
        ..Default::default()
    }),
    sender_staff_data: None,
    sender_staff_code: None,
    invoice_receiver_code: "terminal".to_string(),
    invoice_receiver_data: Some(InvoiceReceiverData {
        register: Some("AA12345678".to_string()),
        name: Some("Customer Name".to_string()),
        email: Some("customer@example.com".to_string()),
        phone: Some("99001122".to_string()),
        address: None,
    }),
    invoice_description: "Detailed invoice".to_string(),
    enable_expiry: Some("true".to_string()),
    allow_partial: Some(false),
    minimum_amount: None,
    allow_exceed: Some(false),
    maximum_amount: None,
    amount: 100000.0,
    callback_url: "https://example.com/callback".to_string(),
    sender_terminal_code: None,
    sender_terminal_data: None,
    allow_subscribe: None,
    subscription_interval: None,
    subscription_webhook: None,
    note: Some("Special instructions".to_string()),
    transactions: None,
    lines: Some(vec![
        InvoiceLine {
            tax_product_code: Some("TAX001".to_string()),
            line_description: "Product A".to_string(),
            line_quantity: "2".to_string(),
            line_unit_price: "50000".to_string(),
            note: None,
            discounts: None,
            surcharges: None,
            taxes: None,
        },
    ]),
};

let invoice = client.create_invoice(&req).await?;
```

### Create an invoice with ebarimt (tax)

```rust
use qpay::models::*;

let req = CreateEbarimtInvoiceRequest {
    invoice_code: "YOUR_INVOICE_CODE".to_string(),
    sender_invoice_no: "INV-TAX-001".to_string(),
    sender_branch_code: None,
    sender_staff_data: None,
    sender_staff_code: None,
    invoice_receiver_code: "terminal".to_string(),
    invoice_receiver_data: None,
    invoice_description: "Tax invoice".to_string(),
    tax_type: "1".to_string(),
    district_code: "23".to_string(),
    callback_url: "https://example.com/callback".to_string(),
    lines: vec![
        EbarimtInvoiceLine {
            tax_product_code: Some("TAX001".to_string()),
            line_description: "Taxable product".to_string(),
            barcode: None,
            line_quantity: "1".to_string(),
            line_unit_price: "10000".to_string(),
            note: None,
            classification_code: None,
            taxes: None,
        },
    ],
};

let invoice = client.create_ebarimt_invoice(&req).await?;
```

### Cancel an invoice

```rust
client.cancel_invoice("invoice_id_here").await?;
```

### Check payment status

```rust
use qpay::models::{PaymentCheckRequest, Offset};

let req = PaymentCheckRequest {
    object_type: "INVOICE".to_string(),
    object_id: "invoice_id_here".to_string(),
    offset: Some(Offset {
        page_number: 1,
        page_limit: 10,
    }),
};

let result = client.check_payment(&req).await?;
println!("Payment count: {}", result.count);

if let Some(amount) = result.paid_amount {
    println!("Total paid: {}", amount);
}

for row in &result.rows {
    println!("Payment {} - Status: {} - Amount: {}",
        row.payment_id, row.payment_status, row.payment_amount);
}
```

### Get payment details

```rust
let payment = client.get_payment("payment_id_here").await?;
println!("Status: {}", payment.payment_status);
println!("Amount: {} {}", payment.payment_amount, payment.payment_currency);
println!("Date: {}", payment.payment_date);
println!("Wallet: {}", payment.payment_wallet);
```

### List payments

```rust
use qpay::models::{PaymentListRequest, Offset};

let req = PaymentListRequest {
    object_type: "INVOICE".to_string(),
    object_id: "invoice_id_here".to_string(),
    start_date: "2026-01-01".to_string(),
    end_date: "2026-01-31".to_string(),
    offset: Offset {
        page_number: 1,
        page_limit: 20,
    },
};

let result = client.list_payments(&req).await?;
println!("Total: {}", result.count);
for item in &result.rows {
    println!("{}: {} {} ({})",
        item.payment_id, item.payment_amount,
        item.payment_currency, item.payment_status);
}
```

### Cancel a payment

```rust
use qpay::models::PaymentCancelRequest;

let req = PaymentCancelRequest {
    callback_url: Some("https://example.com/cancel-callback".to_string()),
    note: Some("Cancelled by customer request".to_string()),
};

client.cancel_payment("payment_id_here", &req).await?;
```

### Refund a payment

```rust
use qpay::models::PaymentRefundRequest;

let req = PaymentRefundRequest {
    callback_url: Some("https://example.com/refund-callback".to_string()),
    note: Some("Refund for order #001".to_string()),
};

client.refund_payment("payment_id_here", &req).await?;
```

### Create ebarimt (electronic tax receipt)

```rust
use qpay::models::CreateEbarimtRequest;

let req = CreateEbarimtRequest {
    payment_id: "payment_id_here".to_string(),
    ebarimt_receiver_type: "83".to_string(),  // "83" = individual, "80" = organization
    ebarimt_receiver: None,  // Set to register number for organizations
    district_code: Some("23".to_string()),
    classification_code: None,
};

let ebarimt = client.create_ebarimt(&req).await?;
println!("Ebarimt ID: {}", ebarimt.id);
println!("Lottery: {}", ebarimt.ebarimt_lottery);
println!("QR: {}", ebarimt.ebarimt_qr_data);
println!("Status: {}", ebarimt.barimt_status);
```

### Cancel ebarimt

```rust
let ebarimt = client.cancel_ebarimt("payment_id_here").await?;
println!("Cancelled: {}", ebarimt.barimt_status);
```

## Error Handling

All methods return `Result<T, QPayError>`. Error variants:

| Variant | Description |
|---|---|
| `QPayError::Api` | QPay API returned an error response (status code, error code, message) |
| `QPayError::Http` | Network/HTTP error from reqwest |
| `QPayError::Json` | JSON serialization/deserialization error |
| `QPayError::Config` | Configuration error (missing environment variable, etc.) |
| `QPayError::Token` | Token acquisition failed |

### Checking for API errors

```rust
use qpay::{is_qpay_error, error};

match client.create_simple_invoice(&req).await {
    Ok(invoice) => println!("Success: {}", invoice.invoice_id),
    Err(err) => {
        if let Some((status_code, code, message)) = is_qpay_error(&err) {
            println!("QPay error {}: {} - {}", status_code, code, message);

            match code {
                error::ERR_INVOICE_NOT_FOUND => println!("Invoice not found"),
                error::ERR_INVALID_AMOUNT => println!("Invalid amount"),
                error::ERR_AUTHENTICATION_FAILED => println!("Auth failed"),
                error::ERR_PERMISSION_DENIED => println!("Permission denied"),
                _ => println!("Other API error: {}", code),
            }
        } else {
            println!("Non-API error: {}", err);
        }
    }
}
```

### Error code constants

The `qpay::error` module exports all QPay error code constants for pattern matching:

```rust
use qpay::error::*;

// Invoice errors
ERR_INVOICE_NOT_FOUND       // "INVOICE_NOTFOUND"
ERR_INVOICE_PAID            // "INVOICE_PAID"
ERR_INVOICE_ALREADY_CANCELED // "INVOICE_ALREADY_CANCELED"
ERR_INVOICE_CODE_INVALID    // "INVOICE_CODE_INVALID"
ERR_INVOICE_LINE_REQUIRED   // "INVOICE_LINE_REQUIRED"

// Payment errors
ERR_PAYMENT_NOT_FOUND        // "PAYMENT_NOTFOUND"
ERR_PAYMENT_ALREADY_CANCELED // "PAYMENT_ALREADY_CANCELED"
ERR_PAYMENT_NOT_PAID         // "PAYMENT_NOT_PAID"

// Auth errors
ERR_AUTHENTICATION_FAILED   // "AUTHENTICATION_FAILED"
ERR_PERMISSION_DENIED       // "PERMISSION_DENIED"
ERR_NO_CREDENTIALS          // "NO_CREDENDIALS"

// Merchant errors
ERR_MERCHANT_NOT_FOUND      // "MERCHANT_NOTFOUND"
ERR_MERCHANT_INACTIVE       // "MERCHANT_INACTIVE"

// Ebarimt errors
ERR_EBARIMT_NOT_REGISTERED       // "EBARIMT_NOT_REGISTERED"
ERR_EBARIMT_CANCEL_NOT_SUPPORTED // "EBARIMT_CANCEL_NOTSUPPERDED"
ERR_EBARIMT_QR_CODE_INVALID      // "EBARIMT_QR_CODE_INVALID"

// Amount errors
ERR_INVALID_AMOUNT  // "INVALID_AMOUNT"
ERR_MIN_AMOUNT_ERR  // "MIN_AMOUNT_ERR"
ERR_MAX_AMOUNT_ERR  // "MAX_AMOUNT_ERR"
```

## API Reference

### `QPayConfig`

| Method | Description |
|---|---|
| `QPayConfig::new(base_url, username, password, invoice_code, callback_url)` | Create config with explicit values |
| `QPayConfig::from_env()` | Load config from environment variables |

### `QPayClient`

| Method | Description |
|---|---|
| `QPayClient::new(config)` | Create client with default HTTP settings |
| `QPayClient::with_http_client(config, http)` | Create client with custom `reqwest::Client` |

### Auth

| Method | Description |
|---|---|
| `client.get_token()` | Authenticate and get token pair |
| `client.refresh_token()` | Refresh the current access token |

### Invoice

| Method | Description |
|---|---|
| `client.create_invoice(&req)` | Create invoice with full options |
| `client.create_simple_invoice(&req)` | Create invoice with minimal fields |
| `client.create_ebarimt_invoice(&req)` | Create invoice with tax information |
| `client.cancel_invoice(id)` | Cancel an invoice |

### Payment

| Method | Description |
|---|---|
| `client.get_payment(id)` | Get payment details |
| `client.check_payment(&req)` | Check payment status for an invoice |
| `client.list_payments(&req)` | List payments with filters |
| `client.cancel_payment(id, &req)` | Cancel a payment (card only) |
| `client.refund_payment(id, &req)` | Refund a payment (card only) |

### Ebarimt

| Method | Description |
|---|---|
| `client.create_ebarimt(&req)` | Create electronic tax receipt |
| `client.cancel_ebarimt(payment_id)` | Cancel electronic tax receipt |

## License

MIT
