use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

#[path = "../set_file_attributes_transacted.rs"]
mod set_file_attributes_transacted;
pub use set_file_attributes_transacted::*;

#[path = "../set_default_comm_config.rs"]
mod set_default_comm_config;
pub use set_default_comm_config::*;

#[path = "../remove_local_alternate_computer_name.rs"]
mod remove_local_alternate_computer_name;
pub use remove_local_alternate_computer_name::*;

#[path = "../remove_directory_transacted.rs"]
mod remove_directory_transacted;
pub use remove_directory_transacted::*;

#[path = "../query_dos_device.rs"]
mod query_dos_device;
pub use query_dos_device::*;

#[path = "../open_file_mapping.rs"]
mod open_file_mapping;
pub use open_file_mapping::*;

#[path = "../move_file_ex.rs"]
mod move_file_ex;
pub use move_file_ex::*;

#[path = "../get_volume_path_names_for_volume_name.rs"]
mod get_volume_path_names_for_volume_name;
pub use get_volume_path_names_for_volume_name::*;

#[path = "../get_volume_name_for_volume_mount_point.rs"]
mod get_volume_name_for_volume_mount_point;
pub use get_volume_name_for_volume_mount_point::*;

#[path = "../get_named_pipe_handle_state.rs"]
mod get_named_pipe_handle_state;
pub use get_named_pipe_handle_state::*;

#[path = "../get_named_pipe_client_computer_name.rs"]
mod get_named_pipe_client_computer_name;
pub use get_named_pipe_client_computer_name::*;

#[path = "../get_file_attributes_transacted.rs"]
mod get_file_attributes_transacted;
pub use get_file_attributes_transacted::*;

#[path = "../get_compressed_file_size_transacted.rs"]
mod get_compressed_file_size_transacted;
pub use get_compressed_file_size_transacted::*;

#[path = "../find_next_volume.rs"]
mod find_next_volume;
pub use find_next_volume::*;

#[path = "../find_first_volume.rs"]
mod find_first_volume;
pub use find_first_volume::*;

#[path = "../delete_volume_mount_point.rs"]
mod delete_volume_mount_point;
pub use delete_volume_mount_point::*;

#[path = "../delete_file_transacted.rs"]
mod delete_file_transacted;
pub use delete_file_transacted::*;

#[path = "../define_dos_device.rs"]
mod define_dos_device;
pub use define_dos_device::*;

#[path = "../create_named_pipe.rs"]
mod create_named_pipe;
pub use create_named_pipe::*;

#[path = "../create_file_mapping_numa.rs"]
mod create_file_mapping_numa;
pub use create_file_mapping_numa::*;

#[path = "../create_directory_ex.rs"]
mod create_directory_ex;
pub use create_directory_ex::*;

#[path = "../copy_file_ex.rs"]
mod copy_file_ex;
pub use copy_file_ex::*;

#[path = "../comm_config_dialog.rs"]
mod comm_config_dialog;
pub use comm_config_dialog::*;

#[path = "../call_named_pipe.rs"]
mod call_named_pipe;
pub use call_named_pipe::*;

#[path = "../set_volume_mount_point.rs"]
mod set_volume_mount_point;
pub use set_volume_mount_point::*;

#[path = "../set_volume_label.rs"]
mod set_volume_label;
pub use set_volume_label::*;

#[path = "../set_file_short_name.rs"]
mod set_file_short_name;
pub use set_file_short_name::*;

#[path = "../set_file_attributes.rs"]
mod set_file_attributes;
pub use set_file_attributes::*;

#[path = "../set_dll_directory.rs"]
mod set_dll_directory;
pub use set_dll_directory::*;

#[path = "../remove_directory.rs"]
mod remove_directory;
pub use remove_directory::*;

#[path = "../move_file_transacted.rs"]
mod move_file_transacted;
pub use move_file_transacted::*;

#[path = "../move_file.rs"]
mod move_file;
pub use move_file::*;

#[path = "../lz_open_file.rs"]
mod lz_open_file;
pub use lz_open_file::*;

#[path = "../get_windows_directory.rs"]
mod get_windows_directory;
pub use get_windows_directory::*;

#[path = "../get_volume_information.rs"]
mod get_volume_information;
pub use get_volume_information::*;

#[path = "../get_temp_path.rs"]
mod get_temp_path;
pub use get_temp_path::*;

#[path = "../get_system_windows_directory.rs"]
mod get_system_windows_directory;
pub use get_system_windows_directory::*;

#[path = "../get_system_directory.rs"]
mod get_system_directory;
pub use get_system_directory::*;

#[path = "../get_short_path_name.rs"]
mod get_short_path_name;
pub use get_short_path_name::*;

#[path = "../get_long_path_name.rs"]
mod get_long_path_name;
pub use get_long_path_name::*;

#[path = "../get_full_path_name_transacted.rs"]
mod get_full_path_name_transacted;
pub use get_full_path_name_transacted::*;

#[path = "../get_full_path_name.rs"]
mod get_full_path_name;
pub use get_full_path_name::*;

#[path = "../get_file_attributes.rs"]
mod get_file_attributes;
pub use get_file_attributes::*;

#[path = "../get_drive_type.rs"]
mod get_drive_type;
pub use get_drive_type::*;

