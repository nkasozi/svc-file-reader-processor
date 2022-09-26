use async_trait::async_trait;
use reqwest::{Error, Response, StatusCode};
use serde::Serialize;
use validator::HasLen;

use crate::external::services::view_models::requests::{AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest};
use crate::internal::{
    interfaces::svc_recon_tasks_handler::ReconTasksHandlerInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        file::FileThatHasBeenRead,
    },
};
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::FileMetadata;
use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{ComparisonPair, ReconciliationConfigs, ReconFileType};
use crate::internal::shared_reconciler_rust_libraries::models::view_models::recon_task_response_details::{FileResponseSummary, ReconTaskResponseDetails};

use super::{
    constants::{APP_ID_HEADER_NAME, CONTENT_TYPE_HEADER_NAME, CONTENT_TYPE_HEADER_VALUE},
};

pub struct DaprSvcReconTasksHandler {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub recon_tasks_service_app_id: String,
}

#[async_trait]
impl ReconTasksHandlerInterface for DaprSvcReconTasksHandler {
    async fn create_recon_task(&self, _file: &FileThatHasBeenRead) -> Result<String, AppError> {
        //format body and url
        //http://localhost:3602/v1.0/invoke/checkout/method/checkout/100
        let app_id = self.recon_tasks_service_app_id.clone();
        let host = self.dapr_grpc_server_address.clone();
        let url = format!("{host}/recon-tasks");
        let request = CreateReconTaskRequest {
            user_id: Self::get_user_id(),
            recon_configurations: ReconciliationConfigs {
                should_check_for_duplicate_records_in_comparison_file: false,
                should_reconciliation_be_case_sensitive: false,
                should_ignore_white_space: false,
                should_do_reverse_reconciliation: false,
            },
            comparison_pairs: Self::get_comparison_pairs(_file.clone().file_metadata),
        };

        let payload = Self::convert_to_json_string(&request);

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

        return Self::parse_create_recon_task_response(http_response).await;
    }

    async fn attach_primary_file_to_task(
        &self,
        file: &FileThatHasBeenRead,
    ) -> Result<String, AppError> {
        //format body and url
        let app_id = self.recon_tasks_service_app_id.clone();
        let host = self.dapr_grpc_server_address.clone();
        let url = format!("{host}/recon-tasks/attach-primary-file");
        let request = AttachPrimaryFileRequest {
            task_id: Self::get_recon_task_id(&file.clone()),
            primary_file_name: Self::get_file_name(&file.clone()),
            primary_file_hash: Self::get_file_hash(&file.clone()),
            primary_file_row_count: file.file_rows.length(),
            primary_file_headers: file.column_headers.clone(),
            primary_file_delimiters: Self::get_column_delimiters(file.file_metadata.clone()),
        };

        let payload = Self::convert_to_json_string(&request);

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

        return Self::parse_attach_file_response(http_response).await;
    }

    async fn attach_comparison_file_to_task(
        &self,
        file: &FileThatHasBeenRead,
    ) -> Result<String, AppError> {
        //format body and url
        let app_id = self.recon_tasks_service_app_id.clone();
        let host = self.dapr_grpc_server_address.clone();
        let url = format!("{host}/recon-tasks/attach-comparison-file");
        let request = AttachComparisonFileRequest {
            task_id: Self::get_recon_task_id(&file.clone()),
            comparison_file_name: Self::get_file_name(&file.clone()),
            comparison_file_hash: Self::get_file_hash(&file.clone()),
            comparison_file_row_count: file.file_rows.length(),
            comparison_file_headers: file.column_headers.clone(),
            comparison_file_delimiters: Self::get_column_delimiters(file.file_metadata.clone()),
        };

        let payload = Self::convert_to_json_string(&request);

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

        return Self::parse_attach_file_response(http_response).await;
    }
}

impl DaprSvcReconTasksHandler {
    async fn get_http_client(&self) -> Result<reqwest::Client, AppError> {
        return Ok(reqwest::Client::new());
    }

    fn get_file_name(file: &FileThatHasBeenRead) -> String {
        return match file.clone().file_type {
            ReconFileType::PrimaryFile => {
                format!("PRIMARY-FILE-{}", file.upload_request_id.unwrap_or("".to_string()))
            }
            ReconFileType::ComparisonFile => {
                format!("COMPARISON-FILE-{}", file.upload_request_id.unwrap_or("".to_string()))
            }
        };
    }

    fn get_recon_task_id(file: &FileThatHasBeenRead) -> String {
        return match file.clone().upload_request_id {
            None => { "".to_string() }
            Some(task_id) => { task_id }
        };
    }

    fn get_file_hash(file: &FileThatHasBeenRead) -> String {
        return match file.clone().file_type {
            ReconFileType::PrimaryFile => {
                format!("PRIMARY-FILE-{}", file.upload_request_id.unwrap_or("".to_string()))
            }
            ReconFileType::ComparisonFile => {
                format!("COMPARISON-FILE-{}", file.upload_request_id.unwrap_or("".to_string()))
            }
        };
    }

    fn get_column_delimiters(file_metadata: Option<FileMetadata>) -> Vec<char> {
        let default_column_delimiters = vec![','];
        return match file_metadata {
            None => {
                default_column_delimiters
            }
            Some(metadata) => {
                match metadata.column_delimiters {
                    None => {
                        default_column_delimiters
                    }
                    Some(column_delimiters) => {
                        column_delimiters
                    }
                }
            }
        };
    }

    fn get_comparison_pairs(file_metadata: Option<FileMetadata>) -> Vec<ComparisonPair> {
        let no_comparison_pairs = vec![];
        return match file_metadata {
            None => {
                no_comparison_pairs
            }
            Some(metadata) => {
                match metadata.comparison_pairs {
                    None => {
                        no_comparison_pairs
                    }
                    Some(comparison_pairs) => {
                        comparison_pairs
                    }
                }
            }
        };
    }

    async fn parse_attach_file_response(http_response: Result<Response, Error>) -> Result<String, AppError> {
        match http_response {
            //success
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let service_response = resp.json::<FileResponseSummary>().await;

                    return match service_response {
                        Ok(file_info) => Ok(file_info.task_id),
                        Err(e) => {
                            Err(AppError::new(
                                AppErrorKind::ResponseUnmarshalError,
                                e.to_string(),
                            ))
                        }
                    };
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

    async fn parse_create_recon_task_response(http_response: Result<Response, Error>) -> Result<String, AppError> {
        match http_response {
            //success
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let create_recon_task_response = resp.json::<ReconTaskResponseDetails>().await;

                    return match create_recon_task_response {
                        Ok(recon_task) => Ok(recon_task.task_id),
                        Err(e) => {
                            Err(AppError::new(
                                AppErrorKind::ResponseUnmarshalError,
                                e.to_string(),
                            ))
                        }
                    };
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

    fn convert_to_json_string<T>(request: &T) -> String where T: ?Sized + Serialize {
        serde_json::to_string(request).unwrap_or("".to_string())
    }

    fn get_user_id() -> String {
        "nkasozi@gmail.com".to_string()
    }
}
