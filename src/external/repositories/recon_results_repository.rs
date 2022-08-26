use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::recon_results_repository::ReconResultsRepositoryInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        reconstructed_file::ReconstructedFile,
    },
};

pub struct ReconResultsRepository {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub dapr_component_name: String,

    //the dapr state store name
    pub dapr_state_store_name: String,
}

#[async_trait]
impl ReconResultsRepositoryInterface for ReconResultsRepository {
    async fn get_reconstructed_file(
        &self,
        _file_id: &String,
    ) -> Result<Option<ReconstructedFile>, AppError> {
        let _ = self.get_dapr_connection();
        Ok(Option::None)
    }

    async fn save_reconstructed_file(
        &self,
        _reconstructed_file: &ReconstructedFile,
    ) -> Result<bool, AppError> {
        let _ = self.get_dapr_connection();
        Ok(true)
    }
}

impl ReconResultsRepository {
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
