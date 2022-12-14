use async_trait::async_trait;

use crate::internal::{
    interfaces::recon_tasks_service_connector::ReconTasksServiceConnectorInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::AppError,
        file::FileThatHasBeenRead,
    },
};
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::FileMetadata;
use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{ComparisonPair, ReconciliationConfigs, ReconFileType};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::interfaces::recon_tasks_microservice::ReconTasksMicroserviceClientInterface;
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::{AttachComparisonFileRequest, AttachPrimaryFileRequest, CreateReconTaskRequest};

pub struct ReconTasksServiceConnector {
    recon_tasks_microservice_client: Box<dyn ReconTasksMicroserviceClientInterface>,
}

#[async_trait]
impl ReconTasksServiceConnectorInterface for ReconTasksServiceConnector {
    async fn create_recon_task(&self, _file: &FileThatHasBeenRead) -> Result<String, AppError> {
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

        let result = self.recon_tasks_microservice_client.create_recon_task(&request).await?;
        return Ok(result.task_id);
    }

    async fn attach_primary_file_to_task(
        &self,
        file: &FileThatHasBeenRead,
    ) -> Result<String, AppError> {
        let request = AttachPrimaryFileRequest {
            task_id: Self::get_recon_task_id(&file.clone()),
            primary_file_name: Self::get_file_name(&file.clone()),
            primary_file_hash: Self::get_file_hash(&file.clone()),
            primary_file_row_count: file.file_rows.len() as u64,
            primary_file_headers: file.column_headers.clone(),
            primary_file_delimiters: Self::get_column_delimiters(file.file_metadata.clone()),
        };

        let result = self.recon_tasks_microservice_client.attach_primary_file_to_task(&request).await?;
        return Ok(result.task_id);
    }

    async fn attach_comparison_file_to_task(
        &self,
        file: &FileThatHasBeenRead,
    ) -> Result<String, AppError> {
        let request = AttachComparisonFileRequest {
            task_id: Self::get_recon_task_id(&file.clone()),
            comparison_file_name: Self::get_file_name(&file.clone()),
            comparison_file_hash: Self::get_file_hash(&file.clone()),
            comparison_file_row_count: file.file_rows.len() as u64,
            comparison_file_headers: file.column_headers.clone(),
            comparison_file_delimiters: Self::get_column_delimiters(file.file_metadata.clone()),
        };

        let result = self.recon_tasks_microservice_client.attach_comparison_file_to_task(&request).await?;
        return Ok(result.task_id);
    }
}

impl ReconTasksServiceConnector {
    pub(crate) fn new(recon_tasks_microservice_client: Box<dyn ReconTasksMicroserviceClientInterface>) -> ReconTasksServiceConnector {
        return ReconTasksServiceConnector {
            recon_tasks_microservice_client
        };
    }

    fn get_file_name(file: &FileThatHasBeenRead) -> String {
        return match file.clone().file_type {
            ReconFileType::PrimaryFile => {
                format!("PRIMARY-FILE-{}", file.clone().upload_request_id.unwrap_or("".to_string()))
            }
            ReconFileType::ComparisonFile => {
                format!("COMPARISON-FILE-{}", file.clone().upload_request_id.unwrap_or("".to_string()))
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
                format!("PRIMARY-FILE-{}", file.clone().upload_request_id.unwrap_or("".to_string()))
            }
            ReconFileType::ComparisonFile => {
                format!("COMPARISON-FILE-{}", file.clone().upload_request_id.unwrap_or("".to_string()))
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

    fn get_user_id() -> String {
        "nkasozi@gmail.com".to_string()
    }
}
