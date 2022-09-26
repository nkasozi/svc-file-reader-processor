use actix_web::{
    HttpResponse,
    post,
    web::{self, Data},
};

use crate::internal::{
    interfaces::split_file_service::SplitFileServiceInterface,
    models::view_models::requests::split_file_request::SplitFileRequest,
    shared_reconciler_rust_libraries::web_api::utils::ok_or_error,
};

#[post("/read-file")]
pub async fn read_file(
    task_details: web::Json<SplitFileRequest>,
    service: Data<Box<dyn SplitFileServiceInterface>>,
) -> HttpResponse {
    let response = service
        .read_and_split_file_into_chunks(task_details.0)
        .await;

    return ok_or_error(response);
}
