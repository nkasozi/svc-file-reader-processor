use crate::internal::{
    interfaces::transformer::TransformerInterface,
    models::view_models::requests::upload_file_chunk_request::UploadFileChunkRequest,
    shared_reconciler_rust_libraries::models::entities::{
        file::FileThatHasBeenRead, file_upload_chunk::FileUploadChunkSource,
        recon_tasks_models::ReconFileType,
    },
};

pub struct Transformer {}

impl TransformerInterface for Transformer {
    fn group_rows_into_file_chunks(
        &self,
        file_that_has_been_read: &FileThatHasBeenRead,
        max_group_size: i64,
    ) -> Vec<UploadFileChunkRequest> {
        let mut results = vec![];
        let mut group_member_count = 1;
        let mut chunk_sequence_number = 1;

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

            if group_member_count == max_group_size {
                //add this group of rows as a new batch of rows for upload
                results.push(file_upload_request.clone());

                //reset the group count
                group_member_count = 1;

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
            }

            group_member_count = group_member_count + 1;
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
