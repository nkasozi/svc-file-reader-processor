use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError,
    file::{File, FileThatHasBeenRead},
    file_row::FileRow,
};

pub struct ExcelFileReader {}

impl ExcelFileReader {
    pub fn read_file(file: &File) -> Result<FileThatHasBeenRead, AppError> {
        let file_that_has_been_read = FileThatHasBeenRead {
            id: file.id.clone(),
            upload_request_id: file.upload_request_id.clone(),
            file_type: file.file_type.clone(),
            column_headers: ExcelFileReader::read_column_headers(file),
            file_rows: ExcelFileReader::read_file_rows(file),
        };
        return Ok(file_that_has_been_read);
    }

    fn read_column_headers(_file: &File) -> Vec<String> {
        vec![]
    }

    fn read_file_rows(_file: &File) -> Vec<FileRow> {
        vec![]
    }
}
