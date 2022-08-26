use mockall::automock;

use crate::internal::shared_reconciler_rust_libraries::models::entities::{
    file_upload_chunk::FileUploadChunk, reconstructed_file::ReconstructedFile,
};

#[automock]
pub trait TransformerInterface: Send + Sync {
    fn put_file_chunk_in_correct_position_in_reconstructed_file(
        &self,
        file_upload_chunk: &FileUploadChunk,
        reconstructed_file: &mut ReconstructedFile,
    ) -> ReconstructedFile;

    fn create_new_reconstructed_file(
        &self,
        file_upload_chunk: &FileUploadChunk,
    ) -> ReconstructedFile;
}
