use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

#[path = "../application_recovery_finished.rs"]
mod application_recovery_finished;
pub use application_recovery_finished::*;

#[path = "../application_recovery_in_progress.rs"]
mod application_recovery_in_progress;
pub use application_recovery_in_progress::*;

#[path = "../base_check_appcompat_cache_ex_worker.rs"]
mod base_check_appcompat_cache_ex_worker;
pub use base_check_appcompat_cache_ex_worker::*;

#[path = "../base_check_appcompat_cache_worker.rs"]
mod base_check_appcompat_cache_worker;
pub use base_check_appcompat_cache_worker::*;

#[path = "../base_check_elevation.rs"]
mod base_check_elevation;
pub use base_check_elevation::*;

#[path = "../base_cleanup_appcompat_cache_support_worker.rs"]
mod base_cleanup_appcompat_cache_support_worker;
pub use base_cleanup_appcompat_cache_support_worker::*;

#[path = "../base_dll_initialize.rs"]
mod base_dll_initialize;
pub use base_dll_initialize::*;

#[path = "../base_dll_read_write_ini_file.rs"]
mod base_dll_read_write_ini_file;
pub use base_dll_read_write_ini_file::*;

#[path = "../base_exit_thread_pool_thread.rs"]
mod base_exit_thread_pool_thread;
pub use base_exit_thread_pool_thread::*;

#[path = "../base_flush_appcompat_cache_worker.rs"]
mod base_flush_appcompat_cache_worker;
pub use base_flush_appcompat_cache_worker::*;

#[path = "../base_free_app_compat_data_for_process_worker.rs"]
mod base_free_app_compat_data_for_process_worker;
pub use base_free_app_compat_data_for_process_worker::*;

#[path = "../base_generate_app_compat_data.rs"]
mod base_generate_app_compat_data;
pub use base_generate_app_compat_data::*;

#[path = "../base_is_dos_application.rs"]
mod base_is_dos_application;
pub use base_is_dos_application::*;

#[path = "../base_read_app_compat_data_for_process_worker.rs"]
mod base_read_app_compat_data_for_process_worker;
pub use base_read_app_compat_data_for_process_worker::*;

#[path = "../base_set_last_nt_error.rs"]
mod base_set_last_nt_error;
pub use base_set_last_nt_error::*;

#[path = "../base_update_appcompat_cache_worker.rs"]
mod base_update_appcompat_cache_worker;
pub use base_update_appcompat_cache_worker::*;

#[path = "../base_verify_unicode_string.rs"]
mod base_verify_unicode_string;
pub use base_verify_unicode_string::*;

#[path = "../base_write_error_elevation_required_event.rs"]
mod base_write_error_elevation_required_event;
pub use base_write_error_elevation_required_event::*;

#[path = "../basep8_bit_string_to_dynamic_unicode_string.rs"]
mod basep8_bit_string_to_dynamic_unicode_string;
pub use basep8_bit_string_to_dynamic_unicode_string::*;

#[path = "../basep_accumulate_io_rate_control_information_buffer_size.rs"]
mod basep_accumulate_io_rate_control_information_buffer_size;
pub use basep_accumulate_io_rate_control_information_buffer_size::*;

#[path = "../basep_allocate_activation_context_activation_block.rs"]
mod basep_allocate_activation_context_activation_block;
pub use basep_allocate_activation_context_activation_block::*;

#[path = "../basep_ansi_string_to_dynamic_unicode_string.rs"]
mod basep_ansi_string_to_dynamic_unicode_string;
pub use basep_ansi_string_to_dynamic_unicode_string::*;

#[path = "../basep_app_container_environment_extension.rs"]
mod basep_app_container_environment_extension;
pub use basep_app_container_environment_extension::*;

#[path = "../basep_app_x_extension.rs"]
mod basep_app_x_extension;
pub use basep_app_x_extension::*;

#[path = "../basep_check_app_compat.rs"]
mod basep_check_app_compat;
pub use basep_check_app_compat::*;

#[path = "../basep_check_web_blade_hashes.rs"]
mod basep_check_web_blade_hashes;
pub use basep_check_web_blade_hashes::*;

