use crate::{
    external::{
        pubsub::dapr_pubsub::DaprPubSub,
        repositories::recon_results_repository::ReconResultsRepository,
    },
    internal::{
        interfaces::reconstruct_file_service::ReconstructFileServiceInterface,
        services::{
            core_logic::transformer::Transformer, reconstruct_file_service::ReconstructFileService,
        },
        web_api::handlers,
    },
};
use actix_web::{web::Data, App, HttpServer};

// constants
const DEFAULT_DAPR_CONNECTION_URL: &'static str = "http://localhost:5005";
const DEFAULT_DAPR_STATE_STORE_COMPONENT_NAME: &'static str = "State";
const DEFAULT_DAPR_STATE_STORE_NAME: &'static str = "ReconstructedFilesStore";
const DEFAULT_DAPR_PUBSUB_COMPONENT_NAME: &'static str = "PubSub";
const DEFAULT_APP_LISTEN_IP: &'static str = "0.0.0.0";
const DEFAULT_APP_LISTEN_PORT: u16 = 8080;

#[derive(Clone, Debug)]
struct AppSettings {
    pub app_port: String,

    pub app_ip: String,

    pub dapr_state_store_component_name: String,

    pub dapr_pubsub_component_name: String,

    pub dapr_grpc_server_address: String,

    pub dapr_state_store_name: String,
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
            .service(handlers::reconstruct_file)
    })
    .bind(app_listen_url)?
    .run()
    .await
}

fn setup_service(app_settings: AppSettings) -> Box<dyn ReconstructFileServiceInterface> {
    let service: Box<dyn ReconstructFileServiceInterface> = Box::new(ReconstructFileService {
        recon_results_repository: Box::new(ReconResultsRepository {
            dapr_grpc_server_address: app_settings.dapr_grpc_server_address.clone(),
            dapr_component_name: app_settings.dapr_state_store_component_name.clone(),
            dapr_state_store_name: app_settings.dapr_state_store_name.clone(),
        }),

        transformer: Box::new(Transformer {}),

        pubsub: Box::new(DaprPubSub {
            dapr_grpc_server_address: app_settings.dapr_grpc_server_address.clone(),
            dapr_component_name: app_settings.dapr_pubsub_component_name.clone(),
        }),
    });
    service
}

fn read_app_settings() -> AppSettings {
    AppSettings {
        app_port: std::env::var("APP_PORT").unwrap_or(DEFAULT_APP_LISTEN_PORT.to_string()),

        app_ip: std::env::var("APP_IP").unwrap_or(DEFAULT_APP_LISTEN_IP.to_string()),

        dapr_state_store_component_name: std::env::var("DAPR_STATE_STORE_COMPONENT_NAME")
            .unwrap_or(DEFAULT_DAPR_STATE_STORE_COMPONENT_NAME.to_string()),

        dapr_grpc_server_address: std::env::var("DAPR_IP")
            .unwrap_or(DEFAULT_DAPR_CONNECTION_URL.to_string()),

        dapr_state_store_name: std::env::var("DAPR_STATE_STORE_NAME")
            .unwrap_or(DEFAULT_DAPR_STATE_STORE_NAME.to_string()),

        dapr_pubsub_component_name: std::env::var("DAPR_PUBUB_COMPONENT_NAME")
            .unwrap_or(DEFAULT_DAPR_PUBSUB_COMPONENT_NAME.to_string()),
    }
}
