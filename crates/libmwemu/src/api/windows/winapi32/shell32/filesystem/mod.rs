use crate::emu::Emu;
use crate::winapi::winapi32::shell32;

mod str_str;
pub use str_str::{StrStrA, StrStrW};

mod str_r_str;
pub use str_r_str::{StrRStrA, StrRStrW};

mod str_r_chr;
pub use str_r_chr::{StrRChrA, StrRChrW};

mod str_n_cmp;
pub use str_n_cmp::{StrNCmpA, StrNCmpW};

mod str_chr;
pub use str_chr::{StrChrA, StrChrW};

mod shell_message_box;
pub use shell_message_box::{ShellMessageBoxA, ShellMessageBoxW};

mod shell_about;
pub use shell_about::{ShellAboutA, ShellAboutW};

mod sh_set_folder_path;
pub use sh_set_folder_path::{SHSetFolderPathA, SHSetFolderPathW};

mod sh_query_recycle_bin;
pub use sh_query_recycle_bin::{SHQueryRecycleBinA, SHQueryRecycleBinW};

mod sh_path_prepare_for_write;
pub use sh_path_prepare_for_write::{SHPathPrepareForWriteA, SHPathPrepareForWriteW};

mod sh_invoke_printer_command;
pub use sh_invoke_printer_command::{SHInvokePrinterCommandA, SHInvokePrinterCommandW};

mod sh_get_special_folder_path;
pub use sh_get_special_folder_path::{SHGetSpecialFolderPathA, SHGetSpecialFolderPathW};

mod sh_get_folder_path_and_sub_dir;
pub use sh_get_folder_path_and_sub_dir::{SHGetFolderPathAndSubDirA, SHGetFolderPathAndSubDirW};

mod sh_get_folder_path;
pub use sh_get_folder_path::{SHGetFolderPathA, SHGetFolderPathW};

mod sh_get_disk_free_space_ex;
pub use sh_get_disk_free_space_ex::{SHGetDiskFreeSpaceExA, SHGetDiskFreeSpaceExW};

mod sh_empty_recycle_bin;
pub use sh_empty_recycle_bin::{SHEmptyRecycleBinA, SHEmptyRecycleBinW};

mod sh_create_directory_ex;
pub use sh_create_directory_ex::{SHCreateDirectoryExA, SHCreateDirectoryExW};

mod path_is_slow;
pub use path_is_slow::{PathIsSlowA, PathIsSlowW};

mod call_file_copy_hook;
pub use call_file_copy_hook::CallFileCopyHook;

mod check_escapes_w;
pub use check_escapes_w::CheckEscapesW;

mod drive_type;
pub use drive_type::DriveType;

mod init_network_address_control;
pub use init_network_address_control::InitNetworkAddressControl;

mod is_lfn_drive;
pub use is_lfn_drive::{IsLFNDrive, IsLFNDriveA, IsLFNDriveW};

mod is_net_drive;
pub use is_net_drive::IsNetDrive;

mod path_cleanup_spec;
pub use path_cleanup_spec::PathCleanupSpec;

mod path_get_short_path;
pub use path_get_short_path::PathGetShortPath;

mod path_is_exe;
pub use path_is_exe::PathIsExe;

mod path_make_unique_name;
pub use path_make_unique_name::PathMakeUniqueName;

mod path_qualify;
pub use path_qualify::PathQualify;

mod path_resolve;
pub use path_resolve::PathResolve;

mod path_yet_another_make_unique_name;
pub use path_yet_another_make_unique_name::PathYetAnotherMakeUniqueName;

mod read_cabinet_state;
pub use read_cabinet_state::ReadCabinetState;

mod real_drive_type;
pub use real_drive_type::RealDriveType;

mod sh_add_to_recent_docs;
pub use sh_add_to_recent_docs::SHAddToRecentDocs;

mod sh_change_notification_lock;
pub use sh_change_notification_lock::SHChangeNotification_Lock;

mod sh_change_notification_unlock;
pub use sh_change_notification_unlock::SHChangeNotification_Unlock;

mod sh_change_notify;
pub use sh_change_notify::SHChangeNotify;

mod sh_change_notify_deregister;
pub use sh_change_notify_deregister::SHChangeNotifyDeregister;

mod sh_change_notify_register;
pub use sh_change_notify_register::SHChangeNotifyRegister;

mod sh_change_notify_register_thread;
pub use sh_change_notify_register_thread::SHChangeNotifyRegisterThread;

mod sh_change_notify_suspend_resume;
pub use sh_change_notify_suspend_resume::SHChangeNotifySuspendResume;

