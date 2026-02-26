use crate::client::QPayClient;
use crate::error::QPayError;
use crate::models::{
    PaymentCancelRequest, PaymentCheckRequest, PaymentCheckResponse, PaymentDetail,
    PaymentListRequest, PaymentListResponse, PaymentRefundRequest,
};

impl QPayClient {
    /// Retrieve payment details by payment ID.
    /// GET /v2/payment/{id}
    pub async fn get_payment(&self, payment_id: &str) -> Result<PaymentDetail, QPayError> {
        let path = format!("/v2/payment/{}", payment_id);
        self.do_request::<(), PaymentDetail>(reqwest::Method::GET, &path, None)
            .await
    }

    /// Check if a payment has been made for an invoice.
    /// POST /v2/payment/check
    pub async fn check_payment(
        &self,
        req: &PaymentCheckRequest,
    ) -> Result<PaymentCheckResponse, QPayError> {
        self.do_request(reqwest::Method::POST, "/v2/payment/check", Some(req))
            .await
    }

    /// Return a list of payments matching the given criteria.
    /// POST /v2/payment/list
    pub async fn list_payments(
        &self,
        req: &PaymentListRequest,
    ) -> Result<PaymentListResponse, QPayError> {
        self.do_request(reqwest::Method::POST, "/v2/payment/list", Some(req))
            .await
    }

    /// Cancel a payment (card transactions only).
    /// DELETE /v2/payment/cancel/{id}
    pub async fn cancel_payment(
        &self,
        payment_id: &str,
        req: &PaymentCancelRequest,
    ) -> Result<(), QPayError> {
        let path = format!("/v2/payment/cancel/{}", payment_id);
        self.do_request_no_response(reqwest::Method::DELETE, &path, Some(req))
            .await
    }

    /// Refund a payment (card transactions only).
    /// DELETE /v2/payment/refund/{id}
    pub async fn refund_payment(
        &self,
        payment_id: &str,
        req: &PaymentRefundRequest,
    ) -> Result<(), QPayError> {
        let path = format!("/v2/payment/refund/{}", payment_id);
        self.do_request_no_response(reqwest::Method::DELETE, &path, Some(req))
            .await
    }
}
