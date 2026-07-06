//! PE resource-directory structures (pure PE concepts).

mod image_resource_data_entry32;
mod image_resource_data_entry64;
mod image_resource_directory;
mod image_resource_directory_entry;

pub use image_resource_data_entry32::ImageResourceDataEntry32;
pub use image_resource_data_entry64::ImageResourceDataEntry64;
pub use image_resource_directory::ImageResourceDirectory;
pub use image_resource_directory_entry::ImageResourceDirectoryEntry;