mod sh_create_category_enum;
pub use sh_create_category_enum::SHCreateCategoryEnum;

mod sh_create_directory;
pub use sh_create_directory::SHCreateDirectory;

mod sh_create_std_enum_fmt_etc;
pub use sh_create_std_enum_fmt_etc::SHCreateStdEnumFmtEtc;

mod shell32_are_all_items_available;
pub use shell32_are_all_items_available::SHELL32_AreAllItemsAvailable;

mod shell32_c_common_places_folder_create_instance;
pub use shell32_c_common_places_folder_create_instance::SHELL32_CCommonPlacesFolder_CreateInstance;

mod shell32_cd_burn_close_session;
pub use shell32_cd_burn_close_session::SHELL32_CDBurn_CloseSession;

mod shell32_cd_burn_drive_supported_for_data_burn;
pub use shell32_cd_burn_drive_supported_for_data_burn::SHELL32_CDBurn_DriveSupportedForDataBurn;

mod shell32_cd_burn_erase;
pub use shell32_cd_burn_erase::SHELL32_CDBurn_Erase;

mod shell32_cd_burn_get_cd_info;
pub use shell32_cd_burn_get_cd_info::SHELL32_CDBurn_GetCDInfo;

mod shell32_cd_burn_get_live_fs_disc_info;
pub use shell32_cd_burn_get_live_fs_disc_info::SHELL32_CDBurn_GetLiveFSDiscInfo;

mod shell32_cd_burn_get_staging_path_or_normal_path;
pub use shell32_cd_burn_get_staging_path_or_normal_path::SHELL32_CDBurn_GetStagingPathOrNormalPath;

mod shell32_cd_burn_get_task_info;
pub use shell32_cd_burn_get_task_info::SHELL32_CDBurn_GetTaskInfo;

mod shell32_cd_burn_is_blank_disc;
pub use shell32_cd_burn_is_blank_disc::SHELL32_CDBurn_IsBlankDisc;

mod shell32_cd_burn_is_blank_disc2;
pub use shell32_cd_burn_is_blank_disc2::SHELL32_CDBurn_IsBlankDisc2;

mod shell32_cd_burn_is_live_fs;
pub use shell32_cd_burn_is_live_fs::SHELL32_CDBurn_IsLiveFS;

mod shell32_cd_burn_on_device_change;
pub use shell32_cd_burn_on_device_change::SHELL32_CDBurn_OnDeviceChange;

mod shell32_cd_burn_on_eject;
pub use shell32_cd_burn_on_eject::SHELL32_CDBurn_OnEject;

mod shell32_cd_burn_on_media_change;
pub use shell32_cd_burn_on_media_change::SHELL32_CDBurn_OnMediaChange;

mod shell32_c_drives_create_sfvcb;
pub use shell32_c_drives_create_sfvcb::SHELL32_CDrives_CreateSFVCB;

mod shell32_cfs_folder_callback_create;
pub use shell32_cfs_folder_callback_create::SHELL32_CFSFolderCallback_Create;

mod shell32_cpl_category_id_array_from_variant;
pub use shell32_cpl_category_id_array_from_variant::SHELL32_CPL_CategoryIdArrayFromVariant;

mod shell32_cpl_is_legacy_canonical_name_listed_under_key;
pub use shell32_cpl_is_legacy_canonical_name_listed_under_key::SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey;

mod shell32_cpl_modify_wow_display_name;
pub use shell32_cpl_modify_wow_display_name::SHELL32_CPL_ModifyWowDisplayName;

mod shell32_c_transfer_confirmation_create_instance;
pub use shell32_c_transfer_confirmation_create_instance::SHELL32_CTransferConfirmation_CreateInstance;

mod shell32_call_file_copy_hooks;
pub use shell32_call_file_copy_hooks::SHELL32_CallFileCopyHooks;

mod shell32_copy_secondary_tiles;
pub use shell32_copy_secondary_tiles::SHELL32_CopySecondaryTiles;

mod shell32_create_confirmation_interrupt;
pub use shell32_create_confirmation_interrupt::SHELL32_CreateConfirmationInterrupt;

mod shell32_create_conflict_interrupt;
pub use shell32_create_conflict_interrupt::SHELL32_CreateConflictInterrupt;

mod shell32_create_default_operation_data_provider;
pub use shell32_create_default_operation_data_provider::SHELL32_CreateDefaultOperationDataProvider;

mod shell32_create_link_info_w;
pub use shell32_create_link_info_w::SHELL32_CreateLinkInfoW;

