use actix_web::{App, HttpServer, web::Data};

use crate::{
    external::services::{
        svc_file_chunks_upload_handler::DaprSvcFileChunksUploadHandler,
        svc_recon_tasks_handler::DaprSvcReconTasksHandler,
    },
    internal::{
        interfaces::split_file_service::SplitFileServiceInterface,
        services::{
            core_logic::transformer::Transformer,
            split_file_service::SplitFileService,
        },
        web_api::handlers,
    },
};
use crate::external::readers::factory::FileReaderFactory;

// constants
const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:3500";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8082;
const DEFAULT_FILE_CHUNKS_UPLOAD_SERVICE_NAME: &'static str = "svc-file-chunks-upload-manager";
const DEFAULT_RECON_TASKS_SERVICE_NAME: &'static str = "svc-task-details-repository-manager";

#[derive(Clone, Debug)]
struct AppSettings {
    pub app_port: String,

    pub app_ip: String,

    pub dapr_grpc_server_address: String,

    pub file_chunks_uploader_service_name: String,

    pub recon_tasks_service_name: String,
}

pub async fn run_async() -> Result<(), std::io::Error> {
    //retrieve app settings from the env variables
    let app_settings = read_app_settings();

    let app_listen_url = format!("{}:{}", app_settings.app_ip, app_settings.app_port);

    //just for logging purposes
    println!("App is listening on: {:?}", app_listen_url);

    HttpServer::new(move || {
        // Create some global state prior to running the handler threads
        let service = setup_service(app_settings.clone());

        // add shared state and routing
        App::new()
            .app_data(Data::new(service))
            .service(handlers::read_file)
    })
        .bind(app_listen_url)?
        .run()
        .await
}

fn setup_service(app_settings: AppSettings) -> Box<dyn SplitFileServiceInterface> {
    let service: Box<dyn SplitFileServiceInterface> = Box::new(SplitFileService {
        transformer: Box::new(Transformer {}),
        file_reader: Box::new(FileReaderFactory {}),
        file_chunks_uploader: Box::new(DaprSvcFileChunksUploadHandler {
            dapr_grpc_server_address: app_settings.dapr_grpc_server_address.clone(),
            file_chunks_service_app_id: app_settings
                .file_chunks_uploader_service_name
                .clone(),
        }),
        recon_tasks_handler: Box::new(DaprSvcReconTasksHandler {
            dapr_grpc_server_address: app_settings.dapr_grpc_server_address.clone(),
            recon_tasks_service_app_id: app_settings.recon_tasks_service_name.clone(),
        }),
    });
    service
}

fn read_app_settings() -> AppSettings {
    AppSettings {
        app_port: DEFAULT_APP_LISTEN_PORT.to_string(),

        app_ip: std::env::var("APP_IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string()),

        dapr_grpc_server_address: std::env::var("DAPR_IP")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),

        file_chunks_uploader_service_name: std::env::var("FILE_CHUNKS_UPLOAD_SERVICE_NAME")
            .unwrap_or(DEFAULT_FILE_CHUNKS_UPLOAD_SERVICE_NAME.to_string()),

        recon_tasks_service_name: std::env::var("RECON_TASKS_SERVICE_NAME")
            .unwrap_or(DEFAULT_RECON_TASKS_SERVICE_NAME.to_string()),
    }
}
