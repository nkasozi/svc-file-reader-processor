use crate::internal::{
    interfaces::file_reader::FileReader,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::AppError,
        file::{File, FileThatHasBeenRead, SupportedFileExtension},
    },
};
use async_trait::async_trait;

use super::{csv::CsvFileReader, excel::ExcelFileReader, pdf::PdfFileReader};

pub struct FileReaderFactory {}

#[async_trait]
impl FileReader for FileReaderFactory {
    async fn read_file(&self, file: &File) -> Result<FileThatHasBeenRead, AppError> {
        match file.file_extension {
            SupportedFileExtension::Csv => CsvFileReader::read_file(file),
            SupportedFileExtension::Excel => ExcelFileReader::read_file(file),
            SupportedFileExtension::Pdf => PdfFileReader::read_file(file),
        }
    }
}
