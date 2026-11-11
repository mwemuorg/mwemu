use crate::emu::Emu;
use crate::winapi::winapi64::shell32;

#[path = "../str_str.rs"]
mod str_str;
pub use str_str::*;

#[path = "../str_r_str.rs"]
mod str_r_str;
pub use str_r_str::*;

#[path = "../str_r_chr.rs"]
mod str_r_chr;
pub use str_r_chr::*;

#[path = "../str_n_cmp.rs"]
mod str_n_cmp;
pub use str_n_cmp::*;

#[path = "../str_chr.rs"]
mod str_chr;
pub use str_chr::*;

#[path = "../shell_message_box.rs"]
mod shell_message_box;
pub use shell_message_box::*;

#[path = "../shell_about.rs"]
mod shell_about;
pub use shell_about::*;

#[path = "../sh_set_folder_path.rs"]
mod sh_set_folder_path;
pub use sh_set_folder_path::*;

#[path = "../sh_query_recycle_bin.rs"]
mod sh_query_recycle_bin;
pub use sh_query_recycle_bin::*;

#[path = "../sh_path_prepare_for_write.rs"]
mod sh_path_prepare_for_write;
pub use sh_path_prepare_for_write::*;

#[path = "../sh_invoke_printer_command.rs"]
mod sh_invoke_printer_command;
pub use sh_invoke_printer_command::*;

#[path = "../sh_get_special_folder_path.rs"]
mod sh_get_special_folder_path;
pub use sh_get_special_folder_path::*;

#[path = "../sh_get_folder_path_and_sub_dir.rs"]
mod sh_get_folder_path_and_sub_dir;
pub use sh_get_folder_path_and_sub_dir::*;

#[path = "../sh_get_folder_path.rs"]
mod sh_get_folder_path;
pub use sh_get_folder_path::*;

#[path = "../sh_get_disk_free_space_ex.rs"]
mod sh_get_disk_free_space_ex;
pub use sh_get_disk_free_space_ex::*;

#[path = "../sh_empty_recycle_bin.rs"]
mod sh_empty_recycle_bin;
pub use sh_empty_recycle_bin::*;

#[path = "../sh_create_directory_ex.rs"]
mod sh_create_directory_ex;
pub use sh_create_directory_ex::*;

#[path = "../path_is_slow.rs"]
mod path_is_slow;
pub use path_is_slow::*;

#[path = "../call_file_copy_hook.rs"]
mod call_file_copy_hook;
pub use call_file_copy_hook::CallFileCopyHook;

#[path = "../check_escapes_w.rs"]
mod check_escapes_w;
pub use check_escapes_w::CheckEscapesW;

#[path = "../drive_type.rs"]
mod drive_type;
pub use drive_type::DriveType;

#[path = "../init_network_address_control.rs"]
mod init_network_address_control;
pub use init_network_address_control::InitNetworkAddressControl;

#[path = "../is_lfn_drive.rs"]
mod is_lfn_drive;
pub use is_lfn_drive::IsLFNDrive;

pub use is_lfn_drive::IsLFNDriveA;

pub use is_lfn_drive::IsLFNDriveW;

#[path = "../is_net_drive.rs"]
mod is_net_drive;
pub use is_net_drive::IsNetDrive;

#[path = "../path_cleanup_spec.rs"]
mod path_cleanup_spec;
pub use path_cleanup_spec::PathCleanupSpec;

#[path = "../path_get_short_path.rs"]
mod path_get_short_path;
pub use path_get_short_path::PathGetShortPath;

#[path = "../path_is_exe.rs"]
mod path_is_exe;
pub use path_is_exe::PathIsExe;

#[path = "../path_make_unique_name.rs"]
mod path_make_unique_name;
pub use path_make_unique_name::PathMakeUniqueName;

#[path = "../path_qualify.rs"]
mod path_qualify;
pub use path_qualify::PathQualify;

#[path = "../path_resolve.rs"]
mod path_resolve;
pub use path_resolve::PathResolve;

#[path = "../path_yet_another_make_unique_name.rs"]
mod path_yet_another_make_unique_name;
pub use path_yet_another_make_unique_name::PathYetAnotherMakeUniqueName;

#[path = "../read_cabinet_state.rs"]
mod read_cabinet_state;
pub use read_cabinet_state::ReadCabinetState;

#[path = "../real_drive_type.rs"]
mod real_drive_type;
pub use real_drive_type::RealDriveType;