mod shell32_create_qos_recorder;
pub use shell32_create_qos_recorder::SHELL32_CreateQosRecorder;

mod shell32_create_share_point_view;
pub use shell32_create_share_point_view::SHELL32_CreateSharePointView;

mod shell32_destroy_link_info;
pub use shell32_destroy_link_info::SHELL32_DestroyLinkInfo;

mod shell32_encrypt_directory;
pub use shell32_encrypt_directory::SHELL32_EncryptDirectory;

mod shell32_encrypted_file_key_info;
pub use shell32_encrypted_file_key_info::SHELL32_EncryptedFileKeyInfo;

mod shell32_enum_common_tasks;
pub use shell32_enum_common_tasks::SHELL32_EnumCommonTasks;

mod shell32_free_encrypted_file_key_info;
pub use shell32_free_encrypted_file_key_info::SHELL32_FreeEncryptedFileKeyInfo;

mod shell32_get_command_provider_for_folder_type;
pub use shell32_get_command_provider_for_folder_type::SHELL32_GetCommandProviderForFolderType;

mod shell32_get_dpi_adjusted_logical_size;
pub use shell32_get_dpi_adjusted_logical_size::SHELL32_GetDPIAdjustedLogicalSize;

mod shell32_get_disk_cleanup_path;
pub use shell32_get_disk_cleanup_path::SHELL32_GetDiskCleanupPath;

mod shell32_get_link_info_data;
pub use shell32_get_link_info_data::SHELL32_GetLinkInfoData;

mod shell32_get_rating_bucket;
pub use shell32_get_rating_bucket::SHELL32_GetRatingBucket;

mod shell32_get_sqmable_file_name;
pub use shell32_get_sqmable_file_name::SHELL32_GetSqmableFileName;

mod shell32_handle_unrecognized_file_system;
pub use shell32_handle_unrecognized_file_system::SHELL32_HandleUnrecognizedFileSystem;

mod shell32_is_get_keyboard_layout_present;
pub use shell32_is_get_keyboard_layout_present::SHELL32_IsGetKeyboardLayoutPresent;

mod shell32_is_system_upgrade_in_progress;
pub use shell32_is_system_upgrade_in_progress::SHELL32_IsSystemUpgradeInProgress;

mod shell32_is_valid_link_info;
pub use shell32_is_valid_link_info::SHELL32_IsValidLinkInfo;

mod shell32_legacy_enum_special_tasks_by_type;
pub use shell32_legacy_enum_special_tasks_by_type::SHELL32_LegacyEnumSpecialTasksByType;

mod shell32_legacy_enum_tasks;
pub use shell32_legacy_enum_tasks::SHELL32_LegacyEnumTasks;

mod shell32_normalize_rating;
pub use shell32_normalize_rating::SHELL32_NormalizeRating;

mod shell32_notify_link_tracking_service_of_move;
pub use shell32_notify_link_tracking_service_of_move::SHELL32_NotifyLinkTrackingServiceOfMove;

mod shell32_printers_create_bind_info;
pub use shell32_printers_create_bind_info::SHELL32_Printers_CreateBindInfo;

mod shell32_resolve_link_info_w;
pub use shell32_resolve_link_info_w::SHELL32_ResolveLinkInfoW;

mod shell32_sh_create_by_value_operation_interrupt;
pub use shell32_sh_create_by_value_operation_interrupt::SHELL32_SHCreateByValueOperationInterrupt;

mod shell32_sh_create_local_server;
pub use shell32_sh_create_local_server::SHELL32_SHCreateLocalServer;

mod shell32_sh_duplicate_encryption_info_file;
pub use shell32_sh_duplicate_encryption_info_file::SHELL32_SHDuplicateEncryptionInfoFile;

mod shell32_sh_encrypt_file;
pub use shell32_sh_encrypt_file::SHELL32_SHEncryptFile;

mod shell32_sh_format_drive_async;
pub use shell32_sh_format_drive_async::SHELL32_SHFormatDriveAsync;

mod shell32_sh_get_thread_undo_manager;
pub use shell32_sh_get_thread_undo_manager::SHELL32_SHGetThreadUndoManager;

mod shell32_sh_get_user_name_w;
pub use shell32_sh_get_user_name_w::SHELL32_SHGetUserNameW;

mod shell32_sh_is_virtual_device;
pub use shell32_sh_is_virtual_device::SHELL32_SHIsVirtualDevice;

mod shell32_sh_log_il_from_fsil;
pub use shell32_sh_log_il_from_fsil::SHELL32_SHLogILFromFSIL;

