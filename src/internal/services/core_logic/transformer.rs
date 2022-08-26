use std::collections::HashMap;

use crate::internal::{
    interfaces::transformer::TransformerInterface,
    shared_reconciler_rust_libraries::models::entities::{
        file_upload_chunk::FileUploadChunk, reconstructed_file::ReconstructedFile,
    },
};

pub struct Transformer {}

impl TransformerInterface for Transformer {
    fn put_file_chunk_in_correct_position_in_reconstructed_file(
        &self,
        file_upload_chunk: &FileUploadChunk,
        reconstructed_file: &mut ReconstructedFile,
    ) -> ReconstructedFile {
        reconstructed_file
            .file_chunk_id_to_chunk_details_map
            .insert(
                file_upload_chunk.chunk_sequence_number,
                file_upload_chunk.clone(),
            );
        reconstructed_file.clone()
    }

    fn create_new_reconstructed_file(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> ReconstructedFile {
        let mut reconstructed_file = ReconstructedFile {
            file_id: file_upload_chunk.id.clone(),
            file_chunk_id_to_chunk_details_map: HashMap::new(),
            upload_request_id: file_upload_chunk.upload_request_id.clone(),
        };

        reconstructed_file
            .file_chunk_id_to_chunk_details_map
            .insert(
                file_upload_chunk.chunk_sequence_number,
                file_upload_chunk.clone(),
            );
        reconstructed_file
    }
}
