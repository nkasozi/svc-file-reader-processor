use async_trait::async_trait;
use reqwest::StatusCode;

use crate::internal::{
    interfaces::svc_recon_tasks_handler::ReconTasksHandlerInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        file::{File, FileThatHasBeenRead},
    },
};

use super::{
    constants::{APP_ID_HEADER_NAME, CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE},
    view_models::responses::CreateReconTaskResponse,
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
        //format body and url
        //http://localhost:3602/v1.0/invoke/checkout/method/checkout/100
        let app_id = self.recon_tasks_service_name.clone();
        let host = self.dapr_grpc_server_address.clone();
        let url = format!("{host}/recon-tasks");
        let payload = serde_json::to_string(&_file).unwrap_or("".to_string());

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
            //successs
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let create_recon_task_response = resp.json::<CreateReconTaskResponse>().await;

                    match create_recon_task_response {
                        Ok(recon_task) => return Ok(recon_task.task_id),
                        Err(e) => {
                            return Err(AppError::new(
                                AppErrorKind::ResponseUnmarshalError,
                                e.to_string(),
                            ))
                        }
                    }
                }
                _ => Err(AppError::new(
                    AppErrorKind::ExternalServerError,
                    resp.text().await.unwrap_or("".to_string()),
                )),
            },
            //failure
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }

    async fn attach_primary_file_to_task(
        &self,
        file: &FileThatHasBeenRead,
    ) -> Result<(), AppError> {
        //format body and url
        //http://localhost:3602/v1.0/invoke/checkout/method/checkout/100
        let app_id = self.recon_tasks_service_name.clone();
        let host = self.dapr_grpc_server_address.clone();
        let url = format!("{host}/recon-tasks/attach-primary-file");
        let payload = serde_json::to_string(&file).unwrap_or("".to_string());

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
            //successs
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

    async fn attach_comparison_file_to_task(
        &self,
        file: &FileThatHasBeenRead,
    ) -> Result<(), AppError> {
        //format body and url
        //http://localhost:3602/v1.0/invoke/checkout/method/checkout/100
        let app_id = self.recon_tasks_service_name.clone();
        let host = self.dapr_grpc_server_address.clone();
        let url = format!("{host}/recon-tasks/attach-comparison-file");
        let payload = serde_json::to_string(&file).unwrap_or("".to_string());

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
            //successs
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

impl DaprSvcReconTasksHandler {
    async fn get_http_client(&self) -> Result<reqwest::Client, AppError> {
        return Ok(reqwest::Client::new());
    }
}
