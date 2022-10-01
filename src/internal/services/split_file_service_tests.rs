use crate::internal::interfaces::file_chunks_upload_service_connector::MockFileChunksUploadHandlerServiceConnectorInterface;
use crate::internal::interfaces::file_reader::MockFileReader;
use crate::internal::interfaces::recon_tasks_service_connector::MockReconTasksServiceConnectorInterface;
use crate::internal::interfaces::split_file_service::SplitFileServiceInterface;
use crate::internal::interfaces::transformer::MockTransformerInterface;
use crate::internal::models::view_models::requests::split_file_request::SplitFileRequest;
use crate::internal::models::view_models::responses::split_file_response::SplitFileResponse;
use crate::internal::services::split_file_service::SplitFileService;
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppError;
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind;
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::{File, FileMetadata, FileStorageLocation, FileThatHasBeenRead, SupportedFileExtension};
use crate::internal::shared_reconciler_rust_libraries::models::entities::file_upload_chunk::FileUploadChunkSource;
use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{ComparisonPair, ReconFileType};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;

//specifies the request, expected mock responses from dependencies and
//the expected final method response for
//a particular test case
#[derive(Clone, Debug)]
struct TestSpecifications {
    request: SplitFileRequest,
    mock_read_file_result: Option<Result<FileThatHasBeenRead, AppError>>,
    mock_create_recon_task_result: Option<Result<String, AppError>>,
    mock_attach_comparison_file_result: Option<Result<String, AppError>>,
    mock_group_rows_into_file_chunks_result: Option<Vec<UploadFileChunkRequest>>,
    mock_upload_file_chunk_result: Option<Result<(), AppError>>,
    expected_final_result: Result<SplitFileResponse, AppError>,
}

//holds a number of test
//cases testing different scenarios
#[derive(Clone, Debug)]
struct ValidRequestsTestScenarios {
    ok_test: TestSpecifications,
    is_read_file_error_handled: TestSpecifications,
    is_create_recon_task_error_handled: TestSpecifications,
    is_attach_file_to_task_error_handled: TestSpecifications,
    is_upload_file_chunk_error_handled: TestSpecifications,
}

#[test]
fn test_read_and_split_file_handler_valid_request() {
    let ok_test_specification = generate_ok_test_specification();
    let valid_requests_test_suite = ValidRequestsTestScenarios {
        ok_test: TestSpecifications {
            ..ok_test_specification.clone()
        },
        is_read_file_error_handled: TestSpecifications {
            mock_read_file_result: Some(dummy_error(AppErrorKind::InternalError)),
            mock_create_recon_task_result: None,
            mock_attach_comparison_file_result: None,
            mock_group_rows_into_file_chunks_result: None,
            mock_upload_file_chunk_result: None,
            expected_final_result: dummy_error(AppErrorKind::InternalError),
            ..ok_test_specification.clone()
        },
        is_create_recon_task_error_handled: TestSpecifications {
            mock_create_recon_task_result: Some(dummy_error(AppErrorKind::InternalError)),
            mock_attach_comparison_file_result: None,
            mock_group_rows_into_file_chunks_result: None,
            mock_upload_file_chunk_result: None,
            expected_final_result: dummy_error(AppErrorKind::InternalError),
            ..ok_test_specification.clone()
        },
        is_attach_file_to_task_error_handled: TestSpecifications {
            mock_attach_comparison_file_result: Some(dummy_error(AppErrorKind::InternalError)),
            mock_group_rows_into_file_chunks_result: None,
            mock_upload_file_chunk_result: None,
            expected_final_result: dummy_error(AppErrorKind::InternalError),
            ..ok_test_specification.clone()
        },
        is_upload_file_chunk_error_handled: TestSpecifications {
            mock_upload_file_chunk_result: Some(dummy_error(AppErrorKind::InternalError)),
            expected_final_result: dummy_error(AppErrorKind::InternalError),
            ..ok_test_specification.clone()
        },
    };

    rspec::run(&rspec::given("valid client request", valid_requests_test_suite, |ctx| {
        ctx.when("all dependencies return OK", |ctx| {
            ctx.then("returns OK", |env| {
                let resp = setup_service_and_send_request(&env.ok_test.clone());
                assert_eq!(resp, env.ok_test.expected_final_result.clone())
            });
        });

        ctx.when("read file returns an error", |ctx| {
            ctx.then("method returns the same error", |env| {
                let resp = setup_service_and_send_request(&env.is_read_file_error_handled.clone());
                assert_eq!(resp, env.is_read_file_error_handled.expected_final_result.clone())
            });
        });

        ctx.when("create recon task returns an error", |ctx| {
            ctx.then("method returns the same error", |env| {
                let resp = setup_service_and_send_request(&env.is_create_recon_task_error_handled.clone());
                assert_eq!(resp, env.is_create_recon_task_error_handled.expected_final_result.clone())
            });
        });

        ctx.when("attach comparison file to task returns an error", |ctx| {
            ctx.then("method returns the same error", |env| {
                let resp = setup_service_and_send_request(&env.is_attach_file_to_task_error_handled.clone());
                assert_eq!(resp, env.is_attach_file_to_task_error_handled.expected_final_result.clone())
            });
        });


        ctx.when("upload file chunks returns an error", |ctx| {
            ctx.then("method returns the same error", |env| {
                let resp = setup_service_and_send_request(&env.is_upload_file_chunk_error_handled.clone());
                assert_eq!(resp, env.is_upload_file_chunk_error_handled.expected_final_result.clone())
            });
        });
    }));
}