#[path = "../sh_add_to_recent_docs.rs"]
mod sh_add_to_recent_docs;
pub use sh_add_to_recent_docs::SHAddToRecentDocs;

#[path = "../sh_change_notification_lock.rs"]
mod sh_change_notification_lock;
pub use sh_change_notification_lock::SHChangeNotification_Lock;

#[path = "../sh_change_notification_unlock.rs"]
mod sh_change_notification_unlock;
pub use sh_change_notification_unlock::SHChangeNotification_Unlock;

#[path = "../sh_change_notify.rs"]
mod sh_change_notify;
pub use sh_change_notify::SHChangeNotify;

#[path = "../sh_change_notify_deregister.rs"]
mod sh_change_notify_deregister;
pub use sh_change_notify_deregister::SHChangeNotifyDeregister;

#[path = "../sh_change_notify_register.rs"]
mod sh_change_notify_register;
pub use sh_change_notify_register::SHChangeNotifyRegister;

#[path = "../sh_change_notify_register_thread.rs"]
mod sh_change_notify_register_thread;
pub use sh_change_notify_register_thread::SHChangeNotifyRegisterThread;

#[path = "../sh_change_notify_suspend_resume.rs"]
mod sh_change_notify_suspend_resume;
pub use sh_change_notify_suspend_resume::SHChangeNotifySuspendResume;

#[path = "../sh_create_category_enum.rs"]
mod sh_create_category_enum;
pub use sh_create_category_enum::SHCreateCategoryEnum;

#[path = "../sh_create_directory.rs"]
mod sh_create_directory;
pub use sh_create_directory::SHCreateDirectory;

#[path = "../sh_create_std_enum_fmt_etc.rs"]
mod sh_create_std_enum_fmt_etc;
pub use sh_create_std_enum_fmt_etc::SHCreateStdEnumFmtEtc;

#[path = "../shell32_are_all_items_available.rs"]
mod shell32_are_all_items_available;
pub use shell32_are_all_items_available::SHELL32_AreAllItemsAvailable;

#[path = "../shell32_c_common_places_folder_create_instance.rs"]
mod shell32_c_common_places_folder_create_instance;
pub use shell32_c_common_places_folder_create_instance::SHELL32_CCommonPlacesFolder_CreateInstance;

#[path = "../shell32_cd_burn_close_session.rs"]
mod shell32_cd_burn_close_session;
pub use shell32_cd_burn_close_session::SHELL32_CDBurn_CloseSession;

#[path = "../shell32_cd_burn_drive_supported_for_data_burn.rs"]
mod shell32_cd_burn_drive_supported_for_data_burn;
pub use shell32_cd_burn_drive_supported_for_data_burn::SHELL32_CDBurn_DriveSupportedForDataBurn;

#[path = "../shell32_cd_burn_erase.rs"]
mod shell32_cd_burn_erase;
pub use shell32_cd_burn_erase::SHELL32_CDBurn_Erase;

#[path = "../shell32_cd_burn_get_cd_info.rs"]
mod shell32_cd_burn_get_cd_info;
pub use shell32_cd_burn_get_cd_info::SHELL32_CDBurn_GetCDInfo;

#[path = "../shell32_cd_burn_get_live_fs_disc_info.rs"]
mod shell32_cd_burn_get_live_fs_disc_info;
pub use shell32_cd_burn_get_live_fs_disc_info::SHELL32_CDBurn_GetLiveFSDiscInfo;

#[path = "../shell32_cd_burn_get_staging_path_or_normal_path.rs"]
mod shell32_cd_burn_get_staging_path_or_normal_path;
pub use shell32_cd_burn_get_staging_path_or_normal_path::SHELL32_CDBurn_GetStagingPathOrNormalPath;

#[path = "../shell32_cd_burn_get_task_info.rs"]
mod shell32_cd_burn_get_task_info;
pub use shell32_cd_burn_get_task_info::SHELL32_CDBurn_GetTaskInfo;

#[path = "../shell32_cd_burn_is_blank_disc.rs"]
mod shell32_cd_burn_is_blank_disc;
pub use shell32_cd_burn_is_blank_disc::SHELL32_CDBurn_IsBlankDisc;

#[path = "../shell32_cd_burn_is_blank_disc2.rs"]
mod shell32_cd_burn_is_blank_disc2;
pub use shell32_cd_burn_is_blank_disc2::SHELL32_CDBurn_IsBlankDisc2;

#[path = "../shell32_cd_burn_is_live_fs.rs"]
mod shell32_cd_burn_is_live_fs;
pub use shell32_cd_burn_is_live_fs::SHELL32_CDBurn_IsLiveFS;