#[path = "../get_dll_directory.rs"]
mod get_dll_directory;
pub use get_dll_directory::*;

#[path = "../get_disk_free_space_ex.rs"]
mod get_disk_free_space_ex;
pub use get_disk_free_space_ex::*;

#[path = "../get_default_comm_config.rs"]
mod get_default_comm_config;
pub use get_default_comm_config::*;

#[path = "../get_current_directory.rs"]
mod get_current_directory;
pub use get_current_directory::*;

#[path = "../get_command_line.rs"]
mod get_command_line;
pub use get_command_line::*;

#[path = "../find_next_volume_mount_point.rs"]
mod find_next_volume_mount_point;
pub use find_next_volume_mount_point::*;

#[path = "../find_next_file.rs"]
mod find_next_file;
pub use find_next_file::*;

#[path = "../find_first_volume_mount_point.rs"]
mod find_first_volume_mount_point;
pub use find_first_volume_mount_point::*;

#[path = "../find_first_file.rs"]
mod find_first_file;
pub use find_first_file::*;

#[path = "../dos_path_to_session_path.rs"]
mod dos_path_to_session_path;
pub use dos_path_to_session_path::*;

#[path = "../delete_file.rs"]
mod delete_file;
pub use delete_file::*;

#[path = "../create_file_mapping.rs"]
mod create_file_mapping;
pub use create_file_mapping::*;

#[path = "../create_directory_transacted.rs"]
mod create_directory_transacted;
pub use create_directory_transacted::*;

#[path = "../create_directory.rs"]
mod create_directory;
pub use create_directory::*;

#[path = "../copy_file_transacted.rs"]
mod copy_file_transacted;
pub use copy_file_transacted::*;

#[path = "../copy_file.rs"]
mod copy_file;
pub use copy_file::*;

#[path = "../build_comm_dcb_and_timeouts.rs"]
mod build_comm_dcb_and_timeouts;
pub use build_comm_dcb_and_timeouts::*;

#[path = "../are_file_apis_ansi.rs"]
mod are_file_apis_ansi;
pub use are_file_apis_ansi::*;

#[path = "../connect_named_pipe.rs"]
mod connect_named_pipe;
pub use connect_named_pipe::*;

#[path = "../disconnect_named_pipe.rs"]
mod disconnect_named_pipe;
pub use disconnect_named_pipe::*;

#[path = "../file_time_to_dos_date_time.rs"]
mod file_time_to_dos_date_time;
pub use file_time_to_dos_date_time::*;

#[path = "../file_time_to_local_file_time.rs"]
mod file_time_to_local_file_time;
pub use file_time_to_local_file_time::*;

#[path = "../file_time_to_system_time.rs"]
mod file_time_to_system_time;
pub use file_time_to_system_time::*;

#[path = "../find_close.rs"]
mod find_close;
pub use find_close::*;

#[path = "../get_file_type.rs"]
mod get_file_type;
pub use get_file_type::*;

#[path = "../get_logical_drives.rs"]
mod get_logical_drives;
pub use get_logical_drives::*;

#[path = "../get_system_time_as_file_time.rs"]
mod get_system_time_as_file_time;
pub use get_system_time_as_file_time::*;

#[path = "../backup_read.rs"]
mod backup_read;
pub use backup_read::*;

#[path = "../backup_seek.rs"]
mod backup_seek;
pub use backup_seek::*;

#[path = "../backup_write.rs"]
mod backup_write;
pub use backup_write::*;

#[path = "../build_comm_dcba.rs"]
mod build_comm_dcba;
pub use build_comm_dcba::*;

#[path = "../build_comm_dcbw.rs"]
mod build_comm_dcbw;
pub use build_comm_dcbw::*;

#[path = "../copy_lz_file.rs"]
mod copy_lz_file;
pub use copy_lz_file::*;

#[path = "../create_tape_partition.rs"]
mod create_tape_partition;
pub use create_tape_partition::*;

#[path = "../delete_atom.rs"]
mod delete_atom;
pub use delete_atom::*;

#[path = "../delete_timer_queue.rs"]
mod delete_timer_queue;
pub use delete_timer_queue::*;

#[path = "../device_io_control_implementation.rs"]
mod device_io_control_implementation;
pub use device_io_control_implementation::*;

#[path = "../dos_date_time_to_file_time.rs"]
mod dos_date_time_to_file_time;
pub use dos_date_time_to_file_time::*;

#[path = "../duplicate_encryption_info_file_ext.rs"]
mod duplicate_encryption_info_file_ext;
pub use duplicate_encryption_info_file_ext::*;

#[path = "../erase_tape.rs"]
mod erase_tape;
pub use erase_tape::*;

#[path = "../find_volume_mount_point_close.rs"]
mod find_volume_mount_point_close;
pub use find_volume_mount_point_close::*;

#[path = "../get_encrypted_file_version_ext.rs"]
mod get_encrypted_file_version_ext;
pub use get_encrypted_file_version_ext::*;

#[path = "../get_file_bandwidth_reservation.rs"]
mod get_file_bandwidth_reservation;
pub use get_file_bandwidth_reservation::*;

#[path = "../get_friendly_match_comm.rs"]
mod get_friendly_match_comm;
pub use get_friendly_match_comm::*;

