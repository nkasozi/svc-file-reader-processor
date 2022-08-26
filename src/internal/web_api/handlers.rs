use actix_web::{
    post,
    web::{self, Data},
    HttpResponse,
};

use crate::internal::{
    interfaces::reconstruct_file_service::ReconstructFileServiceInterface,
    models::view_models::requests::reconstruct_file_from_chunks_request::ReconstructFileFromChunksRequest,
    shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind,
};

#[post("/reconstruct-file")]
async fn reconstruct_file(
    task_details: web::Json<ReconstructFileFromChunksRequest>,
    service: Data<Box<dyn ReconstructFileServiceInterface>>,
) -> HttpResponse {
    let recon_task_details = service.rebuild_file(task_details.0).await;

    return match recon_task_details {
        Ok(details) => HttpResponse::Ok().json(details),

        Err(err) => match err.kind {
            AppErrorKind::BadClientRequest => HttpResponse::BadRequest().json(format!("{}", err)),
            _ => HttpResponse::InternalServerError().json(format!("{}", err)),
        },
    };
}