#[path = "../basep_check_win_safer_restrictions.rs"]
mod basep_check_win_safer_restrictions;
pub use basep_check_win_safer_restrictions::*;

#[path = "../basep_configure_app_cert_dlls.rs"]
mod basep_configure_app_cert_dlls;
pub use basep_configure_app_cert_dlls::*;

#[path = "../basep_construct_sxs_create_process_message.rs"]
mod basep_construct_sxs_create_process_message;
pub use basep_construct_sxs_create_process_message::*;

#[path = "../basep_copy_encryption.rs"]
mod basep_copy_encryption;
pub use basep_copy_encryption::*;

#[path = "../basep_free_activation_context_activation_block.rs"]
mod basep_free_activation_context_activation_block;
pub use basep_free_activation_context_activation_block::*;

#[path = "../basep_free_app_compat_data.rs"]
mod basep_free_app_compat_data;
pub use basep_free_app_compat_data::*;

#[path = "../basep_get_computer_name_from_nt_path.rs"]
mod basep_get_computer_name_from_nt_path;
pub use basep_get_computer_name_from_nt_path::*;

#[path = "../basep_get_exe_arch_type.rs"]
mod basep_get_exe_arch_type;
pub use basep_get_exe_arch_type::*;

#[path = "../basep_get_mini_version_for_create.rs"]
mod basep_get_mini_version_for_create;
pub use basep_get_mini_version_for_create::*;

#[path = "../basep_init_app_compat_data.rs"]
mod basep_init_app_compat_data;
pub use basep_init_app_compat_data::*;

#[path = "../basep_initialize_apphelp_globals.rs"]
mod basep_initialize_apphelp_globals;
pub use basep_initialize_apphelp_globals::*;

#[path = "../basep_initialize_termsrv_fpns.rs"]
mod basep_initialize_termsrv_fpns;
pub use basep_initialize_termsrv_fpns::*;

#[path = "../basep_is_process_allowed.rs"]
mod basep_is_process_allowed;
pub use basep_is_process_allowed::*;

#[path = "../basep_is_test_signing_enabled.rs"]
mod basep_is_test_signing_enabled;
pub use basep_is_test_signing_enabled::*;

#[path = "../basep_notify_load_string_resource.rs"]
mod basep_notify_load_string_resource;
pub use basep_notify_load_string_resource::*;

#[path = "../basep_post_success_app_x_extension.rs"]
mod basep_post_success_app_x_extension;
pub use basep_post_success_app_x_extension::*;

#[path = "../basep_process_invalid_image.rs"]
mod basep_process_invalid_image;
pub use basep_process_invalid_image::*;

#[path = "../basep_query_app_compat.rs"]
mod basep_query_app_compat;
pub use basep_query_app_compat::*;

#[path = "../basep_regenerate_act_ctx_with_language.rs"]
mod basep_regenerate_act_ctx_with_language;
pub use basep_regenerate_act_ctx_with_language::*;

#[path = "../basep_release_sxs_create_process_utility_struct.rs"]
mod basep_release_sxs_create_process_utility_struct;
pub use basep_release_sxs_create_process_utility_struct::*;

#[path = "../basep_report_fault.rs"]
mod basep_report_fault;
pub use basep_report_fault::*;

#[path = "../basep_set_file_encryption_compression.rs"]
mod basep_set_file_encryption_compression;
pub use basep_set_file_encryption_compression::*;

#[path = "../basep_sxs_create_streams.rs"]
mod basep_sxs_create_streams;
pub use basep_sxs_create_streams::*;

#[path = "../basep_tp_io_callback.rs"]
mod basep_tp_io_callback;
pub use basep_tp_io_callback::*;

#[path = "../basep_tp_io_cleanup_callback.rs"]
mod basep_tp_io_cleanup_callback;
pub use basep_tp_io_cleanup_callback::*;

#[path = "../basep_tp_io_finalization_callback.rs"]
mod basep_tp_io_finalization_callback;
pub use basep_tp_io_finalization_callback::*;

#[path = "../compat_cache_lookup_exe.rs"]
mod compat_cache_lookup_exe;
pub use compat_cache_lookup_exe::*;

#[path = "../get_compat_flags.rs"]
mod get_compat_flags;
pub use get_compat_flags::*;

