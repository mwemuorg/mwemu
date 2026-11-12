use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../set_file_attributes.rs"]
mod set_file_attributes;
pub use set_file_attributes::*;

#[path = "../remove_directory.rs"]
mod remove_directory;
pub use remove_directory::*;

#[path = "../move_file.rs"]
mod move_file;
pub use move_file::*;

#[path = "../get_windows_directory.rs"]
mod get_windows_directory;
pub use get_windows_directory::*;

#[path = "../get_temp_path.rs"]
mod get_temp_path;
pub use get_temp_path::*;

#[path = "../get_system_directory.rs"]
mod get_system_directory;
pub use get_system_directory::*;

#[path = "../get_full_path_name.rs"]
mod get_full_path_name;
pub use get_full_path_name::*;

#[path = "../get_file_attributes.rs"]
mod get_file_attributes;
pub use get_file_attributes::*;

#[path = "../get_drive_type.rs"]
mod get_drive_type;
pub use get_drive_type::*;

#[path = "../get_disk_free_space_ex.rs"]
mod get_disk_free_space_ex;
pub use get_disk_free_space_ex::*;

#[path = "../get_current_directory.rs"]
mod get_current_directory;
pub use get_current_directory::*;

#[path = "../get_command_line.rs"]
mod get_command_line;
pub use get_command_line::*;

#[path = "../find_next_file.rs"]
mod find_next_file;
pub use find_next_file::*;

#[path = "../find_first_file.rs"]
mod find_first_file;
pub use find_first_file::*;

#[path = "../delete_file.rs"]
mod delete_file;
pub use delete_file::*;

#[path = "../create_named_pipe.rs"]
mod create_named_pipe;
pub use create_named_pipe::*;

#[path = "../create_file_mapping.rs"]
mod create_file_mapping;
pub use create_file_mapping::*;

#[path = "../create_directory.rs"]
mod create_directory;
pub use create_directory::*;

#[path = "../copy_file.rs"]
mod copy_file;
pub use copy_file::*;

#[path = "../volume.rs"]
mod volume;
pub use volume::*;

#[path = "../file_time.rs"]
mod file_time;
pub use file_time::*;

#[path = "../query_dos_device.rs"]
mod query_dos_device;
pub use query_dos_device::*;

#[path = "../are_file_api_is_ansi.rs"]
mod are_file_api_is_ansi;
pub use are_file_api_is_ansi::*;

#[path = "../connect_named_pipe.rs"]
mod connect_named_pipe;
pub use connect_named_pipe::*;

#[path = "../disconnect_named_pipe.rs"]
mod disconnect_named_pipe;
pub use disconnect_named_pipe::*;

#[path = "../file_time_to_system_time.rs"]
mod file_time_to_system_time;
pub use file_time_to_system_time::*;

#[path = "../find_close.rs"]
mod find_close;
pub use find_close::*;

#[path = "../find_first_file_ex.rs"]
mod find_first_file_ex;
pub use find_first_file_ex::*;

#[path = "../get_file_type.rs"]
mod get_file_type;
pub use get_file_type::*;

#[path = "../get_logical_drives.rs"]
mod get_logical_drives;
pub use get_logical_drives::*;

#[path = "../get_system_time_as_file_time.rs"]
mod get_system_time_as_file_time;
pub use get_system_time_as_file_time::*;

#[path = "../get_system_time_precise_as_file_time.rs"]
mod get_system_time_precise_as_file_time;
pub use get_system_time_precise_as_file_time::*;

#[path = "../device_io_control.rs"]
mod device_io_control;
pub use device_io_control::*;

#[path = "../system_time_to_file_time.rs"]
mod system_time_to_file_time;
pub use system_time_to_file_time::*;

