use serde::{Deserialize, Serialize};

// --- Auth ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token_type: String,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub access_token: String,
    pub expires_in: i64,
    pub scope: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: String,
    pub session_state: String,
}

// --- Common nested types ---

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SenderBranchData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SenderStaffData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceReceiverData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_bank_code: String,
    pub account_number: String,
    pub iban_number: String,
    pub account_name: String,
    pub account_currency: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub description: String,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_product_code: Option<String>,
    pub line_description: String,
    pub line_quantity: String,
    pub line_unit_price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<TaxEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharges: Option<Vec<TaxEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<TaxEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbarimtInvoiceLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_product_code: Option<String>,
    pub line_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    pub line_quantity: String,
    pub line_unit_price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<TaxEntry>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharge_code: Option<String>,
    pub description: String,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deeplink {
    pub name: String,
    pub description: String,
    pub logo: String,
    pub link: String,
}

// --- Invoice ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub invoice_code: String,
    pub sender_invoice_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_branch_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_branch_data: Option<SenderBranchData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_staff_data: Option<SenderStaffData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_staff_code: Option<String>,
    pub invoice_receiver_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_receiver_data: Option<InvoiceReceiverData>,
    pub invoice_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_expiry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_exceed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_amount: Option<f64>,
    pub amount: f64,
    pub callback_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_terminal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_terminal_data: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_subscribe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_webhook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<Transaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<InvoiceLine>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSimpleInvoiceRequest {
    pub invoice_code: String,
    pub sender_invoice_no: String,
    pub invoice_receiver_code: String,
    pub invoice_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_branch_code: Option<String>,
    pub amount: f64,
    pub callback_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEbarimtInvoiceRequest {
    pub invoice_code: String,
    pub sender_invoice_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_branch_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_staff_data: Option<SenderStaffData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_staff_code: Option<String>,
    pub invoice_receiver_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_receiver_data: Option<InvoiceReceiverData>,
    pub invoice_description: String,
    pub tax_type: String,
    pub district_code: String,
    pub callback_url: String,
    pub lines: Vec<EbarimtInvoiceLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub invoice_id: String,
    pub qr_text: String,
    pub qr_image: String,
    #[serde(rename = "qPay_shortUrl")]
    pub qpay_short_url: String,
    pub urls: Vec<Deeplink>,
}

// --- Payment ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offset {
    pub page_number: i32,
    pub page_limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCheckRequest {
    pub object_type: String,
    pub object_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Offset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCheckResponse {
    pub count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<f64>,
    pub rows: Vec<PaymentCheckRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentCheckRow {
    pub payment_id: String,
    pub payment_status: String,
    pub payment_amount: String,
    pub trx_fee: String,
    pub payment_currency: String,
    pub payment_wallet: String,
    pub payment_type: String,
    pub next_payment_date: Option<String>,
    pub next_payment_datetime: Option<String>,
    #[serde(default)]
    pub card_transactions: Vec<CardTransaction>,
    #[serde(default)]
    pub p2p_transactions: Vec<P2PTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentDetail {
    pub payment_id: String,
    pub payment_status: String,
    pub payment_fee: String,
    pub payment_amount: String,
    pub payment_currency: String,
    pub payment_date: String,
    pub payment_wallet: String,
    pub transaction_type: String,
    pub object_type: String,
    pub object_id: String,
    pub next_payment_date: Option<String>,
    pub next_payment_datetime: Option<String>,
    #[serde(default)]
    pub card_transactions: Vec<CardTransaction>,
    #[serde(default)]
    pub p2p_transactions: Vec<P2PTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_merchant_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_terminal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    pub card_type: String,
    pub is_cross_border: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_status: Option<String>,
    pub settlement_status: String,
    pub settlement_status_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PTransaction {
    pub transaction_bank_code: String,
    pub account_bank_code: String,
    pub account_bank_name: String,
    pub account_number: String,
    pub status: String,
    pub amount: String,
    pub currency: String,
    pub settlement_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentListRequest {
    pub object_type: String,
    pub object_id: String,
    pub start_date: String,
    pub end_date: String,
    pub offset: Offset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentListResponse {
    pub count: i32,
    pub rows: Vec<PaymentListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentListItem {
    pub payment_id: String,
    pub payment_date: String,
    pub payment_status: String,
    pub payment_fee: String,
    pub payment_amount: String,
    pub payment_currency: String,
    pub payment_wallet: String,
    pub payment_name: String,
    pub payment_description: String,
    pub qr_code: String,
    pub paid_by: String,
    pub object_type: String,
    pub object_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentCancelRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentRefundRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

// --- Ebarimt ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEbarimtRequest {
    pub payment_id: String,
    pub ebarimt_receiver_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebarimt_receiver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbarimtResponse {
    pub id: String,
    pub ebarimt_by: String,
    pub g_wallet_id: String,
    pub g_wallet_customer_id: String,
    pub ebarimt_receiver_type: String,
    pub ebarimt_receiver: String,
    pub ebarimt_district_code: String,
    pub ebarimt_bill_type: String,
    pub g_merchant_id: String,
    pub merchant_branch_code: String,
    pub merchant_terminal_code: Option<String>,
    pub merchant_staff_code: Option<String>,
    pub merchant_register_no: String,
    pub g_payment_id: String,
    pub paid_by: String,
    pub object_type: String,
    pub object_id: String,
    pub amount: String,
    pub vat_amount: String,
    pub city_tax_amount: String,
    pub ebarimt_qr_data: String,
    pub ebarimt_lottery: String,
    pub note: Option<String>,
    pub barimt_status: String,
    pub barimt_status_date: String,
    pub ebarimt_sent_email: Option<String>,
    pub ebarimt_receiver_phone: String,
    pub tax_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_tin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebarimt_receipt_id: Option<String>,
    pub created_by: String,
    pub created_date: String,
    pub updated_by: String,
    pub updated_date: String,
    pub status: bool,
    #[serde(default)]
    pub barimt_items: Vec<EbarimtItem>,
    #[serde(default)]
    pub barimt_transactions: Vec<serde_json::Value>,
    #[serde(default)]
    pub barimt_histories: Vec<EbarimtHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbarimtItem {
    pub id: String,
    pub barimt_id: String,
    pub merchant_product_code: Option<String>,
    pub tax_product_code: String,
    pub bar_code: Option<String>,
    pub name: String,
    pub unit_price: String,
    pub quantity: String,
    pub amount: String,
    pub city_tax_amount: String,
    pub vat_amount: String,
    pub note: Option<String>,
    pub created_by: String,
    pub created_date: String,
    pub updated_by: String,
    pub updated_date: String,
    pub status: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbarimtHistory {
    pub id: String,
    pub barimt_id: String,
    pub ebarimt_receiver_type: String,
    pub ebarimt_receiver: String,
    pub ebarimt_register_no: Option<String>,
    pub ebarimt_bill_id: String,
    pub ebarimt_date: String,
    pub ebarimt_mac_address: String,
    pub ebarimt_internal_code: String,
    pub ebarimt_bill_type: String,
    pub ebarimt_qr_data: String,
    pub ebarimt_lottery: String,
    pub ebarimt_lottery_msg: Option<String>,
    pub ebarimt_error_code: Option<String>,
    pub ebarimt_error_msg: Option<String>,
    pub ebarimt_response_code: Option<String>,
    pub ebarimt_response_msg: Option<String>,
    pub note: Option<String>,
    pub barimt_status: String,
    pub barimt_status_date: String,
    pub ebarimt_sent_email: Option<String>,
    pub ebarimt_receiver_phone: String,
    pub tax_type: String,
    pub created_by: String,
    pub created_date: String,
    pub updated_by: String,
    pub updated_date: String,
    pub status: bool,
}
