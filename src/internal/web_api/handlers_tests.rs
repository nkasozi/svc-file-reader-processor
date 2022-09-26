use std::sync::{Arc, Mutex};

use actix_web::{
    App,
    test::{self, TestRequest},
    web::Data,
};
use actix_web::body::BoxBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;

use crate::internal::interfaces::split_file_service::MockSplitFileServiceInterface;
use crate::internal::interfaces::split_file_service::SplitFileServiceInterface;
use crate::internal::models::view_models::requests::split_file_request::SplitFileRequest;
use crate::internal::models::view_models::responses::split_file_response::SplitFileResponse;
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppError;
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind;
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::{File, FileStorageLocation, SupportedFileExtension};
use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::ReconFileType;
use crate::internal::web_api::handlers::read_file;

//good request, bad client request, internal server error
#[derive(Clone, Debug)]
struct TestSpecifications {
    request: SplitFileRequest,
    mock_service_response: Result<SplitFileResponse, AppError>,
    expected_status_code: StatusCode,
}

#[derive(Clone, Debug)]
struct ValidRequestTestScenarios {
    ok_test: TestSpecifications,
    internal_server_error_test: TestSpecifications,
}

#[derive(Clone, Debug)]
struct InvalidRequestTestScenarios {
    invalid_client_request: TestSpecifications,
}

#[test]
fn test_read_and_split_file_handler_valid_request() {
    let valid_request_test_suite = ValidRequestTestScenarios {
        ok_test: TestSpecifications {
            request: get_dummy_request(),
            mock_service_response: Ok(SplitFileResponse {
                upload_request_id: "FILE-1234".to_string(),
            }),
            expected_status_code: StatusCode::OK,
        },
        internal_server_error_test: TestSpecifications {
            request: get_dummy_request(),
            mock_service_response: get_dummy_error(AppErrorKind::InternalError),
            expected_status_code: StatusCode::INTERNAL_SERVER_ERROR,
        },
    };

    rspec::run(&rspec::given("valid client request", valid_request_test_suite, |ctx| {
        ctx.when("service returns OK", |ctx| {
            ctx.then("returns 200", |env| {
                let resp = setup_server_and_send_request(&env.ok_test);
                assert_eq!(resp.status(), env.ok_test.expected_status_code);
            });
        });

        ctx.when("service returns InternalError", |ctx| {
            ctx.then("returns 500", |env| {
                let resp = setup_server_and_send_request(&env.internal_server_error_test);
                assert_eq!(resp.status(), env.internal_server_error_test.expected_status_code);
            });
        });
    }));
}


#[test]
fn test_read_and_split_file_handler_invalid_request() {
    let invalid_request_test_suite = InvalidRequestTestScenarios {
        invalid_client_request: TestSpecifications {
            request: get_dummy_request(),
            mock_service_response: get_dummy_error(AppErrorKind::BadClientRequest),
            expected_status_code: StatusCode::BAD_REQUEST,
        },
    };

    rspec::run(&rspec::given("invalid client request", invalid_request_test_suite, |ctx| {
        ctx.when("service returns BadClientRequest", |ctx| {
            ctx.then("returns 400", |env| {
                let resp = setup_server_and_send_request(&env.invalid_client_request);
                assert_eq!(resp.status(), env.invalid_client_request.expected_status_code);
            });
        });
    }));
}

fn get_dummy_error(app_error_kind: AppErrorKind) -> Result<SplitFileResponse, AppError> {
    Err(AppError::new(
        app_error_kind, "error occurred".to_string(),
    ))
}


fn setup_server_and_send_request(test_specifications: &TestSpecifications) -> ServiceResponse<BoxBody> {
    let mut app = tokio_test::block_on(test::init_service((move || {
        let service_response: Arc<Mutex<Result<SplitFileResponse, AppError>>> = Arc::new(Mutex::from(test_specifications.mock_service_response.clone()));

        let mock_service = get_mock_service_response(service_response);

        App::new()
            .app_data(Data::new(mock_service)) // add shared state
            .service(read_file)
    })()));

    let resp = tokio_test::block_on(TestRequest::post()
        .uri(&format!("/read-file"))
        .set_json(test_specifications.request.clone())
        .send_request(&mut app));

    return resp;
}

fn get_mock_service_response(service_response: Arc<Mutex<Result<SplitFileResponse, AppError>>>) -> Box<dyn SplitFileServiceInterface> {
    let mut mock_service = Box::new(MockSplitFileServiceInterface::new());
    mock_service.expect_read_and_split_file_into_chunks().returning(move |_y| {
        let resp = service_response.lock().unwrap().clone();
        if resp.is_ok() {
            return Ok(resp.ok().unwrap().clone());
        }
        return Err(resp.err().clone().unwrap());
    });
    return mock_service;
}

fn get_dummy_request() -> SplitFileRequest {
    SplitFileRequest {
        file: File {
            id: None,
            upload_request_id: None,
            file_storage_location: FileStorageLocation::LocalFileSystem,
            file_extension: SupportedFileExtension::Csv,
            file_metadata: None,
            file_path: Some("E:/Work/test.csv".to_string()),
            file_type: ReconFileType::ComparisonFile,
        },
    }
}