#[path = "../is_terminal_server_compatible.rs"]
mod is_terminal_server_compatible;
pub use is_terminal_server_compatible::*;

#[path = "../quirk_get_data_worker.rs"]
mod quirk_get_data_worker;
pub use quirk_get_data_worker::*;

#[path = "../quirk_is_enabled2_worker.rs"]
mod quirk_is_enabled2_worker;
pub use quirk_is_enabled2_worker::*;

#[path = "../quirk_is_enabled_for_package2_worker.rs"]
mod quirk_is_enabled_for_package2_worker;
pub use quirk_is_enabled_for_package2_worker::*;

#[path = "../quirk_is_enabled_for_package3_worker.rs"]
mod quirk_is_enabled_for_package3_worker;
pub use quirk_is_enabled_for_package3_worker::*;

#[path = "../quirk_is_enabled_for_package4_worker.rs"]
mod quirk_is_enabled_for_package4_worker;
pub use quirk_is_enabled_for_package4_worker::*;

#[path = "../quirk_is_enabled_for_package_worker.rs"]
mod quirk_is_enabled_for_package_worker;
pub use quirk_is_enabled_for_package_worker::*;

#[path = "../quirk_is_enabled_for_process_worker.rs"]
mod quirk_is_enabled_for_process_worker;
pub use quirk_is_enabled_for_process_worker::*;

#[path = "../quirk_is_enabled_worker.rs"]
mod quirk_is_enabled_worker;
pub use quirk_is_enabled_worker::*;

#[path = "../sdbp_check_from_version.rs"]
mod sdbp_check_from_version;
pub use sdbp_check_from_version::*;

#[path = "../sdbp_check_matching_device.rs"]
mod sdbp_check_matching_device;
pub use sdbp_check_matching_device::*;

#[path = "../sdbp_check_matching_dir.rs"]
mod sdbp_check_matching_dir;
pub use sdbp_check_matching_dir::*;

#[path = "../sdbp_check_matching_files.rs"]
mod sdbp_check_matching_files;
pub use sdbp_check_matching_files::*;

#[path = "../sdbp_check_matching_registry.rs"]
mod sdbp_check_matching_registry;
pub use sdbp_check_matching_registry::*;

#[path = "../sdbp_check_matching_text.rs"]
mod sdbp_check_matching_text;
pub use sdbp_check_matching_text::*;

#[path = "../sdbp_check_matching_wildcard_files.rs"]
mod sdbp_check_matching_wildcard_files;
pub use sdbp_check_matching_wildcard_files::*;

#[path = "../sdbp_check_matching_wildcard_registry.rs"]
mod sdbp_check_matching_wildcard_registry;
pub use sdbp_check_matching_wildcard_registry::*;

#[path = "../sdbp_check_os_kind.rs"]
mod sdbp_check_os_kind;
pub use sdbp_check_os_kind::*;

#[path = "../sdbp_check_package_attributes.rs"]
mod sdbp_check_package_attributes;
pub use sdbp_check_package_attributes::*;

#[path = "../sdbp_check_runtime_platform.rs"]
mod sdbp_check_runtime_platform;
pub use sdbp_check_runtime_platform::*;

#[path = "../sdbp_check_upto_version.rs"]
mod sdbp_check_upto_version;
pub use sdbp_check_upto_version::*;

#[path = "../sdbp_check_version.rs"]
mod sdbp_check_version;
pub use sdbp_check_version::*;

#[path = "../sdbp_get_path_app_patch.rs"]
mod sdbp_get_path_app_patch;
pub use sdbp_get_path_app_patch::*;

#[path = "../sdbp_get_path_app_patch_pre_rs3.rs"]
mod sdbp_get_path_app_patch_pre_rs3;
pub use sdbp_get_path_app_patch_pre_rs3::*;

#[path = "../sdbp_get_path_appraiser.rs"]
mod sdbp_get_path_appraiser;
pub use sdbp_get_path_appraiser::*;

#[path = "../sdbp_get_path_custom_sdb.rs"]
mod sdbp_get_path_custom_sdb;
pub use sdbp_get_path_custom_sdb::*;

