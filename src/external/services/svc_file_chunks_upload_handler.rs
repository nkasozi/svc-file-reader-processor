use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::svc_file_chunks_uploader::FileChunksUploaderInterface,
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::entities::app_errors::{AppError, AppErrorKind},
};

pub struct DaprSvcFileChunksUploader {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub file_chunks_uploader_service_name: String,
}

#[async_trait]
impl FileChunksUploaderInterface for DaprSvcFileChunksUploader {
    async fn upload_file_chunk(
        &self,
        _file_upload_chunk: &UploadFileChunkRequest,
    ) -> Result<(), AppError> {
        //create a dapr client
        let _ = self.get_dapr_connection().await?;

        //call the binding
        // let url = format!("/upload-file-chunk",);
        // let binding_response = client
        //     .invoke_method(
        //         self.file_chunks_uploader_service_name.clone(),
        //         url,
        //         file_upload_chunk.clone(),
        //     )
        //     .await;

        // //handle the bindings response
        // match binding_response {
        //     //successs
        //     Ok(_) => return Ok(()),
        //     //failure
        //     Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        // }
        Ok(())
    }
}

impl DaprSvcFileChunksUploader {
    async fn get_dapr_connection(&self) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
        // Create the client
        let dapr_grpc_server_address = self.dapr_grpc_server_address.clone();

        //connect to dapr
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(dapr_grpc_server_address).await;

        //handle the connection result
        match client_connect_result {
            //connection succeeded
            Ok(s) => return Ok(s),
            //connection failed
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }
}