#[path = "../get_mailslot_info.rs"]
mod get_mailslot_info;
pub use get_mailslot_info::*;

#[path = "../get_named_pipe_client_session_id.rs"]
mod get_named_pipe_client_session_id;
pub use get_named_pipe_client_session_id::*;

#[path = "../get_named_pipe_server_session_id.rs"]
mod get_named_pipe_server_session_id;
pub use get_named_pipe_server_session_id::*;

#[path = "../get_tape_parameters.rs"]
mod get_tape_parameters;
pub use get_tape_parameters::*;

#[path = "../get_tape_position.rs"]
mod get_tape_position;
pub use get_tape_position::*;

#[path = "../get_tape_status.rs"]
mod get_tape_status;
pub use get_tape_status::*;

#[path = "../global_delete_atom.rs"]
mod global_delete_atom;
pub use global_delete_atom::*;

#[path = "../open_file.rs"]
mod open_file;
pub use open_file::*;

#[path = "../prepare_tape.rs"]
mod prepare_tape;
pub use prepare_tape::*;

#[path = "../priv_move_file_identity_w.rs"]
mod priv_move_file_identity_w;
pub use priv_move_file_identity_w::*;

#[path = "../remove_secure_memory_cache_callback.rs"]
mod remove_secure_memory_cache_callback;
pub use remove_secure_memory_cache_callback::*;

#[path = "../replace_file_a.rs"]
mod replace_file_a;
pub use replace_file_a::*;

#[path = "../replace_partition_unit.rs"]
mod replace_partition_unit;
pub use replace_partition_unit::*;

#[path = "../set_file_bandwidth_reservation.rs"]
mod set_file_bandwidth_reservation;
pub use set_file_bandwidth_reservation::*;

#[path = "../set_mailslot_info.rs"]
mod set_mailslot_info;
pub use set_mailslot_info::*;

#[path = "../set_named_pipe_attribute.rs"]
mod set_named_pipe_attribute;
pub use set_named_pipe_attribute::*;

#[path = "../set_tape_parameters.rs"]
mod set_tape_parameters;
pub use set_tape_parameters::*;

#[path = "../set_tape_position.rs"]
mod set_tape_position;
pub use set_tape_position::*;

#[path = "../write_tapemark.rs"]
mod write_tapemark;
pub use write_tapemark::*;

#[path = "../are_file_api_is_ansi.rs"]
mod are_file_api_is_ansi;

