use crate::client::QPayClient;
use crate::error::QPayError;
use crate::models::{CreateEbarimtRequest, EbarimtResponse};

impl QPayClient {
    /// Create an ebarimt (electronic tax receipt) for a payment.
    /// POST /v2/ebarimt_v3/create
    pub async fn create_ebarimt(
        &self,
        req: &CreateEbarimtRequest,
    ) -> Result<EbarimtResponse, QPayError> {
        self.do_request(reqwest::Method::POST, "/v2/ebarimt_v3/create", Some(req))
            .await
    }

    /// Cancel an ebarimt by payment ID.
    /// DELETE /v2/ebarimt_v3/{id}
    pub async fn cancel_ebarimt(
        &self,
        payment_id: &str,
    ) -> Result<EbarimtResponse, QPayError> {
        let path = format!("/v2/ebarimt_v3/{}", payment_id);
        self.do_request::<(), EbarimtResponse>(reqwest::Method::DELETE, &path, None)
            .await
    }
}