#[path = "../shell32_cd_burn_on_device_change.rs"]
mod shell32_cd_burn_on_device_change;
pub use shell32_cd_burn_on_device_change::SHELL32_CDBurn_OnDeviceChange;

#[path = "../shell32_cd_burn_on_eject.rs"]
mod shell32_cd_burn_on_eject;
pub use shell32_cd_burn_on_eject::SHELL32_CDBurn_OnEject;

#[path = "../shell32_cd_burn_on_media_change.rs"]
mod shell32_cd_burn_on_media_change;
pub use shell32_cd_burn_on_media_change::SHELL32_CDBurn_OnMediaChange;

#[path = "../shell32_c_drives_create_sfvcb.rs"]
mod shell32_c_drives_create_sfvcb;
pub use shell32_c_drives_create_sfvcb::SHELL32_CDrives_CreateSFVCB;

#[path = "../shell32_cfs_folder_callback_create.rs"]
mod shell32_cfs_folder_callback_create;
pub use shell32_cfs_folder_callback_create::SHELL32_CFSFolderCallback_Create;

#[path = "../shell32_cpl_category_id_array_from_variant.rs"]
mod shell32_cpl_category_id_array_from_variant;
pub use shell32_cpl_category_id_array_from_variant::SHELL32_CPL_CategoryIdArrayFromVariant;

#[path = "../shell32_cpl_is_legacy_canonical_name_listed_under_key.rs"]
mod shell32_cpl_is_legacy_canonical_name_listed_under_key;
pub use shell32_cpl_is_legacy_canonical_name_listed_under_key::SHELL32_CPL_IsLegacyCanonicalNameListedUnderKey;

#[path = "../shell32_cpl_modify_wow_display_name.rs"]
mod shell32_cpl_modify_wow_display_name;
pub use shell32_cpl_modify_wow_display_name::SHELL32_CPL_ModifyWowDisplayName;

#[path = "../shell32_c_transfer_confirmation_create_instance.rs"]
mod shell32_c_transfer_confirmation_create_instance;
pub use shell32_c_transfer_confirmation_create_instance::SHELL32_CTransferConfirmation_CreateInstance;

#[path = "../shell32_call_file_copy_hooks.rs"]
mod shell32_call_file_copy_hooks;
pub use shell32_call_file_copy_hooks::SHELL32_CallFileCopyHooks;

#[path = "../shell32_copy_secondary_tiles.rs"]
mod shell32_copy_secondary_tiles;
pub use shell32_copy_secondary_tiles::SHELL32_CopySecondaryTiles;

#[path = "../shell32_create_confirmation_interrupt.rs"]
mod shell32_create_confirmation_interrupt;
pub use shell32_create_confirmation_interrupt::SHELL32_CreateConfirmationInterrupt;

#[path = "../shell32_create_conflict_interrupt.rs"]
mod shell32_create_conflict_interrupt;
pub use shell32_create_conflict_interrupt::SHELL32_CreateConflictInterrupt;

#[path = "../shell32_create_default_operation_data_provider.rs"]
mod shell32_create_default_operation_data_provider;
pub use shell32_create_default_operation_data_provider::SHELL32_CreateDefaultOperationDataProvider;

#[path = "../shell32_create_link_info_w.rs"]
mod shell32_create_link_info_w;
pub use shell32_create_link_info_w::SHELL32_CreateLinkInfoW;

#[path = "../shell32_create_qos_recorder.rs"]
mod shell32_create_qos_recorder;
pub use shell32_create_qos_recorder::SHELL32_CreateQosRecorder;

#[path = "../shell32_create_share_point_view.rs"]
mod shell32_create_share_point_view;
pub use shell32_create_share_point_view::SHELL32_CreateSharePointView;

#[path = "../shell32_destroy_link_info.rs"]
mod shell32_destroy_link_info;
pub use shell32_destroy_link_info::SHELL32_DestroyLinkInfo;

#[path = "../shell32_encrypt_directory.rs"]
mod shell32_encrypt_directory;
pub use shell32_encrypt_directory::SHELL32_EncryptDirectory;

#[path = "../shell32_encrypted_file_key_info.rs"]
mod shell32_encrypted_file_key_info;
pub use shell32_encrypted_file_key_info::SHELL32_EncryptedFileKeyInfo;

#[path = "../shell32_enum_common_tasks.rs"]
mod shell32_enum_common_tasks;
pub use shell32_enum_common_tasks::SHELL32_EnumCommonTasks;