#[path = "../GetFileSize.rs"]
mod GetFileSize;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "AreFileApiIsAnsi" | "AreFileApisANSI" => kernel32::AreFileApiIsAnsi(emu),
        "ConnectNamedPipe" => kernel32::ConnectNamedPipe(emu),
        "DeleteFileA" => kernel32::DeleteFileA(emu),
        "DeleteFileW" => kernel32::DeleteFileW(emu),
        "CopyFileA" => kernel32::CopyFileA(emu),
        "CopyFileW" => kernel32::CopyFileW(emu),
        "CreateDirectoryA" => kernel32::CreateDirectoryA(emu),
        "CreateDirectoryW" => kernel32::CreateDirectoryW(emu),
        "CreateFileMappingA" => kernel32::CreateFileMappingA(emu),
        "CreateFileMappingW" => kernel32::CreateFileMappingW(emu),
        "CreateNamedPipeA" => kernel32::CreateNamedPipeA(emu),
        "CreateNamedPipeW" => kernel32::CreateNamedPipeW(emu),
        "DisconnectNamedPipe" => kernel32::DisconnectNamedPipe(emu),
        "FindClose" => kernel32::FindClose(emu),
        "FindFirstFileA" => kernel32::FindFirstFileA(emu),
        "FindFirstFileExA" => kernel32::FindFirstFileExA(emu),
        "FindFirstFileExW" => kernel32::FindFirstFileExW(emu),
        "FindFirstFileW" => kernel32::FindFirstFileW(emu),
        "FindNextFileA" => kernel32::FindNextFileA(emu),
        "FindNextFileW" => kernel32::FindNextFileW(emu),
        "GetCommandLineA" => kernel32::GetCommandLineA(emu),
        "GetCommandLineW" => kernel32::GetCommandLineW(emu),
        "GetDiskFreeSpaceExA" => kernel32::GetDiskFreeSpaceExA(emu),
        "GetDiskFreeSpaceExW" => kernel32::GetDiskFreeSpaceExW(emu),
        "GetDriveTypeA" => kernel32::GetDriveTypeA(emu),
        "GetDriveTypeW" => kernel32::GetDriveTypeW(emu),
        "GetCurrentDirectoryA" => kernel32::GetCurrentDirectoryA(emu),
        "GetCurrentDirectoryW" => kernel32::GetCurrentDirectoryW(emu),
        "GetFileAttributesA" => kernel32::GetFileAttributesA(emu),
        "GetFileAttributesW" => kernel32::GetFileAttributesW(emu),
        "GetFileType" => kernel32::GetFileType(emu),
        "GetFullPathNameA" => kernel32::GetFullPathNameA(emu),
        "GetFullPathNameW" => kernel32::GetFullPathNameW(emu),
        "GetLogicalDrives" => kernel32::GetLogicalDrives(emu),
        "GetSystemDirectoryA" => kernel32::GetSystemDirectoryA(emu),
        "GetSystemDirectoryW" => kernel32::GetSystemDirectoryW(emu),
        "GetTempPathA" => kernel32::GetTempPathA(emu),
        "GetTempPathW" => kernel32::GetTempPathW(emu),
        "GetWindowsDirectoryA" => kernel32::GetWindowsDirectoryA(emu),
        "GetWindowsDirectoryW" => kernel32::GetWindowsDirectoryW(emu),
        "QueryDosDeviceA" => kernel32::QueryDosDeviceA(emu),
        "QueryDosDeviceW" => kernel32::QueryDosDeviceW(emu),
        "MoveFileA" => kernel32::MoveFileA(emu),
        "MoveFileW" => kernel32::MoveFileW(emu),
        "GetVolumeInformationA" => kernel32::GetVolumeInformationA(emu),
        "GetVolumeInformationW" => kernel32::GetVolumeInformationW(emu),
        "GetVolumePathNameA" => kernel32::GetVolumePathNameA(emu),
        "GetVolumePathNameW" => kernel32::GetVolumePathNameW(emu),
        "RemoveDirectoryA" => kernel32::RemoveDirectoryA(emu),
        "RemoveDirectoryW" => kernel32::RemoveDirectoryW(emu),
        "SetFileAttributesA" => kernel32::SetFileAttributesA(emu),
        "SetFileAttributesW" => kernel32::SetFileAttributesW(emu),
        "DeviceIoControl" => kernel32::api_DeviceIoControl(emu),
        "GetLongPathNameA" => kernel32::GetLongPathNameA(emu),
        "GetLongPathNameW" => kernel32::GetLongPathNameW(emu),
        "GetShortPathNameA" => kernel32::GetShortPathNameA(emu),
        "GetShortPathNameW" => kernel32::GetShortPathNameW(emu),
        "SearchPathA" => kernel32::SearchPathA(emu),
        "SearchPathW" => kernel32::SearchPathW(emu),
        "CopyFileExA" => kernel32::CopyFileExA(emu),
        "CopyFileExW" => kernel32::CopyFileExW(emu),
        "MoveFileExA" => kernel32::MoveFileExA(emu),
        "MoveFileExW" => kernel32::MoveFileExW(emu),
        "LockFile" => kernel32::LockFile(emu),
        "UnlockFile" => kernel32::UnlockFile(emu),
        "CreatePipe" => kernel32::CreatePipe(emu),
        "GetSystemWindowsDirectoryA" => kernel32::GetSystemWindowsDirectoryA(emu),
        "GetSystemWindowsDirectoryW" => kernel32::GetSystemWindowsDirectoryW(emu),
        _ => return false,
    }
    true
}