#[path = "../sdbp_get_path_custom_sdb_pre_rs3.rs"]
mod sdbp_get_path_custom_sdb_pre_rs3;
pub use sdbp_get_path_custom_sdb_pre_rs3::*;

#[path = "../sdbp_get_path_system.rs"]
mod sdbp_get_path_system;
pub use sdbp_get_path_system::*;

#[path = "../wer_get_flags_worker.rs"]
mod wer_get_flags_worker;
pub use wer_get_flags_worker::*;

#[path = "../wer_register_file_worker.rs"]
mod wer_register_file_worker;
pub use wer_register_file_worker::*;

#[path = "../wer_set_flags.rs"]
mod wer_set_flags;
pub use wer_set_flags::*;

#[path = "../wer_unregister_file_worker.rs"]
mod wer_unregister_file_worker;
pub use wer_unregister_file_worker::*;

#[path = "../wer_unregister_memory_block_worker.rs"]
mod wer_unregister_memory_block_worker;
pub use wer_unregister_memory_block_worker::*;

#[path = "../werp_acquire_peb_lock.rs"]
mod werp_acquire_peb_lock;
pub use werp_acquire_peb_lock::*;

#[path = "../werp_check_ok_to_register.rs"]
mod werp_check_ok_to_register;
pub use werp_check_ok_to_register::*;

#[path = "../werp_get_debugger.rs"]
mod werp_get_debugger;
pub use werp_get_debugger::*;

#[path = "../werp_init_peb_store.rs"]
mod werp_init_peb_store;
pub use werp_init_peb_store::*;

#[path = "../werp_initiate_remote_recovery.rs"]
mod werp_initiate_remote_recovery;
pub use werp_initiate_remote_recovery::*;

#[path = "../werp_launch_ae_debug.rs"]
mod werp_launch_ae_debug;
pub use werp_launch_ae_debug::*;

#[path = "../werp_map_protection_level.rs"]
mod werp_map_protection_level;
pub use werp_map_protection_level::*;

#[path = "../werp_notify_use_string_resource_worker.rs"]
mod werp_notify_use_string_resource_worker;
pub use werp_notify_use_string_resource_worker::*;