mod shell32_simple_rating_to_filter_condition;
pub use shell32_simple_rating_to_filter_condition::SHELL32_SimpleRatingToFilterCondition;

mod shell32_suspend_undo;
pub use shell32_suspend_undo::SHELL32_SuspendUndo;

mod sh_enable_service_object;
pub use sh_enable_service_object::SHEnableServiceObject;

mod sh_file_operation;
pub use sh_file_operation::{SHFileOperation, SHFileOperationA, SHFileOperationW};

mod sh_find_files;
pub use sh_find_files::SHFindFiles;

mod sh_flush_sf_cache;
pub use sh_flush_sf_cache::SHFlushSFCache;

mod sh_format_drive;
pub use sh_format_drive::SHFormatDrive;

mod sh_free_name_mappings;
pub use sh_free_name_mappings::SHFreeNameMappings;

mod sh_get_disk_free_space_a;
pub use sh_get_disk_free_space_a::SHGetDiskFreeSpaceA;

mod sh_get_drive_media;
pub use sh_get_drive_media::SHGetDriveMedia;

mod sh_get_file_info;
pub use sh_get_file_info::{SHGetFileInfo, SHGetFileInfoA, SHGetFileInfoW};

mod sh_get_folder_location;
pub use sh_get_folder_location::SHGetFolderLocation;

mod sh_get_folder_path_ex;
pub use sh_get_folder_path_ex::SHGetFolderPathEx;

mod sh_get_known_folder_path;
pub use sh_get_known_folder_path::SHGetKnownFolderPath;

mod sh_get_localized_name;
pub use sh_get_localized_name::SHGetLocalizedName;

mod sh_get_new_link_info;
pub use sh_get_new_link_info::{SHGetNewLinkInfo, SHGetNewLinkInfoA, SHGetNewLinkInfoW};

mod sh_get_set_folder_custom_settings;
pub use sh_get_set_folder_custom_settings::SHGetSetFolderCustomSettings;

mod sh_get_set_settings;
pub use sh_get_set_settings::SHGetSetSettings;

mod sh_get_settings;
pub use sh_get_settings::SHGetSettings;

mod sh_get_special_folder_location;
pub use sh_get_special_folder_location::SHGetSpecialFolderLocation;

mod sh_is_file_available_offline;
pub use sh_is_file_available_offline::SHIsFileAvailableOffline;

mod sh_limit_input_edit;
pub use sh_limit_input_edit::SHLimitInputEdit;

mod sh_load_in_proc;
pub use sh_load_in_proc::SHLoadInProc;

mod sh_open_folder_and_select_items;
pub use sh_open_folder_and_select_items::SHOpenFolderAndSelectItems;

mod sh_parse_display_name;
pub use sh_parse_display_name::SHParseDisplayName;

mod sh_query_user_notification_state;
pub use sh_query_user_notification_state::SHQueryUserNotificationState;

mod sh_remove_localized_name;
pub use sh_remove_localized_name::SHRemoveLocalizedName;

mod sh_resolve_library;
pub use sh_resolve_library::SHResolveLibrary;

mod sh_set_known_folder_path;
pub use sh_set_known_folder_path::SHSetKnownFolderPath;

mod sh_set_localized_name;
pub use sh_set_localized_name::SHSetLocalizedName;

mod sh_test_token_membership;
pub use sh_test_token_membership::SHTestTokenMembership;

mod sh_validate_unc;
pub use sh_validate_unc::SHValidateUNC;

mod she_change_dir_a;
pub use she_change_dir_a::SheChangeDirA;

mod she_change_dir_ex_w;
pub use she_change_dir_ex_w::SheChangeDirExW;

mod she_get_dir_a;
pub use she_get_dir_a::SheGetDirA;

mod she_set_cur_drive;
pub use she_set_cur_drive::SheSetCurDrive;

mod signal_file_open;
pub use signal_file_open::SignalFileOpen;

mod stg_make_unique_name;
pub use stg_make_unique_name::StgMakeUniqueName;

mod str_chr_i;
pub use str_chr_i::{StrChrIA, StrChrIW};

mod str_cmp_n;
pub use str_cmp_n::{StrCmpNA, StrCmpNW};

mod str_cmp_ni;
pub use str_cmp_ni::{StrCmpNIA, StrCmpNIW};

mod str_n_cmp_i;
pub use str_n_cmp_i::{StrNCmpIA, StrNCmpIW};

mod str_r_chr_i;
pub use str_r_chr_i::{StrRChrIA, StrRChrIW};

