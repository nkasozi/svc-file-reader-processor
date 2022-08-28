use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::svc_recon_tasks_handler::ReconTasksHandlerInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        file::{File, FileThatHasBeenRead},
    },
};

pub struct DaprSvcReconTasksHandler {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub recon_tasks_service_name: String,
}

#[async_trait]
impl ReconTasksHandlerInterface for DaprSvcReconTasksHandler {
    async fn create_recon_task(&self, _file: &File) -> Result<String, AppError> {
        //create a dapr client
        let _ = self.get_dapr_connection().await?;
        Ok(String::from(""))
    }

    async fn attach_primary_file_to_task(
        &self,
        _file: &FileThatHasBeenRead,
    ) -> Result<(), AppError> {
        Ok(())
    }

    async fn attach_comparison_file_to_task(
        &self,
        _file: &FileThatHasBeenRead,
    ) -> Result<(), AppError> {
        Ok(())
    }
}

impl DaprSvcReconTasksHandler {
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