fn generate_ok_test_specification() -> TestSpecifications {
    TestSpecifications {
        request: get_dummy_request(),
        mock_read_file_result: Some(dummy_file_that_has_been_read()),
        mock_create_recon_task_result: Some(Ok(String::from("RECON-TASK-1234"))),
        mock_attach_comparison_file_result: Some(Ok(String::from("RECON-TASK-1234"))),
        mock_group_rows_into_file_chunks_result: Some(vec![UploadFileChunkRequest {
            upload_request_id: "1234".to_string(),
            chunk_sequence_number: 1,
            chunk_source: FileUploadChunkSource::ComparisonFileChunk,
            chunk_rows: vec![],
            is_last_chunk: false,
        }]),
        mock_upload_file_chunk_result: Some(Ok(())),
        expected_final_result: Ok(SplitFileResponse {
            upload_request_id: String::from("RECON-TASK-1234"),
        }),
    }
}


fn dummy_error<V>(app_error_kind: AppErrorKind) -> Result<V, AppError> {
    Err(AppError::new(
        app_error_kind, "error occurred".to_string(),
    ))
}

fn dummy_file_that_has_been_read() -> Result<FileThatHasBeenRead, AppError> {
    Ok(
        FileThatHasBeenRead {
            id: None,
            upload_request_id: None,
            file_type: ReconFileType::PrimaryFile,
            column_headers: vec![],
            file_rows: vec![],
            file_metadata: None,
        })
}


fn setup_service_and_send_request(test_specifications: &TestSpecifications) -> Result<SplitFileResponse, AppError> {
    let mut mock_file_reader = Box::new(MockFileReader::new());
    let mut mock_transformer = Box::new(MockTransformerInterface::new());
    let mut mock_file_chunks_uploader = Box::new(MockFileChunksUploadHandlerServiceConnectorInterface::new());
    let mut mock_recon_tasks_repo_handler = Box::new(MockReconTasksServiceConnectorInterface::new());

    //setup mock responses
    match test_specifications.clone().mock_read_file_result {
        None => {}
        Some(result) => {
            mock_file_reader.expect_read_file().returning(move |_y| {
                result.clone()
            });
        }
    }

    match test_specifications.clone().mock_create_recon_task_result {
        None => {}
        Some(result) => {
            mock_recon_tasks_repo_handler.expect_create_recon_task().returning(move |_y| {
                result.clone()
            });
        }
    }

    match test_specifications.clone().mock_attach_comparison_file_result {
        None => {}
        Some(result) => {
            mock_recon_tasks_repo_handler.expect_attach_comparison_file_to_task().returning(move |_y| {
                result.clone()
            });
        }
    }

    match test_specifications.clone().mock_group_rows_into_file_chunks_result {
        None => {}
        Some(result) => {
            mock_transformer.expect_group_rows_into_file_chunks().returning(move |_y, _x| {
                result.clone()
            });
        }
    }

    match test_specifications.clone().mock_upload_file_chunk_result {
        None => {}
        Some(result) => {
            mock_file_chunks_uploader.expect_upload_file_chunk().returning(move |_y| {
                result.clone()
            });
        }
    }

    let sut = SplitFileService {
        file_reader: mock_file_reader,
        transformer: mock_transformer,
        file_chunks_uploader: mock_file_chunks_uploader,
        recon_tasks_handler: mock_recon_tasks_repo_handler,
    };

    let result = tokio_test::block_on(sut.read_and_split_file_into_chunks(test_specifications.clone().request));

    return result;
}

fn get_dummy_request() -> SplitFileRequest {
    SplitFileRequest {
        file: File {
            id: None,
            upload_request_id: None,
            file_storage_location: FileStorageLocation::LocalFileSystem,
            file_extension: SupportedFileExtension::Csv,
            file_metadata: Some(FileMetadata {
                column_delimiters: None,
                comparison_pairs: Some(vec![ComparisonPair {
                    primary_file_column_index: 0,
                    comparison_file_column_index: 0,
                    is_row_identifier: true,
                }]),
            }),
            file_path: Some("E:/Work/test.csv".to_string()),
            file_type: ReconFileType::ComparisonFile,
        },
    }
}