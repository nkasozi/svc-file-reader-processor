use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::pubsub::PubSubInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        reconstructed_file::ReconstructedFile,
    },
};

pub struct DaprPubSub {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub dapr_component_name: String,
}

#[async_trait]
impl PubSubInterface for DaprPubSub {
    async fn publish_to_reconciled_files_queue(
        &self,
        _queue: &ReconstructedFile,
    ) -> Result<bool, AppError> {
        let _ = self.get_dapr_connection();
        Ok(true)
    }
}

impl DaprPubSub {
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