#[path = "../shell32_free_encrypted_file_key_info.rs"]
mod shell32_free_encrypted_file_key_info;
pub use shell32_free_encrypted_file_key_info::SHELL32_FreeEncryptedFileKeyInfo;

#[path = "../shell32_get_command_provider_for_folder_type.rs"]
mod shell32_get_command_provider_for_folder_type;
pub use shell32_get_command_provider_for_folder_type::SHELL32_GetCommandProviderForFolderType;

#[path = "../shell32_get_dpi_adjusted_logical_size.rs"]
mod shell32_get_dpi_adjusted_logical_size;
pub use shell32_get_dpi_adjusted_logical_size::SHELL32_GetDPIAdjustedLogicalSize;

#[path = "../shell32_get_disk_cleanup_path.rs"]
mod shell32_get_disk_cleanup_path;
pub use shell32_get_disk_cleanup_path::SHELL32_GetDiskCleanupPath;

#[path = "../shell32_get_link_info_data.rs"]
mod shell32_get_link_info_data;
pub use shell32_get_link_info_data::SHELL32_GetLinkInfoData;

#[path = "../shell32_get_rating_bucket.rs"]
mod shell32_get_rating_bucket;
pub use shell32_get_rating_bucket::SHELL32_GetRatingBucket;

#[path = "../shell32_get_sqmable_file_name.rs"]
mod shell32_get_sqmable_file_name;
pub use shell32_get_sqmable_file_name::SHELL32_GetSqmableFileName;

#[path = "../shell32_handle_unrecognized_file_system.rs"]
mod shell32_handle_unrecognized_file_system;
pub use shell32_handle_unrecognized_file_system::SHELL32_HandleUnrecognizedFileSystem;

#[path = "../shell32_is_get_keyboard_layout_present.rs"]
mod shell32_is_get_keyboard_layout_present;
pub use shell32_is_get_keyboard_layout_present::SHELL32_IsGetKeyboardLayoutPresent;

#[path = "../shell32_is_system_upgrade_in_progress.rs"]
mod shell32_is_system_upgrade_in_progress;
pub use shell32_is_system_upgrade_in_progress::SHELL32_IsSystemUpgradeInProgress;

#[path = "../shell32_is_valid_link_info.rs"]
mod shell32_is_valid_link_info;
pub use shell32_is_valid_link_info::SHELL32_IsValidLinkInfo;

#[path = "../shell32_legacy_enum_special_tasks_by_type.rs"]
mod shell32_legacy_enum_special_tasks_by_type;
pub use shell32_legacy_enum_special_tasks_by_type::SHELL32_LegacyEnumSpecialTasksByType;

#[path = "../shell32_legacy_enum_tasks.rs"]
mod shell32_legacy_enum_tasks;
pub use shell32_legacy_enum_tasks::SHELL32_LegacyEnumTasks;

#[path = "../shell32_normalize_rating.rs"]
mod shell32_normalize_rating;
pub use shell32_normalize_rating::SHELL32_NormalizeRating;

#[path = "../shell32_notify_link_tracking_service_of_move.rs"]
mod shell32_notify_link_tracking_service_of_move;
pub use shell32_notify_link_tracking_service_of_move::SHELL32_NotifyLinkTrackingServiceOfMove;

#[path = "../shell32_printers_create_bind_info.rs"]
mod shell32_printers_create_bind_info;
pub use shell32_printers_create_bind_info::SHELL32_Printers_CreateBindInfo;

#[path = "../shell32_resolve_link_info_w.rs"]
mod shell32_resolve_link_info_w;
pub use shell32_resolve_link_info_w::SHELL32_ResolveLinkInfoW;

#[path = "../shell32_sh_create_by_value_operation_interrupt.rs"]
mod shell32_sh_create_by_value_operation_interrupt;
pub use shell32_sh_create_by_value_operation_interrupt::SHELL32_SHCreateByValueOperationInterrupt;

#[path = "../shell32_sh_create_local_server.rs"]
mod shell32_sh_create_local_server;
pub use shell32_sh_create_local_server::SHELL32_SHCreateLocalServer;

#[path = "../shell32_sh_duplicate_encryption_info_file.rs"]
mod shell32_sh_duplicate_encryption_info_file;
pub use shell32_sh_duplicate_encryption_info_file::SHELL32_SHDuplicateEncryptionInfoFile;

#[path = "../shell32_sh_encrypt_file.rs"]
mod shell32_sh_encrypt_file;
pub use shell32_sh_encrypt_file::SHELL32_SHEncryptFile;

