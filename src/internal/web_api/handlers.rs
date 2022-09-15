use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use crate::internal::{
    interfaces::split_file_service::SplitFileServiceInterface,
    models::view_models::requests::split_file_request::SplitFileRequest,
    shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind,
};

#[post("/read-file")]
async fn read_file(
    task_details: web::Json<SplitFileRequest>,
    service: Data<Box<dyn SplitFileServiceInterface>>,
) -> HttpResponse {
    let recon_task_details = service
        .read_and_split_file_into_chunks(task_details.0)
        .await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),

        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}
