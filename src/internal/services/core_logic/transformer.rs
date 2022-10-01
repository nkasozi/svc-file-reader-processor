use crate::internal::{
    interfaces::transformer::TransformerInterface,
    shared_reconciler_rust_libraries::models::entities::{
        file::FileThatHasBeenRead, file_upload_chunk::FileUploadChunkSource,
        recon_tasks_models::ReconFileType,
    },
};
use crate::internal::shared_reconciler_rust_libraries::sdks::internal_microservices::view_models::requests::UploadFileChunkRequest;

pub struct Transformer {}

impl TransformerInterface for Transformer {
    fn group_rows_into_file_chunks(
        &self,
        file_that_has_been_read: &FileThatHasBeenRead,
        max_group_size: i64,
    ) -> Vec<UploadFileChunkRequest> {
        let mut results = vec![];
        let mut row_count_in_group = 1;
        let mut chunk_sequence_number = 1;
        let mut iteration_count = 1;
        let total_row_count = file_that_has_been_read.file_rows.len();

        let mut file_upload_request = UploadFileChunkRequest {
            upload_request_id: file_that_has_been_read.upload_request_id.clone().unwrap_or("".to_string()),
            chunk_sequence_number: chunk_sequence_number.clone(),
            chunk_source: self.get_chunk_source(file_that_has_been_read.file_type.clone()),
            chunk_rows: vec![],
            is_last_chunk: false,
        };

        for file_row in file_that_has_been_read.file_rows.clone() {
            //add this file row to the group of rows
            file_upload_request.chunk_rows.push(file_row.clone());

            let is_last_chunk = iteration_count == total_row_count;
            let is_chunk_full = row_count_in_group == max_group_size;

            if is_last_chunk || is_chunk_full {
                file_upload_request.is_last_chunk = is_last_chunk;
                file_upload_request.chunk_sequence_number = chunk_sequence_number.clone();

                //add this group of rows as a new batch of rows for upload
                results.push(file_upload_request.clone());

                //reset the group count
                iteration_count = iteration_count + 1;
                row_count_in_group = row_count_in_group + 1;

                //create a new upload request for a new group of rows
                file_upload_request = UploadFileChunkRequest {
                    upload_request_id: file_that_has_been_read.upload_request_id.clone().unwrap_or("".to_string()),
                    chunk_sequence_number: chunk_sequence_number.clone(),
                    chunk_source: self.get_chunk_source(file_that_has_been_read.file_type.clone()),
                    chunk_rows: vec![],
                    is_last_chunk: false,
                };

                //update the chunk sequence number
                chunk_sequence_number = chunk_sequence_number + 1;
                continue;
            }

            iteration_count = iteration_count + 1;
            row_count_in_group = row_count_in_group + 1;
        }

        return results;
    }
}

impl Transformer {
    fn get_chunk_source(&self, recon_file_type: ReconFileType) -> FileUploadChunkSource {
        match recon_file_type {
            ReconFileType::PrimaryFile => FileUploadChunkSource::PrimaryFileChunk,
            ReconFileType::ComparisonFile => FileUploadChunkSource::ComparisonFileChunk,
        }
    }
}