mod str_r_str_i;
pub use str_r_str_i::{StrRStrIA, StrRStrIW};

mod str_str_i;
pub use str_str_i::{StrStrIA, StrStrIW};

mod win32_delete_file;
pub use win32_delete_file::Win32DeleteFile;

mod write_cabinet_state;
pub use write_cabinet_state::WriteCabinetState;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "CallFileCopyHook" => shell32::CallFileCopyHook(emu),
        "CheckEscapesW" => shell32::CheckEscapesW(emu),
        "DriveType" => shell32::DriveType(emu),
        "InitNetworkAddressControl" => shell32::InitNetworkAddressControl(emu),
        "IsLFNDrive" => shell32::IsLFNDrive(emu),
        "IsLFNDriveA" => shell32::IsLFNDriveA(emu),
        "IsLFNDriveW" => shell32::IsLFNDriveW(emu),
        "IsNetDrive" => shell32::IsNetDrive(emu),
        "PathCleanupSpec" => shell32::PathCleanupSpec(emu),
        "PathGetShortPath" => shell32::PathGetShortPath(emu),
        "PathIsExe" => shell32::PathIsExe(emu),
        "PathIsSlowA" => shell32::PathIsSlowA(emu),
        "PathIsSlowW" => shell32::PathIsSlowW(emu),
        "PathMakeUniqueName" => shell32::PathMakeUniqueName(emu),
        "PathQualify" => shell32::PathQualify(emu),
        "PathResolve" => shell32::PathResolve(emu),
        "PathYetAnotherMakeUniqueName" => shell32::PathYetAnotherMakeUniqueName(emu),
        "ReadCabinetState" => shell32::ReadCabinetState(emu),
        "RealDriveType" => shell32::RealDriveType(emu),
        "SHAddToRecentDocs" => shell32::SHAddToRecentDocs(emu),
        "SHChangeNotification_Lock" => shell32::SHChangeNotification_Lock(emu),
        "SHChangeNotification_Unlock" => shell32::SHChangeNotification_Unlock(emu),
        "SHChangeNotify" => shell32::SHChangeNotify(emu),
        "SHChangeNotifyDeregister" => shell32::SHChangeNotifyDeregister(emu),
        "SHChangeNotifyRegister" => shell32::SHChangeNotifyRegister(emu),
        "SHChangeNotifyRegisterThread" => shell32::SHChangeNotifyRegisterThread(emu),
        "SHChangeNotifySuspendResume" => shell32::SHChangeNotifySuspendResume(emu),
        "SHCreateCategoryEnum" => shell32::SHCreateCategoryEnum(emu),
        "SHCreateDirectory" => shell32::SHCreateDirectory(emu),
        "SHCreateDirectoryExA" => shell32::SHCreateDirectoryExA(emu),
        "SHCreateDirectoryExW" => shell32::SHCreateDirectoryExW(emu),
        "SHCreateStdEnumFmtEtc" => shell32::SHCreateStdEnumFmtEtc(emu),
        "SHELL32_AreAllItemsAvailable" => shell32::SHELL32_AreAllItemsAvailable(emu),
        "SHELL32_CCommonPlacesFolder_CreateInstance" => {
            shell32::SHELL32_CCommonPlacesFolder_CreateInstance(emu)
        }
        "SHELL32_CDBurn_CloseSession" => shell32::SHELL32_CDBurn_CloseSession(emu),
        "SHELL32_CDBurn_DriveSupportedForDataBurn" => {
            shell32::SHELL32_CDBurn_DriveSupportedForDataBurn(emu)
        }
        "SHELL32_CDBurn_Erase" => shell32::SHELL32_CDBurn_Erase(emu),
        "SHELL32_CDBurn_GetCDInfo" => shell32::SHELL32_CDBurn_GetCDInfo(emu),
        "SHELL32_CDBurn_GetLiveFSDiscInfo" => shell32::SHELL32_CDBurn_GetLiveFSDiscInfo(emu),
        "SHELL32_CDBurn_GetStagingPathOrNormalPath" => {
            shell32::SHELL32_CDBurn_GetStagingPathOrNormalPath(emu)
        }
        "SHELL32_CDBurn_GetTaskInfo" => shell32::SHELL32_CDBurn_GetTaskInfo(emu),
        "SHELL32_CDBurn_IsBlankDisc" => shell32::SHELL32_CDBurn_IsBlankDisc(emu),
        "SHELL32_CDBurn_IsBlankDisc2" => shell32::SHELL32_CDBurn_IsBlankDisc2(emu),
        "SHELL32_CDBurn_IsLiveFS" => shell32::SHELL32_CDBurn_IsLiveFS(emu),
        "SHELL32_CDBurn_OnDeviceChange" => shell32::SHELL32_CDBurn_OnDeviceChange(emu),
        "SHELL32_CDBurn_OnEject" => shell32::SHELL32_CDBurn_OnEject(emu),
        "SHELL32_CDBurn_OnMediaChange" => shell32::SHELL32_CDBurn_OnMediaChange(emu),
        "SHELL32_CDrives_CreateSFVCB" => shell32::SHELL32_CDrives_CreateSFVCB(emu),
        "SHELL32_CFSFolderCallback_Create" => shell32::SHELL32_CFSFolderCallback_Create(emu),
        "SHELL32_CPL_CategoryIdArrayFromVariant" => {
            shell32::SHELL32_CPL_CategoryIdArrayFromVariant(emu)
        }
        "SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey" => {
            shell32::SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey(emu)
        }
        "SHELL32_CPL_ModifyWowDisplayName" => shell32::SHELL32_CPL_ModifyWowDisplayName(emu),
        "SHELL32_CTransferConfirmation_CreateInstance" => {
            shell32::SHELL32_CTransferConfirmation_CreateInstance(emu)
        }
        "SHELL32_CallFileCopyHooks" => shell32::SHELL32_CallFileCopyHooks(emu),
        "SHELL32_CopySecondaryTiles" => shell32::SHELL32_CopySecondaryTiles(emu),
        "SHELL32_CreateConfirmationInterrupt" => shell32::SHELL32_CreateConfirmationInterrupt(emu),
        "SHELL32_CreateConflictInterrupt" => shell32::SHELL32_CreateConflictInterrupt(emu),
        "SHELL32_CreateDefaultOperationDataProvider" => {
            shell32::SHELL32_CreateDefaultOperationDataProvider(emu)
        }
        "SHELL32_CreateLinkInfoW" => shell32::SHELL32_CreateLinkInfoW(emu),
        "SHELL32_CreateQosRecorder" => shell32::SHELL32_CreateQosRecorder(emu),
        "SHELL32_CreateSharePointView" => shell32::SHELL32_CreateSharePointView(emu),
        "SHELL32_DestroyLinkInfo" => shell32::SHELL32_DestroyLinkInfo(emu),
        "SHELL32_EncryptDirectory" => shell32::SHELL32_EncryptDirectory(emu),
        "SHELL32_EncryptedFileKeyInfo" => shell32::SHELL32_EncryptedFileKeyInfo(emu),
        "SHELL32_EnumCommonTasks" => shell32::SHELL32_EnumCommonTasks(emu),
        "SHELL32_FreeEncryptedFileKeyInfo" => shell32::SHELL32_FreeEncryptedFileKeyInfo(emu),
        "SHELL32_GetCommandProviderForFolderType" => {
            shell32::SHELL32_GetCommandProviderForFolderType(emu)
        }
        "SHELL32_GetDPIAdjustedLogicalSize" => shell32::SHELL32_GetDPIAdjustedLogicalSize(emu),
        "SHELL32_GetDiskCleanupPath" => shell32::SHELL32_GetDiskCleanupPath(emu),
        "SHELL32_GetLinkInfoData" => shell32::SHELL32_GetLinkInfoData(emu),
        "SHELL32_GetRatingBucket" => shell32::SHELL32_GetRatingBucket(emu),
        "SHELL32_GetSqmableFileName" => shell32::SHELL32_GetSqmableFileName(emu),
        "SHELL32_HandleUnrecognizedFileSystem" => {
            shell32::SHELL32_HandleUnrecognizedFileSystem(emu)
        }
        "SHELL32_IsGetKeyboardLayoutPresent" => shell32::SHELL32_IsGetKeyboardLayoutPresent(emu),
        "SHELL32_IsSystemUpgradeInProgress" => shell32::SHELL32_IsSystemUpgradeInProgress(emu),
        "SHELL32_IsValidLinkInfo" => shell32::SHELL32_IsValidLinkInfo(emu),
        "SHELL32_LegacyEnumSpecialTasksByType" => {
            shell32::SHELL32_LegacyEnumSpecialTasksByType(emu)
        }
        "SHELL32_LegacyEnumTasks" => shell32::SHELL32_LegacyEnumTasks(emu),
        "SHELL32_NormalizeRating" => shell32::SHELL32_NormalizeRating(emu),
        "SHELL32_NotifyLinkTrackingServiceOfMove" => {
            shell32::SHELL32_NotifyLinkTrackingServiceOfMove(emu)
        }
        "SHELL32_Printers_CreateBindInfo" => shell32::SHELL32_Printers_CreateBindInfo(emu),
        "SHELL32_ResolveLinkInfoW" => shell32::SHELL32_ResolveLinkInfoW(emu),
        "SHELL32_SHCreateByValueOperationInterrupt" => {
            shell32::SHELL32_SHCreateByValueOperationInterrupt(emu)
        }
        "SHELL32_SHCreateLocalServer" => shell32::SHELL32_SHCreateLocalServer(emu),
        "SHELL32_SHDuplicateEncryptionInfoFile" => {
            shell32::SHELL32_SHDuplicateEncryptionInfoFile(emu)
        }
        "SHELL32_SHEncryptFile" => shell32::SHELL32_SHEncryptFile(emu),
        "SHELL32_SHFormatDriveAsync" => shell32::SHELL32_SHFormatDriveAsync(emu),
        "SHELL32_SHGetThreadUndoManager" => shell32::SHELL32_SHGetThreadUndoManager(emu),
        "SHELL32_SHGetUserNameW" => shell32::SHELL32_SHGetUserNameW(emu),
        "SHELL32_SHIsVirtualDevice" => shell32::SHELL32_SHIsVirtualDevice(emu),
        "SHELL32_SHLogILFromFSIL" => shell32::SHELL32_SHLogILFromFSIL(emu),
        "SHELL32_SimpleRatingToFilterCondition" => {
            shell32::SHELL32_SimpleRatingToFilterCondition(emu)
        }
        "SHELL32_SuspendUndo" => shell32::SHELL32_SuspendUndo(emu),
        "SHEmptyRecycleBinA" => shell32::SHEmptyRecycleBinA(emu),
        "SHEmptyRecycleBinW" => shell32::SHEmptyRecycleBinW(emu),
        "SHEnableServiceObject" => shell32::SHEnableServiceObject(emu),
        "SHFileOperation" => shell32::SHFileOperation(emu),
        "SHFileOperationA" => shell32::SHFileOperationA(emu),
        "SHFileOperationW" => shell32::SHFileOperationW(emu),
        "SHFindFiles" => shell32::SHFindFiles(emu),
        "SHFlushSFCache" => shell32::SHFlushSFCache(emu),
        "SHFormatDrive" => shell32::SHFormatDrive(emu),
        "SHFreeNameMappings" => shell32::SHFreeNameMappings(emu),
        "SHGetDiskFreeSpaceA" => shell32::SHGetDiskFreeSpaceA(emu),
        "SHGetDiskFreeSpaceExA" => shell32::SHGetDiskFreeSpaceExA(emu),
        "SHGetDiskFreeSpaceExW" => shell32::SHGetDiskFreeSpaceExW(emu),
        "SHGetDriveMedia" => shell32::SHGetDriveMedia(emu),
        "SHGetFileInfo" => shell32::SHGetFileInfo(emu),
        "SHGetFileInfoA" => shell32::SHGetFileInfoA(emu),
        "SHGetFileInfoW" => shell32::SHGetFileInfoW(emu),
        "SHGetFolderLocation" => shell32::SHGetFolderLocation(emu),
        "SHGetFolderPathA" => shell32::SHGetFolderPathA(emu),
        "SHGetFolderPathAndSubDirA" => shell32::SHGetFolderPathAndSubDirA(emu),
        "SHGetFolderPathAndSubDirW" => shell32::SHGetFolderPathAndSubDirW(emu),
        "SHGetFolderPathEx" => shell32::SHGetFolderPathEx(emu),
        "SHGetFolderPathW" => shell32::SHGetFolderPathW(emu),
        "SHGetKnownFolderPath" => shell32::SHGetKnownFolderPath(emu),
        "SHGetLocalizedName" => shell32::SHGetLocalizedName(emu),
        "SHGetNewLinkInfo" => shell32::SHGetNewLinkInfo(emu),
        "SHGetNewLinkInfoA" => shell32::SHGetNewLinkInfoA(emu),
        "SHGetNewLinkInfoW" => shell32::SHGetNewLinkInfoW(emu),
        "SHGetSetFolderCustomSettings" => shell32::SHGetSetFolderCustomSettings(emu),
        "SHGetSetSettings" => shell32::SHGetSetSettings(emu),
        "SHGetSettings" => shell32::SHGetSettings(emu),
        "SHGetSpecialFolderLocation" => shell32::SHGetSpecialFolderLocation(emu),
        "SHGetSpecialFolderPathA" => shell32::SHGetSpecialFolderPathA(emu),
        "SHGetSpecialFolderPathW" => shell32::SHGetSpecialFolderPathW(emu),
        "SHInvokePrinterCommandA" => shell32::SHInvokePrinterCommandA(emu),
        "SHInvokePrinterCommandW" => shell32::SHInvokePrinterCommandW(emu),
        "SHIsFileAvailableOffline" => shell32::SHIsFileAvailableOffline(emu),
        "SHLimitInputEdit" => shell32::SHLimitInputEdit(emu),
        "SHLoadInProc" => shell32::SHLoadInProc(emu),
        "SHOpenFolderAndSelectItems" => shell32::SHOpenFolderAndSelectItems(emu),
        "SHParseDisplayName" => shell32::SHParseDisplayName(emu),
        "SHPathPrepareForWriteA" => shell32::SHPathPrepareForWriteA(emu),
        "SHPathPrepareForWriteW" => shell32::SHPathPrepareForWriteW(emu),
        "SHQueryRecycleBinA" => shell32::SHQueryRecycleBinA(emu),
        "SHQueryRecycleBinW" => shell32::SHQueryRecycleBinW(emu),
        "SHQueryUserNotificationState" => shell32::SHQueryUserNotificationState(emu),
        "SHRemoveLocalizedName" => shell32::SHRemoveLocalizedName(emu),
        "SHResolveLibrary" => shell32::SHResolveLibrary(emu),
        "SHSetFolderPathA" => shell32::SHSetFolderPathA(emu),
        "SHSetFolderPathW" => shell32::SHSetFolderPathW(emu),
        "SHSetKnownFolderPath" => shell32::SHSetKnownFolderPath(emu),
        "SHSetLocalizedName" => shell32::SHSetLocalizedName(emu),
        "SHTestTokenMembership" => shell32::SHTestTokenMembership(emu),
        "SHValidateUNC" => shell32::SHValidateUNC(emu),
        "SheChangeDirA" => shell32::SheChangeDirA(emu),
        "SheChangeDirExW" => shell32::SheChangeDirExW(emu),
        "SheGetDirA" => shell32::SheGetDirA(emu),
        "SheSetCurDrive" => shell32::SheSetCurDrive(emu),
        "ShellAboutA" => shell32::ShellAboutA(emu),
        "ShellAboutW" => shell32::ShellAboutW(emu),
        "ShellMessageBoxA" => shell32::ShellMessageBoxA(emu),
        "ShellMessageBoxW" => shell32::ShellMessageBoxW(emu),
        "SignalFileOpen" => shell32::SignalFileOpen(emu),
        "StgMakeUniqueName" => shell32::StgMakeUniqueName(emu),
        "StrChrA" => shell32::StrChrA(emu),
        "StrChrIA" => shell32::StrChrIA(emu),
        "StrChrIW" => shell32::StrChrIW(emu),
        "StrChrW" => shell32::StrChrW(emu),
        "StrCmpNA" => shell32::StrCmpNA(emu),
        "StrCmpNIA" => shell32::StrCmpNIA(emu),
        "StrCmpNIW" => shell32::StrCmpNIW(emu),
        "StrCmpNW" => shell32::StrCmpNW(emu),
        "StrNCmpA" => shell32::StrNCmpA(emu),
        "StrNCmpIA" => shell32::StrNCmpIA(emu),
        "StrNCmpIW" => shell32::StrNCmpIW(emu),
        "StrNCmpW" => shell32::StrNCmpW(emu),
        "StrRChrA" => shell32::StrRChrA(emu),
        "StrRChrIA" => shell32::StrRChrIA(emu),
        "StrRChrIW" => shell32::StrRChrIW(emu),
        "StrRChrW" => shell32::StrRChrW(emu),
        "StrRStrA" => shell32::StrRStrA(emu),
        "StrRStrIA" => shell32::StrRStrIA(emu),
        "StrRStrIW" => shell32::StrRStrIW(emu),
        "StrRStrW" => shell32::StrRStrW(emu),
        "StrStrA" => shell32::StrStrA(emu),
        "StrStrIA" => shell32::StrStrIA(emu),
        "StrStrIW" => shell32::StrStrIW(emu),
        "StrStrW" => shell32::StrStrW(emu),
        "Win32DeleteFile" => shell32::Win32DeleteFile(emu),
        "WriteCabinetState" => shell32::WriteCabinetState(emu),
        _ => return false,
    }
    true
}
