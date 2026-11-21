use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod set_file_attributes;
pub use set_file_attributes::{SetFileAttributesA, SetFileAttributesW};

mod remove_directory;
pub use remove_directory::{RemoveDirectoryA, RemoveDirectoryW};

mod move_file;
pub use move_file::{MoveFileA, MoveFileW};

mod get_windows_directory;
pub use get_windows_directory::{GetWindowsDirectoryA, GetWindowsDirectoryW};

mod get_temp_path;
pub use get_temp_path::{GetTempPathA, GetTempPathW};

mod get_system_directory;
pub use get_system_directory::{GetSystemDirectoryA, GetSystemDirectoryW};

mod get_full_path_name;
pub use get_full_path_name::{GetFullPathNameA, GetFullPathNameW};

mod get_file_attributes;
pub use get_file_attributes::{GetFileAttributesA, GetFileAttributesW};

mod get_drive_type;
pub use get_drive_type::{GetDriveTypeA, GetDriveTypeW};

mod get_disk_free_space_ex;
pub use get_disk_free_space_ex::{GetDiskFreeSpaceExA, GetDiskFreeSpaceExW};

mod get_current_directory;
pub use get_current_directory::{GetCurrentDirectoryA, GetCurrentDirectoryW};

mod get_command_line;
pub use get_command_line::{GetCommandLineA, GetCommandLineW};

mod find_next_file;
pub use find_next_file::{FindNextFileA, FindNextFileW};

mod find_first_file;
pub use find_first_file::{FindFirstFileA, FindFirstFileW};

mod delete_file;
pub use delete_file::{DeleteFileA, DeleteFileW};

mod create_named_pipe;
pub use create_named_pipe::{CreateNamedPipeA, CreateNamedPipeW};

mod create_file_mapping;
pub use create_file_mapping::{CreateFileMappingA, CreateFileMappingW};

mod create_directory;
pub use create_directory::{CreateDirectoryA, CreateDirectoryW};

mod copy_file;
pub use copy_file::{CopyFileA, CopyFileW};

mod volume;
pub use volume::{
    GetVolumeInformationA, GetVolumeInformationW, GetVolumePathNameA, GetVolumePathNameW,
};

mod file_time;
pub use file_time::{GetFileTime, SetFileTime};

mod query_dos_device;
pub use query_dos_device::{QueryDosDeviceA, QueryDosDeviceW};

mod are_file_api_is_ansi;
pub use are_file_api_is_ansi::AreFileApiIsAnsi;

mod connect_named_pipe;
pub use connect_named_pipe::ConnectNamedPipe;

mod disconnect_named_pipe;
pub use disconnect_named_pipe::DisconnectNamedPipe;

mod file_time_to_system_time;
pub use file_time_to_system_time::FileTimeToSystemTime;

mod find_close;
pub use find_close::FindClose;

mod find_first_file_ex;
pub use find_first_file_ex::{FindFirstFileExA, FindFirstFileExW};

mod get_file_type;
pub use get_file_type::GetFileType;

mod get_logical_drives;
pub use get_logical_drives::GetLogicalDrives;

mod get_system_time_as_file_time;
pub use get_system_time_as_file_time::GetSystemTimeAsFileTime;

mod get_system_time_precise_as_file_time;
pub use get_system_time_precise_as_file_time::GetSystemTimePreciseAsFileTime;

mod device_io_control;
pub use device_io_control::DeviceIoControl;

mod system_time_to_file_time;
pub use system_time_to_file_time::SystemTimeToFileTime;

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
        "DeviceIoControl" => kernel32::DeviceIoControl(emu),
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
