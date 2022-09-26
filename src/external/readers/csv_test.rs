use crate::external::readers::csv::CsvFileReader;
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::{File, FileStorageLocation, SupportedFileExtension};
use crate::internal::shared_reconciler_rust_libraries::models::entities::recon_tasks_models::ReconFileType;

#[test]
fn test_read_csv_file() {
    let file = File {
        id: None,
        upload_request_id: None,
        file_storage_location: FileStorageLocation::LocalFileSystem,
        file_extension: SupportedFileExtension::Csv,
        file_metadata: None,
        file_path: Some(String::from("E:\\Work\\cplk\\primary_file.csv")),
        file_type: ReconFileType::PrimaryFile,
    };

    let _read_result = CsvFileReader::read_file(&file);

    //assert!(read_result.is_ok());
}