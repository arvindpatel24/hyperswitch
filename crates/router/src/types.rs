// FIXME: Why were these data types grouped this way?
//
// Folder `types` is strange for Rust ecosystem, nevertheless it might be okay.
// But folder `enum` is even more strange I unlikely okay. Why should not we introduce folders `type`, `structs` and `traits`? :)
// Is it better to split data types according to business logic instead.
// For example, customers/address/dispute/mandate is "models".
// Separation of concerns instead of separation of forms.

pub mod api;
pub mod connector;
pub mod storage;

use std::marker::PhantomData;

pub use self::connector::Connector;
use self::{api::payments, storage::enums};
pub use crate::core::payments::PaymentAddress;
use crate::{core::errors::ApiErrorResponse, services};

pub type PaymentsRouterData = RouterData<api::Authorize, PaymentsRequestData, PaymentsResponseData>;
pub type PaymentsRouterSyncData =
    RouterData<api::PSync, PaymentsRequestSyncData, PaymentsResponseData>;
pub type PaymentsRouterCaptureData =
    RouterData<api::PCapture, PaymentsRequestCaptureData, PaymentsResponseData>;

pub type PaymentRouterCancelData =
    RouterData<api::Void, PaymentRequestCancelData, PaymentsResponseData>;
pub type RefundsRouterData<F> = RouterData<F, RefundsRequestData, RefundsResponseData>;
pub type PaymentsResponseRouterData<R> =
    ResponseRouterData<api::Authorize, R, PaymentsRequestData, PaymentsResponseData>;
pub type PaymentsCancelResponseRouterData<R> =
    ResponseRouterData<api::Void, R, PaymentRequestCancelData, PaymentsResponseData>;
pub type RefundsResponseRouterData<F, R> =
    ResponseRouterData<F, R, RefundsRequestData, RefundsResponseData>;

#[derive(Debug, Clone)]
pub struct RouterData<Flow, Request, Response> {
    pub flow: PhantomData<Flow>,
    pub merchant_id: String,
    pub connector: String,
    pub payment_id: String,
    pub status: enums::AttemptStatus,
    pub amount: i32,
    pub currency: enums::Currency,
    pub payment_method: enums::PaymentMethodType,
    pub connector_auth_type: ConnectorAuthType,
    pub description: Option<String>,
    pub return_url: Option<String>,
    pub address: PaymentAddress,
    pub auth_type: enums::AuthenticationType,

    /// Contains flow-specific data required to construct a request and send it to the connector.
    pub request: Request,

    /// Contains flow-specific data that the connector responds with.
    pub response: Option<Response>,

    /// Contains any error response that the connector returns.
    pub error_response: Option<ErrorResponse>,

    pub payment_method_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PaymentsRequestData {
    pub payment_method_data: payments::PaymentMethod,
    pub confirm: bool,
    pub statement_descriptor_suffix: Option<String>,
    // redirect form not used https://juspay.atlassian.net/browse/ORCA-301
    // pub redirection: Option<Redirection>,
    pub capture_method: Option<enums::CaptureMethod>,
    // Mandates
    pub setup_future_usage: Option<enums::FutureUsage>,
    pub mandate_id: Option<String>,
    pub off_session: Option<bool>,
    pub setup_mandate_details: Option<payments::MandateData>,
}

#[derive(Debug, Clone)]
pub struct PaymentsRequestCaptureData {
    pub amount_to_capture: Option<i32>,
    pub connector_transaction_id: String,
}

#[derive(Debug, Clone)]
pub struct PaymentsRequestSyncData {
    //TODO : add fields based on the connector requirements
    pub connector_transaction_id: String,
    pub encoded_data: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PaymentRequestCancelData {
    pub connector_transaction_id: String,
    pub cancellation_reason: Option<String>,
}
#[derive(Debug, Clone)]
pub struct PaymentsResponseData {
    pub connector_transaction_id: String,
    // pub amount_received: Option<i32>, // Calculation for amount received not in place yet
    pub redirection_data: Option<services::RedirectForm>,
    pub redirect: bool,
}

#[derive(Debug, Clone)]
pub struct RefundsRequestData {
    pub refund_id: String,
    pub payment_method_data: payments::PaymentMethod,
    pub connector_transaction_id: String,
    pub refund_amount: i32,
}

#[derive(Debug, Clone)]
pub struct RefundsResponseData {
    pub connector_refund_id: String,
    pub refund_status: enums::RefundStatus,
    // pub amount_received: Option<i32>, // Calculation for amount received not in place yet
}

#[derive(Debug, Clone, Copy)]
pub enum Redirection {
    Redirect,
    NoRedirect,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct ConnectorResponse {
    pub merchant_id: String,
    pub connector: String,
    pub payment_id: String,
    pub amount: i32,
    pub connector_transaction_id: String,
    pub return_url: Option<String>,
    pub three_ds_form: Option<services::RedirectForm>,
}
pub struct ResponseRouterData<Flow, R, Request, Response> {
    pub response: R,
    pub data: RouterData<Flow, Request, Response>,
    pub http_code: u16,
}

// Different patterns of authentication.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "auth_type")]
pub enum ConnectorAuthType {
    HeaderKey { api_key: String },
    BodyKey { api_key: String, key1: String },
}

impl Default for ConnectorAuthType {
    fn default() -> Self {
        Self::HeaderKey {
            api_key: "".to_string(),
        }
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectorsList {
    pub connectors: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub response: bytes::Bytes,
    pub status_code: u16,
}

#[derive(Clone, Debug)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub reason: Option<String>,
}

impl ErrorResponse {
    pub fn get_not_implemented() -> Self {
        Self {
            code: ApiErrorResponse::NotImplemented.error_code(),
            message: ApiErrorResponse::NotImplemented.error_message(),
            reason: None,
        }
    }
}