#[path = "../werp_recovery_invoked_remotely.rs"]
mod werp_recovery_invoked_remotely;
pub use werp_recovery_invoked_remotely::*;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "CompareStringW" => kernel32::CompareStringW(emu),
        "ApplicationRecoveryFinished" => kernel32::ApplicationRecoveryFinished(emu),
        "ApplicationRecoveryInProgress" => kernel32::ApplicationRecoveryInProgress(emu),
        "BaseCheckAppcompatCacheExWorker" => kernel32::BaseCheckAppcompatCacheExWorker(emu),
        "BaseCheckAppcompatCacheWorker" => kernel32::BaseCheckAppcompatCacheWorker(emu),
        "BaseCleanupAppcompatCacheSupportWorker" => {
            kernel32::BaseCleanupAppcompatCacheSupportWorker(emu)
        }
        "BaseFlushAppcompatCacheWorker" => kernel32::BaseFlushAppcompatCacheWorker(emu),
        "BaseFreeAppCompatDataForProcessWorker" => {
            kernel32::BaseFreeAppCompatDataForProcessWorker(emu)
        }
        "BaseGenerateAppCompatData" => kernel32::BaseGenerateAppCompatData(emu),
        "BaseIsDosApplication" => kernel32::BaseIsDosApplication(emu),
        "BaseReadAppCompatDataForProcessWorker" => {
            kernel32::BaseReadAppCompatDataForProcessWorker(emu)
        }
        "BaseUpdateAppcompatCacheWorker" => kernel32::BaseUpdateAppcompatCacheWorker(emu),
        "BasepAppContainerEnvironmentExtension" => {
            kernel32::BasepAppContainerEnvironmentExtension(emu)
        }
        "BasepAppXExtension" => kernel32::BasepAppXExtension(emu),
        "BasepCheckAppCompat" => kernel32::BasepCheckAppCompat(emu),
        "BasepConfigureAppCertDlls" => kernel32::BasepConfigureAppCertDlls(emu),
        "BasepFreeAppCompatData" => kernel32::BasepFreeAppCompatData(emu),
        "BasepInitAppCompatData" => kernel32::BasepInitAppCompatData(emu),
        "BasepInitializeApphelpGlobals" => kernel32::BasepInitializeApphelpGlobals(emu),
        "BasepPostSuccessAppXExtension" => kernel32::BasepPostSuccessAppXExtension(emu),
        "BasepQueryAppCompat" => kernel32::BasepQueryAppCompat(emu),
        "CompareCalendarDates" => kernel32::CompareCalendarDates(emu),
        "CompatCacheLookupExe" => kernel32::CompatCacheLookupExe(emu),
        "GetApplicationRestartSettingsWorker" => kernel32::GetApplicationRestartSettingsWorker(emu),
        "GetComPlusPackageInstallStatus" => kernel32::GetComPlusPackageInstallStatus(emu),
        "GetProcessDEPPolicy" => kernel32::GetProcessDEPPolicy(emu),
        "GetSystemDEPPolicy" => kernel32::GetSystemDEPPolicy(emu),
        "QuirkGetDataWorker" => kernel32::QuirkGetDataWorker(emu),
        "QuirkIsEnabled2Worker" => kernel32::QuirkIsEnabled2Worker(emu),
        "QuirkIsEnabledForPackage2Worker" => kernel32::QuirkIsEnabledForPackage2Worker(emu),
        "QuirkIsEnabledForPackage3Worker" => kernel32::QuirkIsEnabledForPackage3Worker(emu),
        "QuirkIsEnabledForPackage4Worker" => kernel32::QuirkIsEnabledForPackage4Worker(emu),
        "QuirkIsEnabledForPackageWorker" => kernel32::QuirkIsEnabledForPackageWorker(emu),
        "QuirkIsEnabledForProcessWorker" => kernel32::QuirkIsEnabledForProcessWorker(emu),
        "QuirkIsEnabledWorker" => kernel32::QuirkIsEnabledWorker(emu),
        "SdbpCheckFromVersion" => kernel32::SdbpCheckFromVersion(emu),
        "SdbpCheckMatchingDevice" => kernel32::SdbpCheckMatchingDevice(emu),
        "SdbpCheckMatchingDir" => kernel32::SdbpCheckMatchingDir(emu),
        "SdbpCheckMatchingFiles" => kernel32::SdbpCheckMatchingFiles(emu),
        "SdbpCheckMatchingText" => kernel32::SdbpCheckMatchingText(emu),
        "SdbpCheckMatchingWildcardFiles" => kernel32::SdbpCheckMatchingWildcardFiles(emu),
        "SdbpCheckOSKind" => kernel32::SdbpCheckOSKind(emu),
        "SdbpCheckPackageAttributes" => kernel32::SdbpCheckPackageAttributes(emu),
        "SdbpCheckRuntimePlatform" => kernel32::SdbpCheckRuntimePlatform(emu),
        "SdbpCheckUptoVersion" => kernel32::SdbpCheckUptoVersion(emu),
        "SdbpCheckVersion" => kernel32::SdbpCheckVersion(emu),
        "SdbpGetPathAppPatch" => kernel32::SdbpGetPathAppPatch(emu),
        "SdbpGetPathAppPatchPreRS3" => kernel32::SdbpGetPathAppPatchPreRS3(emu),
        "SdbpGetPathAppraiser" => kernel32::SdbpGetPathAppraiser(emu),
        "SdbpGetPathCustomSdb" => kernel32::SdbpGetPathCustomSdb(emu),
        "SdbpGetPathCustomSdbPreRS3" => kernel32::SdbpGetPathCustomSdbPreRS3(emu),
        "SdbpGetPathSystem" => kernel32::SdbpGetPathSystem(emu),
        "SetComPlusPackageInstallStatus" => kernel32::SetComPlusPackageInstallStatus(emu),
        "SetProcessDEPPolicy" => kernel32::SetProcessDEPPolicy(emu),
        "UnregisterApplicationRecoveryCallback" => {
            kernel32::UnregisterApplicationRecoveryCallback(emu)
        }
        "UnregisterApplicationRestart" => kernel32::UnregisterApplicationRestart(emu),
        "ActivatePackageVirtualizationContext" => {
            kernel32::ActivatePackageVirtualizationContext(emu)
        }
        "AddScopedPolicyIDAce" => kernel32::AddScopedPolicyIDAce(emu),
        "AppPolicyGetClrCompat" => kernel32::AppPolicyGetClrCompat(emu),
        "AppPolicyGetCreateFileAccess" => kernel32::AppPolicyGetCreateFileAccess(emu),
        "AppPolicyGetLifecycleManagement" => kernel32::AppPolicyGetLifecycleManagement(emu),
        "AppPolicyGetMediaFoundationCodecLoading" => {
            kernel32::AppPolicyGetMediaFoundationCodecLoading(emu)
        }
        "AppPolicyGetProcessTerminationMethod" => {
            kernel32::AppPolicyGetProcessTerminationMethod(emu)
        }
        "AppPolicyGetShowDeveloperDiagnostic" => kernel32::AppPolicyGetShowDeveloperDiagnostic(emu),
        "AppPolicyGetThreadInitializationType" => {
            kernel32::AppPolicyGetThreadInitializationType(emu)
        }
        "AppPolicyGetWindowingModel" => kernel32::AppPolicyGetWindowingModel(emu),
        "AppXGetOSMaxVersionTested" => kernel32::AppXGetOSMaxVersionTested(emu),
        "BaseCheckAppcompatCache" => kernel32::BaseCheckAppcompatCache(emu),
        "BaseCheckAppcompatCacheEx" => kernel32::BaseCheckAppcompatCacheEx(emu),
        "BaseCleanupAppcompatCacheSupport" => kernel32::BaseCleanupAppcompatCacheSupport(emu),
        "BaseDumpAppcompatCache" => kernel32::BaseDumpAppcompatCache(emu),
        "BaseDumpAppcompatCacheWorker" => kernel32::BaseDumpAppcompatCacheWorker(emu),
        "BaseFlushAppcompatCache" => kernel32::BaseFlushAppcompatCache(emu),
        "BaseInitAppcompatCacheSupport" => kernel32::BaseInitAppcompatCacheSupport(emu),
        "BaseInitAppcompatCacheSupportWorker" => kernel32::BaseInitAppcompatCacheSupportWorker(emu),
        "BaseIsAppcompatInfrastructureDisabled" => {
            kernel32::BaseIsAppcompatInfrastructureDisabled(emu)
        }
        "BaseIsAppcompatInfrastructureDisabledWorker" => {
            kernel32::BaseIsAppcompatInfrastructureDisabledWorker(emu)
        }
        "BaseUpdateAppcompatCache" => kernel32::BaseUpdateAppcompatCache(emu),
        "BasepFinishPackageActivation" => kernel32::BasepFinishPackageActivation(emu),
        "BasepFinishPackageActivationForSxS" => kernel32::BasepFinishPackageActivationForSxS(emu),
        "BasepGetAppCompatData" => kernel32::BasepGetAppCompatData(emu),
        "BasepGetPackagedAppInfoForFile" => kernel32::BasepGetPackagedAppInfoForFile(emu),
        "BasepReleaseAppXContext" => kernel32::BasepReleaseAppXContext(emu),
        "BasepReleasePackagedAppInfo" => kernel32::BasepReleasePackagedAppInfo(emu),
        "CheckAllowDecryptedRemoteDestinationPolicy" => {
            kernel32::CheckAllowDecryptedRemoteDestinationPolicy(emu)
        }
        "ClosePackageInfo" => kernel32::ClosePackageInfo(emu),
        "CloseState" => kernel32::CloseState(emu),
        "CompareFileTime" => kernel32::CompareFileTime(emu),
        "CompareStringA" => kernel32::CompareStringA(emu),
        "CompareStringEx" => kernel32::CompareStringEx(emu),
        "CompareStringOrdinal" => kernel32::CompareStringOrdinal(emu),
        "CreatePackageVirtualizationContext" => kernel32::CreatePackageVirtualizationContext(emu),
        "DeactivatePackageVirtualizationContext" => {
            kernel32::DeactivatePackageVirtualizationContext(emu)
        }
        "DuplicatePackageVirtualizationContext" => {
            kernel32::DuplicatePackageVirtualizationContext(emu)
        }
        "FindPackagesByPackageFamily" => kernel32::FindPackagesByPackageFamily(emu),
        "FormatApplicationUserModelId" => kernel32::FormatApplicationUserModelId(emu),
        "GetAppContainerNamedObjectPath" => kernel32::GetAppContainerNamedObjectPath(emu),
        "GetApplicationRecoveryCallback" => kernel32::GetApplicationRecoveryCallback(emu),
        "GetApplicationRecoveryCallbackWorker" => {
            kernel32::GetApplicationRecoveryCallbackWorker(emu)
        }
        "GetApplicationRestartSettings" => kernel32::GetApplicationRestartSettings(emu),
        "GetApplicationUserModelId" => kernel32::GetApplicationUserModelId(emu),
        "GetCurrentApplicationUserModelId" => kernel32::GetCurrentApplicationUserModelId(emu),
        "GetCurrentPackageFamilyName" => kernel32::GetCurrentPackageFamilyName(emu),
        "GetCurrentPackageFullName" => kernel32::GetCurrentPackageFullName(emu),
        "GetCurrentPackageId" => kernel32::GetCurrentPackageId(emu),
        "GetCurrentPackageInfo" => kernel32::GetCurrentPackageInfo(emu),
        "GetCurrentPackagePath" => kernel32::GetCurrentPackagePath(emu),
        "GetCurrentPackageVirtualizationContext" => {
            kernel32::GetCurrentPackageVirtualizationContext(emu)
        }
        "GetPackageApplicationIds" => kernel32::GetPackageApplicationIds(emu),
        "GetPackageFamilyName" => kernel32::GetPackageFamilyName(emu),
        "GetPackageFullName" => kernel32::GetPackageFullName(emu),
        "GetPackageId" => kernel32::GetPackageId(emu),
        "GetPackageInfo" => kernel32::GetPackageInfo(emu),
        "GetPackagePath" => kernel32::GetPackagePath(emu),
        "GetPackagePathByFullName" => kernel32::GetPackagePathByFullName(emu),
        "GetPackagesByPackageFamily" => kernel32::GetPackagesByPackageFamily(emu),
        "GetProcessMitigationPolicy" => kernel32::GetProcessMitigationPolicy(emu),
        "GetStagedPackagePathByFullName" => kernel32::GetStagedPackagePathByFullName(emu),
        "GetStateFolder" => kernel32::GetStateFolder(emu),
        "LoadPackagedLibrary" => kernel32::LoadPackagedLibrary(emu),
        "NlsCheckPolicy" => kernel32::NlsCheckPolicy(emu),
        "OpenPackageInfoByFullName" => kernel32::OpenPackageInfoByFullName(emu),
        "OpenState" => kernel32::OpenState(emu),
        "OpenStateExplicit" => kernel32::OpenStateExplicit(emu),
        "PackageFamilyNameFromFullName" => kernel32::PackageFamilyNameFromFullName(emu),
        "PackageFamilyNameFromId" => kernel32::PackageFamilyNameFromId(emu),
        "PackageFullNameFromId" => kernel32::PackageFullNameFromId(emu),
        "PackageIdFromFullName" => kernel32::PackageIdFromFullName(emu),
        "PackageNameAndPublisherIdFromFamilyName" => {
            kernel32::PackageNameAndPublisherIdFromFamilyName(emu)
        }
        "ParseApplicationUserModelId" => kernel32::ParseApplicationUserModelId(emu),
        "QueryProtectedPolicy" => kernel32::QueryProtectedPolicy(emu),
        "QuirkGetData2Worker" => kernel32::QuirkGetData2Worker(emu),
        "QuirkIsEnabled3Worker" => kernel32::QuirkIsEnabled3Worker(emu),
        "ReleasePackageVirtualizationContext" => kernel32::ReleasePackageVirtualizationContext(emu),
        "SetProcessMitigationPolicy" => kernel32::SetProcessMitigationPolicy(emu),
        "SetProtectedPolicy" => kernel32::SetProtectedPolicy(emu),
        _ => return false,
    }
    true
}