#[path = "../shell32_sh_format_drive_async.rs"]
mod shell32_sh_format_drive_async;
pub use shell32_sh_format_drive_async::SHELL32_SHFormatDriveAsync;

#[path = "../shell32_sh_get_thread_undo_manager.rs"]
mod shell32_sh_get_thread_undo_manager;
pub use shell32_sh_get_thread_undo_manager::SHELL32_SHGetThreadUndoManager;

#[path = "../shell32_sh_get_user_name_w.rs"]
mod shell32_sh_get_user_name_w;
pub use shell32_sh_get_user_name_w::SHELL32_SHGetUserNameW;

#[path = "../shell32_sh_is_virtual_device.rs"]
mod shell32_sh_is_virtual_device;
pub use shell32_sh_is_virtual_device::SHELL32_SHIsVirtualDevice;

#[path = "../shell32_sh_log_il_from_fsil.rs"]
mod shell32_sh_log_il_from_fsil;
pub use shell32_sh_log_il_from_fsil::SHELL32_SHLogILFromFSIL;

#[path = "../shell32_simple_rating_to_filter_condition.rs"]
mod shell32_simple_rating_to_filter_condition;
pub use shell32_simple_rating_to_filter_condition::SHELL32_SimpleRatingToFilterCondition;

#[path = "../shell32_suspend_undo.rs"]
mod shell32_suspend_undo;
pub use shell32_suspend_undo::SHELL32_SuspendUndo;

#[path = "../sh_enable_service_object.rs"]
mod sh_enable_service_object;
pub use sh_enable_service_object::SHEnableServiceObject;

#[path = "../sh_file_operation.rs"]
mod sh_file_operation;
pub use sh_file_operation::SHFileOperation;

pub use sh_file_operation::SHFileOperationA;

pub use sh_file_operation::SHFileOperationW;

#[path = "../sh_find_files.rs"]
mod sh_find_files;
pub use sh_find_files::SHFindFiles;

#[path = "../sh_flush_sf_cache.rs"]
mod sh_flush_sf_cache;
pub use sh_flush_sf_cache::SHFlushSFCache;

#[path = "../sh_format_drive.rs"]
mod sh_format_drive;
pub use sh_format_drive::SHFormatDrive;

#[path = "../sh_free_name_mappings.rs"]
mod sh_free_name_mappings;
pub use sh_free_name_mappings::SHFreeNameMappings;

#[path = "../sh_get_disk_free_space_a.rs"]
mod sh_get_disk_free_space_a;
pub use sh_get_disk_free_space_a::SHGetDiskFreeSpaceA;

#[path = "../sh_get_drive_media.rs"]
mod sh_get_drive_media;
pub use sh_get_drive_media::SHGetDriveMedia;

#[path = "../sh_get_file_info.rs"]
mod sh_get_file_info;
pub use sh_get_file_info::SHGetFileInfo;

pub use sh_get_file_info::SHGetFileInfoA;

pub use sh_get_file_info::SHGetFileInfoW;

#[path = "../sh_get_folder_location.rs"]
mod sh_get_folder_location;
pub use sh_get_folder_location::SHGetFolderLocation;

#[path = "../sh_get_folder_path_ex.rs"]
mod sh_get_folder_path_ex;
pub use sh_get_folder_path_ex::SHGetFolderPathEx;

#[path = "../sh_get_known_folder_path.rs"]
mod sh_get_known_folder_path;
pub use sh_get_known_folder_path::SHGetKnownFolderPath;

#[path = "../sh_get_localized_name.rs"]
mod sh_get_localized_name;
pub use sh_get_localized_name::SHGetLocalizedName;

#[path = "../sh_get_new_link_info.rs"]
mod sh_get_new_link_info;
pub use sh_get_new_link_info::SHGetNewLinkInfo;

pub use sh_get_new_link_info::SHGetNewLinkInfoA;

pub use sh_get_new_link_info::SHGetNewLinkInfoW;

#[path = "../sh_get_set_folder_custom_settings.rs"]
mod sh_get_set_folder_custom_settings;
pub use sh_get_set_folder_custom_settings::SHGetSetFolderCustomSettings;

#[path = "../sh_get_set_settings.rs"]
mod sh_get_set_settings;
pub use sh_get_set_settings::SHGetSetSettings;

#[path = "../sh_get_settings.rs"]
mod sh_get_settings;
pub use sh_get_settings::SHGetSettings;