#[path = "../device_io_control.rs"]
mod device_io_control;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "CreateDirectoryA" => kernel32::CreateDirectoryA(emu),
        "CreateDirectoryW" => kernel32::CreateDirectoryW(emu),
        "DeleteFileW" => kernel32::DeleteFileW(emu),
        "AreFileApiIsAnsi" | "AreFileApisANSI" => kernel32::AreFileApisANSI(emu),
        "ConnectNamedPipe" => kernel32::ConnectNamedPipe(emu),
        "DeleteFileA" => kernel32::DeleteFileA(emu),
        "CopyFileA" => kernel32::CopyFileA(emu),
        "CopyFileW" => kernel32::CopyFileW(emu),
        "CreateFileMappingA" => kernel32::CreateFileMappingA(emu),
        "CreateFileMappingW" => kernel32::CreateFileMappingW(emu),
        "CreateNamedPipeA" => kernel32::CreateNamedPipeA(emu),
        "DisconnectNamedPipe" => kernel32::DisconnectNamedPipe(emu),
        "FindClose" => kernel32::FindClose(emu),
        "FindFirstFileA" => kernel32::FindFirstFileA(emu),
        "FindFirstFileW" => kernel32::FindFirstFileW(emu),
        "FindNextFileA" => kernel32::FindNextFileA(emu),
        "FindNextFileW" => kernel32::FindNextFileW(emu),
        "GetCommandLineA" => kernel32::GetCommandLineA(emu),
        "GetCommandLineW" => kernel32::GetCommandLineW(emu),
        "GetCurrentDirectoryA" => kernel32::GetCurrentDirectoryA(emu),
        "GetCurrentDirectoryW" => kernel32::GetCurrentDirectoryW(emu),
        "GetDiskFreeSpaceExA" => kernel32::GetDiskFreeSpaceExA(emu),
        "GetDiskFreeSpaceExW" => kernel32::GetDiskFreeSpaceExW(emu),
        "GetDriveTypeA" => kernel32::GetDriveTypeA(emu),
        "GetDriveTypeW" => kernel32::GetDriveTypeW(emu),
        "GetFileAttributesA" => kernel32::GetFileAttributesA(emu),
        "GetFileAttributesW" => kernel32::GetFileAttributesW(emu),
        "GetFileType" => kernel32::GetFileType(emu),
        "GetFullPathNameA" => kernel32::GetFullPathNameA(emu),
        "GetFullPathNameW" => kernel32::GetFullPathNameW(emu),
        "GetLogicalDrives" => kernel32::GetLogicalDrives(emu),
        "GetLongPathNameW" => kernel32::GetLongPathNameW(emu),
        "GetSystemDirectoryA" => kernel32::GetSystemDirectoryA(emu),
        "GetSystemDirectoryW" => kernel32::GetSystemDirectoryW(emu),
        "GetSystemWindowsDirectoryA" => kernel32::GetSystemWindowsDirectoryA(emu),
        "GetSystemWindowsDirectoryW" => kernel32::GetSystemWindowsDirectoryW(emu),
        "GetTempPathA" => kernel32::GetTempPathA(emu),
        "GetTempPathW" => kernel32::GetTempPathW(emu),
        "GetVolumeInformationA" => kernel32::GetVolumeInformationA(emu),
        "GetVolumeInformationW" => kernel32::GetVolumeInformationW(emu),
        "GetWindowsDirectoryA" => kernel32::GetWindowsDirectoryA(emu),
        "GetWindowsDirectoryW" => kernel32::GetWindowsDirectoryW(emu),
        "MoveFileA" => kernel32::MoveFileA(emu),
        "MoveFileW" => kernel32::MoveFileW(emu),
        "RemoveDirectoryA" => kernel32::RemoveDirectoryA(emu),
        "RemoveDirectoryW" => kernel32::RemoveDirectoryW(emu),
        "SetFileAttributesA" => kernel32::SetFileAttributesA(emu),
        "SetFileAttributesW" => kernel32::SetFileAttributesW(emu),
        "BackupRead" => kernel32::BackupRead(emu),
        "BackupSeek" => kernel32::BackupSeek(emu),
        "BackupWrite" => kernel32::BackupWrite(emu),
        "BaseDllReadWriteIniFile" => kernel32::BaseDllReadWriteIniFile(emu),
        "BasepGetComputerNameFromNtPath" => kernel32::BasepGetComputerNameFromNtPath(emu),
        "BasepSetFileEncryptionCompression" => kernel32::BasepSetFileEncryptionCompression(emu),
        "BindIoCompletionCallback" => kernel32::BindIoCompletionCallback(emu),
        "BuildCommDCBAndTimeoutsA" => kernel32::BuildCommDCBAndTimeoutsA(emu),
        "BuildCommDCBAndTimeoutsW" => kernel32::BuildCommDCBAndTimeoutsW(emu),
        "BuildCommDCBA" => kernel32::BuildCommDCBA(emu),
        "BuildCommDCBW" => kernel32::BuildCommDCBW(emu),
        "CallNamedPipeA" => kernel32::CallNamedPipeA(emu),
        "CheckNameLegalDOS8Dot3A" => kernel32::CheckNameLegalDOS8Dot3A(emu),
        "CheckNameLegalDOS8Dot3W" => kernel32::CheckNameLegalDOS8Dot3W(emu),
        "CommConfigDialogA" => kernel32::CommConfigDialogA(emu),
        "CopyFileExA" => kernel32::CopyFileExA(emu),
        "CopyFileTransactedA" => kernel32::CopyFileTransactedA(emu),
        "CopyFileTransactedW" => kernel32::CopyFileTransactedW(emu),
        "CopyLZFile" => kernel32::CopyLZFile(emu),
        "CreateDirectoryExA" => kernel32::CreateDirectoryExA(emu),
        "CreateDirectoryTransactedA" => kernel32::CreateDirectoryTransactedA(emu),
        "CreateDirectoryTransactedW" => kernel32::CreateDirectoryTransactedW(emu),
        "CreateFileMappingNumaA" => kernel32::CreateFileMappingNumaA(emu),
        "CreateHardLinkTransactedA" => kernel32::CreateHardLinkTransactedA(emu),
        "CreateSymbolicLinkA" => kernel32::CreateSymbolicLinkA(emu),
        "CreateSymbolicLinkTransactedA" => kernel32::CreateSymbolicLinkTransactedA(emu),
        "CreateSymbolicLinkTransactedW" => kernel32::CreateSymbolicLinkTransactedW(emu),
        "CreateTapePartition" => kernel32::CreateTapePartition(emu),
        "DefineDosDeviceA" => kernel32::DefineDosDeviceA(emu),
        "DeleteFileTransactedW" => kernel32::DeleteFileTransactedW(emu),
        "DeleteVolumeMountPointA" => kernel32::DeleteVolumeMountPointA(emu),
        "DeviceIoControlImplementation" => kernel32::DeviceIoControlImplementation(emu),
        "DeviceNameCompare" => kernel32::DeviceNameCompare(emu),
        "DosPathToSessionPathA" => kernel32::DosPathToSessionPathA(emu),
        "DosPathToSessionPathW" => kernel32::DosPathToSessionPathW(emu),
        "DuplicateEncryptionInfoFileExt" => kernel32::DuplicateEncryptionInfoFileExt(emu),
        "FindFirstVolumeA" => kernel32::FindFirstVolumeA(emu),
        "FindFirstVolumeMountPointA" => kernel32::FindFirstVolumeMountPointA(emu),
        "FindFirstVolumeMountPointW" => kernel32::FindFirstVolumeMountPointW(emu),
        "FindNextVolumeA" => kernel32::FindNextVolumeA(emu),
        "FindNextVolumeMountPointA" => kernel32::FindNextVolumeMountPointA(emu),
        "FindNextVolumeMountPointW" => kernel32::FindNextVolumeMountPointW(emu),
        "FindVolumeMountPointClose" => kernel32::FindVolumeMountPointClose(emu),
        "GetCompressedFileSizeTransactedW" => kernel32::GetCompressedFileSizeTransactedW(emu),
        "GetDefaultCommConfigA" => kernel32::GetDefaultCommConfigA(emu),
        "GetDefaultCommConfigW" => kernel32::GetDefaultCommConfigW(emu),
        "GetEncryptedFileVersionExt" => kernel32::GetEncryptedFileVersionExt(emu),
        "GetFileAttributesTransactedA" => kernel32::GetFileAttributesTransactedA(emu),
        "GetFileBandwidthReservation" => kernel32::GetFileBandwidthReservation(emu),
        "GetFriendlyMatchComm" => kernel32::GetFriendlyMatchComm(emu),
        "GetFullPathNameTransactedA" => kernel32::GetFullPathNameTransactedA(emu),
        "GetFullPathNameTransactedW" => kernel32::GetFullPathNameTransactedW(emu),
        "GetLongPathNameA" => kernel32::GetLongPathNameA(emu),
        "GetMailslotInfo" => kernel32::GetMailslotInfo(emu),
        "GetNamedPipeClientComputerNameA" => kernel32::GetNamedPipeClientComputerNameA(emu),
        "GetNamedPipeClientProcessId" => kernel32::GetNamedPipeClientProcessId(emu),
        "GetNamedPipeClientSessionId" => kernel32::GetNamedPipeClientSessionId(emu),
        "GetNamedPipeServerSessionId" => kernel32::GetNamedPipeServerSessionId(emu),
        "GetShortPathNameA" => kernel32::GetShortPathNameA(emu),
        "GetShortPathNameW" => kernel32::GetShortPathNameW(emu),
        "GetTapeParameters" => kernel32::GetTapeParameters(emu),
        "GetTapePosition" => kernel32::GetTapePosition(emu),
        "GetTapeStatus" => kernel32::GetTapeStatus(emu),
        "GetVolumeNameForVolumeMountPointA" => kernel32::GetVolumeNameForVolumeMountPointA(emu),
        "GetVolumePathNamesForVolumeNameA" => kernel32::GetVolumePathNamesForVolumeNameA(emu),
        "LZClose" => kernel32::LZClose(emu),
        "LZCopy" => kernel32::LZCopy(emu),
        "LZInit" => kernel32::LZInit(emu),
        "LZOpenFileA" => kernel32::LZOpenFileA(emu),
        "LZOpenFileW" => kernel32::LZOpenFileW(emu),
        "LZRead" => kernel32::LZRead(emu),
        "LZSeek" => kernel32::LZSeek(emu),
        "LZStart" => kernel32::LZStart(emu),
        "MoveFileExA" => kernel32::MoveFileExA(emu),
        "MoveFileTransactedA" => kernel32::MoveFileTransactedA(emu),
        "MoveFileTransactedW" => kernel32::MoveFileTransactedW(emu),
        "NtWow64CsrBasepRefreshIniFileMapping" => {
            kernel32::NtWow64CsrBasepRefreshIniFileMapping(emu)
        }
        "OpenFile" => kernel32::OpenFile(emu),
        "OpenFileMappingA" => kernel32::OpenFileMappingA(emu),
        "PrepareTape" => kernel32::PrepareTape(emu),
        "PrivMoveFileIdentityW" => kernel32::PrivMoveFileIdentityW(emu),
        "QueryDosDeviceA" => kernel32::QueryDosDeviceA(emu),
        "RemoveDirectoryTransactedW" => kernel32::RemoveDirectoryTransactedW(emu),
        "ReplaceFileA" => kernel32::ReplaceFileA(emu),
        "SetDefaultCommConfigA" => kernel32::SetDefaultCommConfigA(emu),
        "SetFileAttributesTransactedW" => kernel32::SetFileAttributesTransactedW(emu),
        "SetFileBandwidthReservation" => kernel32::SetFileBandwidthReservation(emu),
        "SetFileShortNameA" => kernel32::SetFileShortNameA(emu),
        "SetFileShortNameW" => kernel32::SetFileShortNameW(emu),
        "SetMailslotInfo" => kernel32::SetMailslotInfo(emu),
        "SetNamedPipeAttribute" => kernel32::SetNamedPipeAttribute(emu),
        "SetTapeParameters" => kernel32::SetTapeParameters(emu),
        "SetTapePosition" => kernel32::SetTapePosition(emu),
        "SetVolumeLabelA" => kernel32::SetVolumeLabelA(emu),
        "SetVolumeLabelW" => kernel32::SetVolumeLabelW(emu),
        "SetVolumeMountPointA" => kernel32::SetVolumeMountPointA(emu),
        "SetVolumeMountPointW" => kernel32::SetVolumeMountPointW(emu),
        "WaitNamedPipeA" => kernel32::WaitNamedPipeA(emu),
        "WriteTapemark" => kernel32::WriteTapemark(emu),
        "AreShortNamesEnabled" => kernel32::AreShortNamesEnabled(emu),
        "BackupReadEx" => kernel32::BackupReadEx(emu),
        "BackupWriteEx" => kernel32::BackupWriteEx(emu),
        "BaseGetNamedObjectDirectory" => kernel32::BaseGetNamedObjectDirectory(emu),
        "BuildIoRingCancelRequest" => kernel32::BuildIoRingCancelRequest(emu),
        "BuildIoRingFlushFile" => kernel32::BuildIoRingFlushFile(emu),
        "BuildIoRingReadFile" => kernel32::BuildIoRingReadFile(emu),
        "BuildIoRingReadFileScatter" => kernel32::BuildIoRingReadFileScatter(emu),
        "BuildIoRingRegisterBuffers" => kernel32::BuildIoRingRegisterBuffers(emu),
        "BuildIoRingWriteFile" => kernel32::BuildIoRingWriteFile(emu),
        "BuildIoRingWriteFileGather" => kernel32::BuildIoRingWriteFileGather(emu),
        "CallNamedPipeW" => kernel32::CallNamedPipeW(emu),
        "CancelIo" => kernel32::CancelIo(emu),
        "CancelIoEx" => kernel32::CancelIoEx(emu),
        "CancelSynchronousIo" => kernel32::CancelSynchronousIo(emu),
        "ClearCommBreak" => kernel32::ClearCommBreak(emu),
        "ClearCommError" => kernel32::ClearCommError(emu),
        "CloseIoRing" => kernel32::CloseIoRing(emu),
        "CommConfigDialogW" => kernel32::CommConfigDialogW(emu),
        "CopyFile2" => kernel32::CopyFile2(emu),
        "CopyFileExW" => kernel32::CopyFileExW(emu),
        "CreateDirectory2A" => kernel32::CreateDirectory2A(emu),
        "CreateDirectory2W" => kernel32::CreateDirectory2W(emu),
        "CreateDirectoryExW" => kernel32::CreateDirectoryExW(emu),
        "CreateFile2" => kernel32::CreateFile2(emu),
        "CreateFile3" => kernel32::CreateFile3(emu),
        "CreateFileMappingFromApp" => kernel32::CreateFileMappingFromApp(emu),
        "CreateFileMappingNumaW" => kernel32::CreateFileMappingNumaW(emu),
        "CreateFileTransactedA" => kernel32::CreateFileTransactedA(emu),
        "CreateFileTransactedW" => kernel32::CreateFileTransactedW(emu),
        "CreateHardLinkA" => kernel32::CreateHardLinkA(emu),
        "CreateHardLinkTransactedW" => kernel32::CreateHardLinkTransactedW(emu),
        "CreateHardLinkW" => kernel32::CreateHardLinkW(emu),
        "CreateIoCompletionPort" => kernel32::CreateIoCompletionPort(emu),
        "CreateIoRing" => kernel32::CreateIoRing(emu),
        "CreateMailslotA" => kernel32::CreateMailslotA(emu),
        "CreateMailslotW" => kernel32::CreateMailslotW(emu),
        "CreateNamedPipeW" => kernel32::CreateNamedPipeW(emu),
        "CreatePipe" => kernel32::CreatePipe(emu),
        "CreateSymbolicLinkW" => kernel32::CreateSymbolicLinkW(emu),
        "DefineDosDeviceW" => kernel32::DefineDosDeviceW(emu),
        "DeleteFile2A" => kernel32::DeleteFile2A(emu),
        "DeleteFile2W" => kernel32::DeleteFile2W(emu),
        "DeleteFileTransactedA" => kernel32::DeleteFileTransactedA(emu),
        "DeleteVolumeMountPointW" => kernel32::DeleteVolumeMountPointW(emu),
        "DeviceIoControl" => kernel32::DeviceIoControl(emu),
        "EscapeCommFunction" => kernel32::EscapeCommFunction(emu),
        "ExpungeConsoleCommandHistoryA" => kernel32::ExpungeConsoleCommandHistoryA(emu),
        "ExpungeConsoleCommandHistoryW" => kernel32::ExpungeConsoleCommandHistoryW(emu),
        "FindCloseChangeNotification" => kernel32::FindCloseChangeNotification(emu),
        "FindFirstChangeNotificationA" => kernel32::FindFirstChangeNotificationA(emu),
        "FindFirstChangeNotificationW" => kernel32::FindFirstChangeNotificationW(emu),
        "FindFirstFileExA" => kernel32::FindFirstFileExA(emu),
        "FindFirstFileExW" => kernel32::FindFirstFileExW(emu),
        "FindFirstFileNameTransactedW" => kernel32::FindFirstFileNameTransactedW(emu),
        "FindFirstFileNameW" => kernel32::FindFirstFileNameW(emu),
        "FindFirstFileTransactedA" => kernel32::FindFirstFileTransactedA(emu),
        "FindFirstFileTransactedW" => kernel32::FindFirstFileTransactedW(emu),
        "FindFirstStreamTransactedW" => kernel32::FindFirstStreamTransactedW(emu),
        "FindFirstStreamW" => kernel32::FindFirstStreamW(emu),
        "FindFirstVolumeW" => kernel32::FindFirstVolumeW(emu),
        "FindNextChangeNotification" => kernel32::FindNextChangeNotification(emu),
        "FindNextFileNameW" => kernel32::FindNextFileNameW(emu),
        "FindNextStreamW" => kernel32::FindNextStreamW(emu),
        "FindNextVolumeW" => kernel32::FindNextVolumeW(emu),
        "FindVolumeClose" => kernel32::FindVolumeClose(emu),
        "GetCommConfig" => kernel32::GetCommConfig(emu),
        "GetCommMask" => kernel32::GetCommMask(emu),
        "GetCommModemStatus" => kernel32::GetCommModemStatus(emu),
        "GetCommProperties" => kernel32::GetCommProperties(emu),
        "GetCommState" => kernel32::GetCommState(emu),
        "GetCommTimeouts" => kernel32::GetCommTimeouts(emu),
        "GetCompressedFileSizeA" => kernel32::GetCompressedFileSizeA(emu),
        "GetCompressedFileSizeTransactedA" => kernel32::GetCompressedFileSizeTransactedA(emu),
        "GetCompressedFileSizeW" => kernel32::GetCompressedFileSizeW(emu),
        "GetConsoleCommandHistoryA" => kernel32::GetConsoleCommandHistoryA(emu),
        "GetConsoleCommandHistoryLengthA" => kernel32::GetConsoleCommandHistoryLengthA(emu),
        "GetConsoleCommandHistoryLengthW" => kernel32::GetConsoleCommandHistoryLengthW(emu),
        "GetConsoleCommandHistoryW" => kernel32::GetConsoleCommandHistoryW(emu),
        "GetDiskSpaceInformationA" => kernel32::GetDiskSpaceInformationA(emu),
        "GetDiskSpaceInformationW" => kernel32::GetDiskSpaceInformationW(emu),
        "GetFileAttributesExA" => kernel32::GetFileAttributesExA(emu),
        "GetFileAttributesExW" => kernel32::GetFileAttributesExW(emu),
        "GetFileAttributesTransactedW" => kernel32::GetFileAttributesTransactedW(emu),
        "GetFileInformationByName" => kernel32::GetFileInformationByName(emu),
        "GetFileMUIInfo" => kernel32::GetFileMUIInfo(emu),
        "GetFileMUIPath" => kernel32::GetFileMUIPath(emu),
        "GetIoRingInfo" => kernel32::GetIoRingInfo(emu),
        "GetLongPathNameTransactedA" => kernel32::GetLongPathNameTransactedA(emu),
        "GetLongPathNameTransactedW" => kernel32::GetLongPathNameTransactedW(emu),
        "GetNamedPipeAttribute" => kernel32::GetNamedPipeAttribute(emu),
        "GetNamedPipeClientComputerNameW" => kernel32::GetNamedPipeClientComputerNameW(emu),
        "GetNamedPipeInfo" => kernel32::GetNamedPipeInfo(emu),
        "GetNamedPipeServerProcessId" => kernel32::GetNamedPipeServerProcessId(emu),
        "GetNextVDMCommand" => kernel32::GetNextVDMCommand(emu),
        "GetOverlappedResult" => kernel32::GetOverlappedResult(emu),
        "GetOverlappedResultEx" => kernel32::GetOverlappedResultEx(emu),
        "GetQueuedCompletionStatus" => kernel32::GetQueuedCompletionStatus(emu),
        "GetQueuedCompletionStatusEx" => kernel32::GetQueuedCompletionStatusEx(emu),
        "GetSystemFileCacheSize" => kernel32::GetSystemFileCacheSize(emu),
        "GetSystemWow64DirectoryA" => kernel32::GetSystemWow64DirectoryA(emu),
        "GetSystemWow64DirectoryW" => kernel32::GetSystemWow64DirectoryW(emu),
        "GetTempFileNameA" => kernel32::GetTempFileNameA(emu),
        "GetTempFileNameW" => kernel32::GetTempFileNameW(emu),
        "GetTempPath2A" => kernel32::GetTempPath2A(emu),
        "GetTempPath2W" => kernel32::GetTempPath2W(emu),
        "GetVolumeNameForVolumeMountPointW" => kernel32::GetVolumeNameForVolumeMountPointW(emu),
        "GetVolumePathNameA" => kernel32::GetVolumePathNameA(emu),
        "GetVolumePathNameW" => kernel32::GetVolumePathNameW(emu),
        "GetVolumePathNamesForVolumeNameW" => kernel32::GetVolumePathNamesForVolumeNameW(emu),
        "IsIoRingOpSupported" => kernel32::IsIoRingOpSupported(emu),
        "K32EnumDeviceDrivers" => kernel32::K32EnumDeviceDrivers(emu),
        "K32EnumPageFilesA" => kernel32::K32EnumPageFilesA(emu),
        "K32EnumPageFilesW" => kernel32::K32EnumPageFilesW(emu),
        "K32GetDeviceDriverBaseNameA" => kernel32::K32GetDeviceDriverBaseNameA(emu),
        "K32GetDeviceDriverBaseNameW" => kernel32::K32GetDeviceDriverBaseNameW(emu),
        "K32GetDeviceDriverFileNameA" => kernel32::K32GetDeviceDriverFileNameA(emu),
        "K32GetDeviceDriverFileNameW" => kernel32::K32GetDeviceDriverFileNameW(emu),
        "K32GetMappedFileNameA" => kernel32::K32GetMappedFileNameA(emu),
        "K32GetMappedFileNameW" => kernel32::K32GetMappedFileNameW(emu),
        "LZCloseFile" => kernel32::LZCloseFile(emu),
        "LZCreateFileW" => kernel32::LZCreateFileW(emu),
        "LZDone" => kernel32::LZDone(emu),
        "LockFile" => kernel32::LockFile(emu),
        "LockFileEx" => kernel32::LockFileEx(emu),
        "MoveFileExW" => kernel32::MoveFileExW(emu),
        "MoveFileWithProgressA" => kernel32::MoveFileWithProgressA(emu),
        "MoveFileWithProgressW" => kernel32::MoveFileWithProgressW(emu),
        "NeedCurrentDirectoryForExePathA" => kernel32::NeedCurrentDirectoryForExePathA(emu),
        "NeedCurrentDirectoryForExePathW" => kernel32::NeedCurrentDirectoryForExePathW(emu),
        "NotifyMountMgr" => kernel32::NotifyMountMgr(emu),
        "OpenFileById" => kernel32::OpenFileById(emu),
        "OpenFileMappingW" => kernel32::OpenFileMappingW(emu),
        "PeekNamedPipe" => kernel32::PeekNamedPipe(emu),
        "PopIoRingCompletion" => kernel32::PopIoRingCompletion(emu),
        "PostQueuedCompletionStatus" => kernel32::PostQueuedCompletionStatus(emu),
        "PrivCopyFileExW" => kernel32::PrivCopyFileExW(emu),
        "PurgeComm" => kernel32::PurgeComm(emu),
        "QueryDosDeviceW" => kernel32::QueryDosDeviceW(emu),
        "QueryIoRingCapabilities" => kernel32::QueryIoRingCapabilities(emu),
        "ReOpenFile" => kernel32::ReOpenFile(emu),
        "ReadDirectoryChangesExW" => kernel32::ReadDirectoryChangesExW(emu),
        "ReadDirectoryChangesW" => kernel32::ReadDirectoryChangesW(emu),
        "ReadFileEx" => kernel32::ReadFileEx(emu),
        "ReadFileScatter" => kernel32::ReadFileScatter(emu),
        "RemoveDirectory2A" => kernel32::RemoveDirectory2A(emu),
        "RemoveDirectory2W" => kernel32::RemoveDirectory2W(emu),
        "RemoveDirectoryTransactedA" => kernel32::RemoveDirectoryTransactedA(emu),
        "ReplaceFile" => kernel32::ReplaceFile(emu),
        "ReplaceFileW" => kernel32::ReplaceFileW(emu),
        "RtlPcToFileHeader" => kernel32::RtlPcToFileHeader(emu),
        "SearchPathA" => kernel32::SearchPathA(emu),
        "SearchPathW" => kernel32::SearchPathW(emu),
        "SetCommBreak" => kernel32::SetCommBreak(emu),
        "SetCommConfig" => kernel32::SetCommConfig(emu),
        "SetCommMask" => kernel32::SetCommMask(emu),
        "SetCommState" => kernel32::SetCommState(emu),
        "SetCommTimeouts" => kernel32::SetCommTimeouts(emu),
        "SetConsoleNumberOfCommandsA" => kernel32::SetConsoleNumberOfCommandsA(emu),
        "SetConsoleNumberOfCommandsW" => kernel32::SetConsoleNumberOfCommandsW(emu),
        "SetDefaultCommConfigW" => kernel32::SetDefaultCommConfigW(emu),
        "SetFileAttributesTransactedA" => kernel32::SetFileAttributesTransactedA(emu),
        "SetFileCompletionNotificationModes" => kernel32::SetFileCompletionNotificationModes(emu),
        "SetFileIoOverlappedRange" => kernel32::SetFileIoOverlappedRange(emu),
        "SetFileValidData" => kernel32::SetFileValidData(emu),
        "SetIoRingCompletionEvent" => kernel32::SetIoRingCompletionEvent(emu),
        "SetSearchPathMode" => kernel32::SetSearchPathMode(emu),
        "SetSystemFileCacheSize" => kernel32::SetSystemFileCacheSize(emu),
        "SetVolumeMountPointWStub" => kernel32::SetVolumeMountPointWStub(emu),
        "SetupComm" => kernel32::SetupComm(emu),
        "SubmitIoRing" => kernel32::SubmitIoRing(emu),
        "TransactNamedPipe" => kernel32::TransactNamedPipe(emu),
        "TransmitCommChar" => kernel32::TransmitCommChar(emu),
        "UnlockFile" => kernel32::UnlockFile(emu),
        "UnlockFileEx" => kernel32::UnlockFileEx(emu),
        "WaitCommEvent" => kernel32::WaitCommEvent(emu),
        "WaitNamedPipeW" => kernel32::WaitNamedPipeW(emu),
        "WriteFileEx" => kernel32::WriteFileEx(emu),
        "WriteFileGather" => kernel32::WriteFileGather(emu),
        "CreateUmsCompletionList" => kernel32::CreateUmsCompletionList(emu),
        "DeleteUmsCompletionList" => kernel32::DeleteUmsCompletionList(emu),
        "DequeueUmsCompletionListItems" => kernel32::DequeueUmsCompletionListItems(emu),
        "GetUmsCompletionListEvent" => kernel32::GetUmsCompletionListEvent(emu),
        _ => return false,
    }
    true
}
