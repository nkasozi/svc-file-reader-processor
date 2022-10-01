use crate::internal::interfaces::transformer::TransformerInterface;
use crate::internal::services::core_logic::transformer::Transformer;
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::{FileMetadata, FileThatHasBeenRead};
use crate::internal::shared_reconciler_rust_libraries::models::entities::file_row::FileRow;
use crate::internal::shared_reconciler_rust_libraries::models::entities::file_upload_chunk::FileUploadChunkSource;
use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::{ComparisonPair, ReconFileType};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;

//specifies the request, expected mock responses from dependencies and
//the expected final method response for
//a particular test case
#[derive(Clone, Debug)]
struct TestSpecifications {
    request: (FileThatHasBeenRead, i64),
    expected_final_result: Vec<UploadFileChunkRequest>,
}

//holds a number of test
//cases testing different scenarios
#[derive(Clone, Debug)]
struct ValidRequestsTestScenarios {
    ok_test: TestSpecifications,
    is_max_rows_less_than_file_rows_handled_correctly: TestSpecifications,
}

#[test]
fn test_group_rows_into_file_chunks() {
    let valid_requests_test_suite = ValidRequestsTestScenarios {
        ok_test: generate_ok_test_specification(),
        is_max_rows_less_than_file_rows_handled_correctly: generate_is_max_rows_less_than_file_rows_handled_correctly_test_specification(),
    };

    rspec::run(&rspec::given("a FileThatHasBeenRead and max num of rows", valid_requests_test_suite, |ctx| {
        ctx.when("the supplied max number of rows is MORE than those in the FileThatHasBeenRead", |ctx| {
            ctx.then("correctly groups rows into One FileChunk", |env| {
                let resp = setup_service_and_send_request(&env.ok_test.clone());
                assert_eq!(resp.len(), env.ok_test.expected_final_result.clone().len());
                assert_eq!(resp.last(), env.ok_test.expected_final_result.clone().last())
            });
        });
        ctx.when("the supplied max number of rows is LESS than those in the FileThatHasBeenRead", |ctx| {
            ctx.then("correctly groups rows into Two or more FileChunks", |env| {
                let resp = setup_service_and_send_request(&env.is_max_rows_less_than_file_rows_handled_correctly.clone());
                assert_eq!(resp.len(), env.is_max_rows_less_than_file_rows_handled_correctly.expected_final_result.clone().len());
                assert_eq!(resp, env.is_max_rows_less_than_file_rows_handled_correctly.expected_final_result.clone())
            });
        });
    }));
}

fn generate_ok_test_specification() -> TestSpecifications {
    TestSpecifications {
        request: (get_dummy_request(), 200),
        expected_final_result: vec![UploadFileChunkRequest {
            upload_request_id: "RECON-TASK-1234".to_string(),
            chunk_sequence_number: 1,
            chunk_source: FileUploadChunkSource::PrimaryFileChunk,
            chunk_rows: vec![
                FileRow {
                    raw_data: "001,2000".to_string(),
                    row_number: 1,
                },
                FileRow {
                    raw_data: "001,4000".to_string(),
                    row_number: 2,
                },
            ],
            is_last_chunk: true,
        }],
    }
}

fn generate_is_max_rows_less_than_file_rows_handled_correctly_test_specification() -> TestSpecifications {
    TestSpecifications {
        request: (get_dummy_request(), 1),
        expected_final_result: vec![
            UploadFileChunkRequest {
                upload_request_id: "RECON-TASK-1234".to_string(),
                chunk_sequence_number: 1,
                chunk_source: FileUploadChunkSource::PrimaryFileChunk,
                chunk_rows: vec![
                    FileRow {
                        raw_data: "001,2000".to_string(),
                        row_number: 1,
                    },
                ],
                is_last_chunk: false,
            },
            UploadFileChunkRequest {
                upload_request_id: "RECON-TASK-1234".to_string(),
                chunk_sequence_number: 2,
                chunk_source: FileUploadChunkSource::PrimaryFileChunk,
                chunk_rows: vec![
                    FileRow {
                        raw_data: "001,4000".to_string(),
                        row_number: 2,
                    },
                ],
                is_last_chunk: true,
            },
        ],
    }
}


fn setup_service_and_send_request(test_specifications: &TestSpecifications) -> Vec<UploadFileChunkRequest> {
    let sut = Transformer {};
    let (file_that_has_been_read, max_rows_per_group) = test_specifications.request.clone();
    let result = sut.group_rows_into_file_chunks(&file_that_has_been_read, max_rows_per_group);
    return result;
}

fn get_dummy_request() -> FileThatHasBeenRead {
    FileThatHasBeenRead {
        id: None,
        upload_request_id: Some("RECON-TASK-1234".to_string()),
        file_type: ReconFileType::PrimaryFile,
        column_headers: vec![
            String::from("record_id"),
            String::from("transaction_amount"),
        ],
        file_rows: vec![
            FileRow {
                raw_data: "001,2000".to_string(),
                row_number: 1,
            },
            FileRow {
                raw_data: "001,4000".to_string(),
                row_number: 2,
            },
        ],

        file_metadata: Some(FileMetadata {
            column_delimiters: Some(vec![',']),
            comparison_pairs: Some(vec![ComparisonPair {
                primary_file_column_index: 0,
                comparison_file_column_index: 0,
                is_row_identifier: true,
            }]),
        }),
    }
}