#[path = "../sh_get_special_folder_location.rs"]
mod sh_get_special_folder_location;
pub use sh_get_special_folder_location::SHGetSpecialFolderLocation;

#[path = "../sh_is_file_available_offline.rs"]
mod sh_is_file_available_offline;
pub use sh_is_file_available_offline::SHIsFileAvailableOffline;

#[path = "../sh_limit_input_edit.rs"]
mod sh_limit_input_edit;
pub use sh_limit_input_edit::SHLimitInputEdit;

#[path = "../sh_load_in_proc.rs"]
mod sh_load_in_proc;
pub use sh_load_in_proc::SHLoadInProc;

#[path = "../sh_open_folder_and_select_items.rs"]
mod sh_open_folder_and_select_items;
pub use sh_open_folder_and_select_items::SHOpenFolderAndSelectItems;

#[path = "../sh_parse_display_name.rs"]
mod sh_parse_display_name;
pub use sh_parse_display_name::SHParseDisplayName;

#[path = "../sh_query_user_notification_state.rs"]
mod sh_query_user_notification_state;
pub use sh_query_user_notification_state::SHQueryUserNotificationState;

#[path = "../sh_remove_localized_name.rs"]
mod sh_remove_localized_name;
pub use sh_remove_localized_name::SHRemoveLocalizedName;

#[path = "../sh_resolve_library.rs"]
mod sh_resolve_library;
pub use sh_resolve_library::SHResolveLibrary;

#[path = "../sh_set_known_folder_path.rs"]
mod sh_set_known_folder_path;
pub use sh_set_known_folder_path::SHSetKnownFolderPath;

#[path = "../sh_set_localized_name.rs"]
mod sh_set_localized_name;
pub use sh_set_localized_name::SHSetLocalizedName;

#[path = "../sh_test_token_membership.rs"]
mod sh_test_token_membership;
pub use sh_test_token_membership::SHTestTokenMembership;

#[path = "../sh_validate_unc.rs"]
mod sh_validate_unc;
pub use sh_validate_unc::SHValidateUNC;

#[path = "../she_change_dir_a.rs"]
mod she_change_dir_a;
pub use she_change_dir_a::SheChangeDirA;

#[path = "../she_change_dir_ex_w.rs"]
mod she_change_dir_ex_w;
pub use she_change_dir_ex_w::SheChangeDirExW;

#[path = "../she_get_dir_a.rs"]
mod she_get_dir_a;
pub use she_get_dir_a::SheGetDirA;

#[path = "../she_set_cur_drive.rs"]
mod she_set_cur_drive;
pub use she_set_cur_drive::SheSetCurDrive;

#[path = "../signal_file_open.rs"]
mod signal_file_open;
pub use signal_file_open::SignalFileOpen;

#[path = "../stg_make_unique_name.rs"]
mod stg_make_unique_name;
pub use stg_make_unique_name::StgMakeUniqueName;

#[path = "../str_chr_i.rs"]
mod str_chr_i;
pub use str_chr_i::StrChrIA;

pub use str_chr_i::StrChrIW;

#[path = "../str_cmp_n.rs"]
mod str_cmp_n;
pub use str_cmp_n::StrCmpNA;

#[path = "../str_cmp_ni.rs"]
mod str_cmp_ni;
pub use str_cmp_ni::StrCmpNIA;

pub use str_cmp_ni::StrCmpNIW;

pub use str_cmp_n::StrCmpNW;

#[path = "../str_n_cmp_i.rs"]
mod str_n_cmp_i;
pub use str_n_cmp_i::StrNCmpIA;

pub use str_n_cmp_i::StrNCmpIW;

#[path = "../str_r_chr_i.rs"]
mod str_r_chr_i;
pub use str_r_chr_i::StrRChrIA;

pub use str_r_chr_i::StrRChrIW;

#[path = "../str_r_str_i.rs"]
mod str_r_str_i;
pub use str_r_str_i::StrRStrIA;

pub use str_r_str_i::StrRStrIW;

#[path = "../str_str_i.rs"]
mod str_str_i;
pub use str_str_i::StrStrIA;

pub use str_str_i::StrStrIW;

#[path = "../win32_delete_file.rs"]
mod win32_delete_file;
pub use win32_delete_file::Win32DeleteFile;

#[path = "../write_cabinet_state.rs"]
mod write_cabinet_state;
pub use write_cabinet_state::WriteCabinetState;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "SHGetFolderPathW" => shell32::SHGetFolderPathW(emu),
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
