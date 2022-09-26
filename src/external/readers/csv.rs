use std::io::{BufRead, BufReader};

use crate::internal::shared_reconciler_rust_libraries::common::utils::app_error;
use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    app_errors::AppError,
    file::{File, FileThatHasBeenRead},
    file_row::FileRow,
};
use crate::internal::shared_reconciler_rust_libraries::models::entities::app_errors::AppErrorKind;
use crate::internal::shared_reconciler_rust_libraries::models::entities::file::FileMetadata;

pub struct CsvFileReader {}

impl CsvFileReader {
    pub fn read_file(file: &File) -> Result<FileThatHasBeenRead, AppError> {
        let updated_file = CsvFileReader::set_default_column_delimiter_if_none_found(file);
        let column_headers_found = CsvFileReader::read_column_headers(&updated_file)?;
        let file_rows_found = CsvFileReader::read_file_rows(&updated_file, if column_headers_found.is_empty() { false } else { true })?;
        let file_that_has_been_read = FileThatHasBeenRead {
            id: updated_file.id.clone(),
            upload_request_id: updated_file.upload_request_id.clone(),
            file_type: updated_file.file_type.clone(),
            column_headers: column_headers_found,
            file_rows: file_rows_found,
            file_metadata: file.file_metadata.clone(),
        };
        return Ok(file_that_has_been_read);
    }


    fn read_column_headers(_file: &File) -> Result<Vec<String>, AppError> {
        let mut headers = vec![];

        let column_delimiters = match _file.file_metadata.clone() {
            None => { return Ok(headers); }
            Some(metadata) => {
                match metadata.column_delimiters {
                    None => { return Ok(headers); }
                    Some(delimiters) => delimiters
                }
            }
        };

        let file_path = match _file.file_path.clone() {
            None => { return Ok(headers); }
            Some(path) => path
        };

        let open_result = std::fs::File::open(file_path);

        let opened_file = match open_result {
            Ok(opened_file) => opened_file,
            Err(_) => { return Ok(headers); }
        };

        let reader = BufReader::new(opened_file);

        for line in reader.lines() {
            return match line {
                Ok(line_details) => {
                    headers = CsvFileReader::split_file_row(line_details, column_delimiters);
                    Ok(headers)
                }
                Err(_) => {
                    Ok(headers)
                }
            };
        }

        return Ok(headers);
    }

    fn split_file_row(line_details: String, column_delimiters: Vec<char>) -> Vec<String> {
        let mut result = vec![];
        for column_delimiter in column_delimiters {
            let split_file_row_values: Vec<String> = line_details.split(column_delimiter).into_iter().map(|x| x.to_string()).collect();
            for split_file_row_value in split_file_row_values {
                result.push(split_file_row_value);
            }
        }
        return result;
    }

    fn read_file_rows(_file: &File, has_header_row: bool) -> Result<Vec<FileRow>, AppError> {
        let mut file_rows = vec![];
        let start_row_index = if has_header_row { 1 } else { 0 };

        let file_path = match _file.file_path.clone() {
            None => { return Ok(file_rows); }
            Some(path) => path
        };

        let open_result = std::fs::File::open(file_path);

        let opened_file = match open_result {
            Ok(opened_file) => opened_file,
            Err(_) => { return Ok(file_rows); }
        };

        let reader = BufReader::new(opened_file);

        let mut row_index = 0;
        for line in reader.lines() {
            if row_index < start_row_index {
                row_index = row_index + 1;
                continue;
            }

            match line {
                Ok(line_details) => {
                    let split_file_row = FileRow {
                        raw_data: line_details.clone(),
                        row_number: row_index.clone(),
                    };
                    file_rows.push(split_file_row);
                }
                Err(e) => {
                    return app_error(AppErrorKind::InternalError, Box::new(e));
                }
            };

            row_index = row_index + 1;
        }

        return Ok(file_rows);
    }

    fn set_default_column_delimiter_if_none_found(file: &File) -> File {
        match file.clone().file_metadata {
            None =>
                File {
                    file_metadata: Some(FileMetadata {
                        column_delimiters: Some(vec![',']),
                        comparison_pairs: None,
                    }),
                    ..file.clone()
                },

            Some(metadata) => match metadata.column_delimiters {
                None => {
                    File {
                        file_metadata: Some(FileMetadata {
                            column_delimiters: Some(vec![',']),
                            ..metadata
                        }),
                        ..file.clone()
                    }
                }

                Some(_) => {
                    return file.clone();
                }
            }
        }
    }
}

