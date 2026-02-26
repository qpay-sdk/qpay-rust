use crate::client::QPayClient;
use crate::error::QPayError;
use crate::models::{
    CreateEbarimtInvoiceRequest, CreateInvoiceRequest, CreateSimpleInvoiceRequest,
    InvoiceResponse,
};

impl QPayClient {
    /// Create a detailed invoice with full options.
    /// POST /v2/invoice
    pub async fn create_invoice(
        &self,
        req: &CreateInvoiceRequest,
    ) -> Result<InvoiceResponse, QPayError> {
        self.do_request(reqwest::Method::POST, "/v2/invoice", Some(req))
            .await
    }

    /// Create a simple invoice with minimal fields.
    /// POST /v2/invoice
    pub async fn create_simple_invoice(
        &self,
        req: &CreateSimpleInvoiceRequest,
    ) -> Result<InvoiceResponse, QPayError> {
        self.do_request(reqwest::Method::POST, "/v2/invoice", Some(req))
            .await
    }

    /// Create an invoice with ebarimt (tax) information.
    /// POST /v2/invoice
    pub async fn create_ebarimt_invoice(
        &self,
        req: &CreateEbarimtInvoiceRequest,
    ) -> Result<InvoiceResponse, QPayError> {
        self.do_request(reqwest::Method::POST, "/v2/invoice", Some(req))
            .await
    }

    /// Cancel an existing invoice by ID.
    /// DELETE /v2/invoice/{id}
    pub async fn cancel_invoice(&self, invoice_id: &str) -> Result<(), QPayError> {
        let path = format!("/v2/invoice/{}", invoice_id);
        self.do_request_no_response::<()>(reqwest::Method::DELETE, &path, None)
            .await
    }
}
