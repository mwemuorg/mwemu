use crate::winapi::common::files::file_entry_points;

file_entry_points!(
    "kernel32",
    FindClose,
    GetCurrentDirectoryA,
    GetCurrentDirectoryW,
    GetSystemDirectoryA,
    GetSystemDirectoryW,
    GetSystemWindowsDirectoryA,
    GetSystemWindowsDirectoryW,
    GetTempPathA,
    GetTempPathW,
    GetWindowsDirectoryA,
    GetWindowsDirectoryW,
);

mod create_file_mapping;
pub use create_file_mapping::{CreateFileMappingA, CreateFileMappingW};

mod device_io_control;
pub use device_io_control::DeviceIoControl;

mod get_file_attributes;
pub use get_file_attributes::{
    GetFileAttributesA, GetFileAttributesExA, GetFileAttributesExW, GetFileAttributesW,
};
