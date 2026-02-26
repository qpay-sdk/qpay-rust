//! QPay V2 API SDK for Rust.
//!
//! This crate provides an async client for the QPay V2 payment API with
//! automatic token management, invoice creation, payment operations, and
//! ebarimt (electronic tax receipt) support.
//!
//! # Example
//!
//! ```no_run
//! use qpay::{QPayClient, QPayConfig, models::CreateSimpleInvoiceRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), qpay::QPayError> {
//!     let config = QPayConfig::from_env()?;
//!     let client = QPayClient::new(config);
//!
//!     let req = CreateSimpleInvoiceRequest {
//!         invoice_code: "INVOICE_CODE".to_string(),
//!         sender_invoice_no: "INV-001".to_string(),
//!         invoice_receiver_code: "terminal".to_string(),
//!         invoice_description: "Test invoice".to_string(),
//!         sender_branch_code: None,
//!         amount: 1000.0,
//!         callback_url: "https://example.com/callback".to_string(),
//!     };
//!
//!     let invoice = client.create_simple_invoice(&req).await?;
//!     println!("Invoice ID: {}", invoice.invoice_id);
//!
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod client;
pub mod config;
pub mod ebarimt;
pub mod error;
pub mod invoice;
pub mod models;
pub mod payment;

pub use client::QPayClient;
pub use config::QPayConfig;
pub use error::{is_qpay_error, QPayError};
