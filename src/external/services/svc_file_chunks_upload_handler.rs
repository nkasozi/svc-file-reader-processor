use async_trait::async_trait;
use reqwest::StatusCode;

use crate::internal::{
    interfaces::svc_file_chunks_uploader::FileChunksUploaderInterface,
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind},
};

use super::constants::{APP_ID_HEADER_NAME, CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE};

pub struct DaprSvcFileChunksUploadHandler {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub file_chunks_service_app_id: String,
}

#[async_trait]
impl FileChunksUploaderInterface for DaprSvcFileChunksUploadHandler {
    async fn upload_file_chunk(
        &self,
        file_upload_chunk: &UploadFileChunkRequest,
    ) -> Result<(), AppError> {
        //format body and url
        let app_id = self.file_chunks_service_app_id.clone();
        let host = self.dapr_grpc_server_address.clone();
        let url = format!("{host}/upload-file-chunk");
        let payload = serde_json::to_string(&file_upload_chunk).unwrap_or("".to_string());

        //create client
        let client = self.get_http_client().await?;

        //send request
        let http_response = client
            .post(url)
            .body(payload)
            .header(CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE)
            .header(APP_ID_HEADER_NAME, app_id)
            .send()
            .await;

        // //handle the bindings response
        match http_response {
            //success
            Ok(resp) => match resp.status() {
                StatusCode::OK => Ok(()),
                _ => Err(AppError::new(
                    AppErrorKind::ExternalServerError,
                    resp.text().await.unwrap_or("".to_string()),
                )),
            },
            //failure
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }
}

impl DaprSvcFileChunksUploadHandler {
    async fn get_http_client(&self) -> Result<reqwest::Client, AppError> {
        return Ok(reqwest::Client::new());
    }
}
