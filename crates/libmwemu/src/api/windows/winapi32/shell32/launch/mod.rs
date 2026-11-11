use crate::emu::Emu;
use crate::winapi::winapi32::shell32;

#[path = "../real_shell_execute_ex.rs"]
mod real_shell_execute_ex;
pub use real_shell_execute_ex::*;

#[path = "../real_shell_execute.rs"]
mod real_shell_execute;
pub use real_shell_execute::*;

#[path = "../find_executable.rs"]
mod find_executable;
pub use find_executable::*;

#[path = "../do_environment_subst.rs"]
mod do_environment_subst;
pub use do_environment_subst::*;

#[path = "../app_compat_run_dllw.rs"]
mod app_compat_run_dllw;
pub use app_compat_run_dllw::AppCompat_RunDLLW;

#[path = "../assoc_create_for_classes.rs"]
mod assoc_create_for_classes;
pub use assoc_create_for_classes::AssocCreateForClasses;

#[path = "../assoc_elem_create_for_key.rs"]
mod assoc_elem_create_for_key;
pub use assoc_elem_create_for_key::AssocElemCreateForKey;

#[path = "../assoc_get_details_of_prop_key.rs"]
mod assoc_get_details_of_prop_key;
pub use assoc_get_details_of_prop_key::AssocGetDetailsOfPropKey;

#[path = "../command_line_to_argv_w.rs"]
mod command_line_to_argv_w;
pub use command_line_to_argv_w::CommandLineToArgvW;

#[path = "../control_run_dll.rs"]
mod control_run_dll;
pub use control_run_dll::Control_RunDLL;

#[path = "../control_run_dlla.rs"]
mod control_run_dlla;
pub use control_run_dlla::Control_RunDLLA;

#[path = "../control_run_dll_as_user_w.rs"]
mod control_run_dll_as_user_w;
pub use control_run_dll_as_user_w::Control_RunDLLAsUserW;

#[path = "../control_run_dllw.rs"]
mod control_run_dllw;
pub use control_run_dllw::Control_RunDLLW;

#[path = "../create_storage_item_from_shell_item_full_trust_caller_for_package_with_process_handle.rs"]
mod create_storage_item_from_shell_item_full_trust_caller_for_package_with_process_handle;
pub use create_storage_item_from_shell_item_full_trust_caller_for_package_with_process_handle::CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle;

#[path = "../dll_can_unload_now.rs"]
mod dll_can_unload_now;
pub use dll_can_unload_now::DllCanUnloadNow;

#[path = "../dll_get_activation_factory.rs"]
mod dll_get_activation_factory;
pub use dll_get_activation_factory::DllGetActivationFactory;

#[path = "../dll_get_class_object.rs"]
mod dll_get_class_object;
pub use dll_get_class_object::DllGetClassObject;

#[path = "../dll_get_version.rs"]
mod dll_get_version;
pub use dll_get_version::DllGetVersion;

#[path = "../dll_install.rs"]
mod dll_install;
pub use dll_install::DllInstall;

#[path = "../dll_register_server.rs"]
mod dll_register_server;
pub use dll_register_server::DllRegisterServer;

#[path = "../dll_unregister_server.rs"]
mod dll_unregister_server;
pub use dll_unregister_server::DllUnregisterServer;

#[path = "../get_current_process_explicit_app_user_model_id.rs"]
mod get_current_process_explicit_app_user_model_id;
pub use get_current_process_explicit_app_user_model_id::GetCurrentProcessExplicitAppUserModelID;

#[path = "../is_desktop_explorer_process.rs"]
mod is_desktop_explorer_process;
pub use is_desktop_explorer_process::IsDesktopExplorerProcess;

#[path = "../is_process_an_explorer.rs"]
mod is_process_an_explorer;
pub use is_process_an_explorer::IsProcessAnExplorer;

#[path = "../is_user_an_admin.rs"]
mod is_user_an_admin;
pub use is_user_an_admin::IsUserAnAdmin;

#[path = "../launch_ms_help_run_dllw.rs"]
mod launch_ms_help_run_dllw;
pub use launch_ms_help_run_dllw::LaunchMSHelp_RunDLLW;

#[path = "../open_as_run_dll.rs"]
mod open_as_run_dll;
pub use open_as_run_dll::OpenAs_RunDLL;

#[path = "../open_as_run_dlla.rs"]
mod open_as_run_dlla;
pub use open_as_run_dlla::OpenAs_RunDLLA;

#[path = "../open_as_run_dllw.rs"]
mod open_as_run_dllw;
pub use open_as_run_dllw::OpenAs_RunDLLW;

#[path = "../options_run_dll.rs"]
mod options_run_dll;
pub use options_run_dll::Options_RunDLL;

#[path = "../options_run_dlla.rs"]
mod options_run_dlla;
pub use options_run_dlla::Options_RunDLLA;

#[path = "../options_run_dllw.rs"]
mod options_run_dllw;
pub use options_run_dllw::Options_RunDLLW;

#[path = "../prepare_disc_for_burn_run_dll_w.rs"]
mod prepare_disc_for_burn_run_dll_w;
pub use prepare_disc_for_burn_run_dll_w::PrepareDiscForBurnRunDllW;

#[path = "../printers_get_command_run_dll.rs"]
mod printers_get_command_run_dll;
pub use printers_get_command_run_dll::PrintersGetCommand_RunDLL;

#[path = "../printers_get_command_run_dlla.rs"]
mod printers_get_command_run_dlla;
pub use printers_get_command_run_dlla::PrintersGetCommand_RunDLLA;

#[path = "../printers_get_command_run_dllw.rs"]
mod printers_get_command_run_dllw;
pub use printers_get_command_run_dllw::PrintersGetCommand_RunDLLW;

#[path = "../regenerate_user_environment.rs"]
mod regenerate_user_environment;
pub use regenerate_user_environment::RegenerateUserEnvironment;

#[path = "../run_as_new_user_run_dllw.rs"]
mod run_as_new_user_run_dllw;
pub use run_as_new_user_run_dllw::RunAsNewUser_RunDLLW;

#[path = "../sh_assoc_enum_handlers.rs"]
mod sh_assoc_enum_handlers;
pub use sh_assoc_enum_handlers::SHAssocEnumHandlers;

#[path = "../sh_assoc_enum_handlers_for_protocol_by_application.rs"]
mod sh_assoc_enum_handlers_for_protocol_by_application;
pub use sh_assoc_enum_handlers_for_protocol_by_application::SHAssocEnumHandlersForProtocolByApplication;

#[path = "../sh_create_association_registration.rs"]
mod sh_create_association_registration;
pub use sh_create_association_registration::SHCreateAssociationRegistration;

#[path = "../sh_create_local_server_run_dll.rs"]
mod sh_create_local_server_run_dll;
pub use sh_create_local_server_run_dll::SHCreateLocalServerRunDll;

#[path = "../sh_create_process_as_user_w.rs"]
mod sh_create_process_as_user_w;
pub use sh_create_process_as_user_w::SHCreateProcessAsUserW;

#[path = "../sh_create_query_cancel_auto_play_moniker.rs"]
mod sh_create_query_cancel_auto_play_moniker;
pub use sh_create_query_cancel_auto_play_moniker::SHCreateQueryCancelAutoPlayMoniker;

#[path = "../shell32_c_mount_point_do_autorun.rs"]
mod shell32_c_mount_point_do_autorun;
pub use shell32_c_mount_point_do_autorun::SHELL32_CMountPoint_DoAutorun;

#[path = "../shell32_c_mount_point_do_autorun_prompt.rs"]
mod shell32_c_mount_point_do_autorun_prompt;
pub use shell32_c_mount_point_do_autorun_prompt::SHELL32_CMountPoint_DoAutorunPrompt;

#[path = "../shell32_c_mount_point_is_auto_run_drive_and_enabled_by_policy.rs"]
mod shell32_c_mount_point_is_auto_run_drive_and_enabled_by_policy;
pub use shell32_c_mount_point_is_auto_run_drive_and_enabled_by_policy::SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy;

#[path = "../shell32_c_mount_point_process_auto_run_file.rs"]
mod shell32_c_mount_point_process_auto_run_file;
pub use shell32_c_mount_point_process_auto_run_file::SHELL32_CMountPoint_ProcessAutoRunFile;

#[path = "../shell32_close_autoplay_prompt.rs"]
mod shell32_close_autoplay_prompt;
pub use shell32_close_autoplay_prompt::SHELL32_CloseAutoplayPrompt;

#[path = "../shell32_command_line_from_msi_descriptor.rs"]
mod shell32_command_line_from_msi_descriptor;
pub use shell32_command_line_from_msi_descriptor::SHELL32_CommandLineFromMsiDescriptor;

#[path = "../shell32_generate_app_id.rs"]
mod shell32_generate_app_id;
pub use shell32_generate_app_id::SHELL32_GenerateAppID;

#[path = "../shell32_get_app_id_root.rs"]
mod shell32_get_app_id_root;
pub use shell32_get_app_id_root::SHELL32_GetAppIDRoot;

#[path = "../shell32_verify_safer_trust.rs"]
mod shell32_verify_safer_trust;
pub use shell32_verify_safer_trust::SHELL32_VerifySaferTrust;

#[path = "../sh_enumerate_unread_mail_accounts_w.rs"]
mod sh_enumerate_unread_mail_accounts_w;
pub use sh_enumerate_unread_mail_accounts_w::SHEnumerateUnreadMailAccountsW;

#[path = "../sh_evaluate_system_command_template.rs"]
mod sh_evaluate_system_command_template;
pub use sh_evaluate_system_command_template::SHEvaluateSystemCommandTemplate;

#[path = "../sh_execute_error_message_box.rs"]
mod sh_execute_error_message_box;
pub use sh_execute_error_message_box::SHExecuteErrorMessageBox;

#[path = "../sh_get_unread_mail_count_w.rs"]
mod sh_get_unread_mail_count_w;
pub use sh_get_unread_mail_count_w::SHGetUnreadMailCountW;

#[path = "../sh_help_shortcuts_run_dll.rs"]
mod sh_help_shortcuts_run_dll;
pub use sh_help_shortcuts_run_dll::SHHelpShortcuts_RunDLL;

#[path = "../sh_help_shortcuts_run_dlla.rs"]
mod sh_help_shortcuts_run_dlla;
pub use sh_help_shortcuts_run_dlla::SHHelpShortcuts_RunDLLA;

#[path = "../sh_help_shortcuts_run_dllw.rs"]
mod sh_help_shortcuts_run_dllw;
pub use sh_help_shortcuts_run_dllw::SHHelpShortcuts_RunDLLW;

#[path = "../sh_restricted.rs"]
mod sh_restricted;
pub use sh_restricted::SHRestricted;

#[path = "../sh_set_unread_mail_count_w.rs"]
mod sh_set_unread_mail_count_w;
pub use sh_set_unread_mail_count_w::SHSetUnreadMailCountW;

#[path = "../set_current_process_explicit_app_user_model_id.rs"]
mod set_current_process_explicit_app_user_model_id;
pub use set_current_process_explicit_app_user_model_id::SetCurrentProcessExplicitAppUserModelID;

#[path = "../shell_exec_run_dll.rs"]
mod shell_exec_run_dll;
pub use shell_exec_run_dll::ShellExec_RunDLL;

#[path = "../shell_exec_run_dlla.rs"]
mod shell_exec_run_dlla;
pub use shell_exec_run_dlla::ShellExec_RunDLLA;

#[path = "../shell_exec_run_dllw.rs"]
mod shell_exec_run_dllw;
pub use shell_exec_run_dllw::ShellExec_RunDLLW;

#[path = "../shell_execute_ex.rs"]
mod shell_execute_ex;
pub use shell_execute_ex::ShellExecuteEx;

#[path = "../shell_execute_ex_a.rs"]
mod shell_execute_ex_a;
pub use shell_execute_ex_a::ShellExecuteExA;

#[path = "../shell_execute_ex_w.rs"]
mod shell_execute_ex_w;
pub use shell_execute_ex_w::ShellExecuteExW;

#[path = "../shell_hook_proc.rs"]
mod shell_hook_proc;
pub use shell_hook_proc::ShellHookProc;

#[path = "../wow_shell_execute.rs"]
mod wow_shell_execute;
pub use wow_shell_execute::WOWShellExecute;

#[path = "../wait_for_explorer_restart_w.rs"]
mod wait_for_explorer_restart_w;
pub use wait_for_explorer_restart_w::WaitForExplorerRestartW;

#[path = "../shell_execute_a.rs"]
mod shell_execute_a;
pub use shell_execute_a::ShellExecuteA;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "ShellExecuteA" => shell32::ShellExecuteA(emu),
        "ShellExecuteW" => shell32::ShellExecuteW(emu),
        "AppCompat_RunDLLW" => shell32::AppCompat_RunDLLW(emu),
        "AssocCreateForClasses" => shell32::AssocCreateForClasses(emu),
        "AssocElemCreateForKey" => shell32::AssocElemCreateForKey(emu),
        "AssocGetDetailsOfPropKey" => shell32::AssocGetDetailsOfPropKey(emu),
        "CommandLineToArgvW" => shell32::CommandLineToArgvW(emu),
        "Control_RunDLL" => shell32::Control_RunDLL(emu),
        "Control_RunDLLA" => shell32::Control_RunDLLA(emu),
        "Control_RunDLLAsUserW" => shell32::Control_RunDLLAsUserW(emu),
        "Control_RunDLLW" => shell32::Control_RunDLLW(emu),
        "CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle" => {
            shell32::CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle(
                emu,
            )
        }
        "DllCanUnloadNow" => shell32::DllCanUnloadNow(emu),
        "DllGetActivationFactory" => shell32::DllGetActivationFactory(emu),
        "DllGetClassObject" => shell32::DllGetClassObject(emu),
        "DllGetVersion" => shell32::DllGetVersion(emu),
        "DllInstall" => shell32::DllInstall(emu),
        "DllRegisterServer" => shell32::DllRegisterServer(emu),
        "DllUnregisterServer" => shell32::DllUnregisterServer(emu),
        "DoEnvironmentSubstA" => shell32::DoEnvironmentSubstA(emu),
        "DoEnvironmentSubstW" => shell32::DoEnvironmentSubstW(emu),
        "FindExecutableA" => shell32::FindExecutableA(emu),
        "FindExecutableW" => shell32::FindExecutableW(emu),
        "GetCurrentProcessExplicitAppUserModelID" => {
            shell32::GetCurrentProcessExplicitAppUserModelID(emu)
        }
        "IsDesktopExplorerProcess" => shell32::IsDesktopExplorerProcess(emu),
        "IsProcessAnExplorer" => shell32::IsProcessAnExplorer(emu),
        "IsUserAnAdmin" => shell32::IsUserAnAdmin(emu),
        "LaunchMSHelp_RunDLLW" => shell32::LaunchMSHelp_RunDLLW(emu),
        "OpenAs_RunDLL" => shell32::OpenAs_RunDLL(emu),
        "OpenAs_RunDLLA" => shell32::OpenAs_RunDLLA(emu),
        "OpenAs_RunDLLW" => shell32::OpenAs_RunDLLW(emu),
        "Options_RunDLL" => shell32::Options_RunDLL(emu),
        "Options_RunDLLA" => shell32::Options_RunDLLA(emu),
        "Options_RunDLLW" => shell32::Options_RunDLLW(emu),
        "PrepareDiscForBurnRunDllW" => shell32::PrepareDiscForBurnRunDllW(emu),
        "PrintersGetCommand_RunDLL" => shell32::PrintersGetCommand_RunDLL(emu),
        "PrintersGetCommand_RunDLLA" => shell32::PrintersGetCommand_RunDLLA(emu),
        "PrintersGetCommand_RunDLLW" => shell32::PrintersGetCommand_RunDLLW(emu),
        "RealShellExecuteA" => shell32::RealShellExecuteA(emu),
        "RealShellExecuteExA" => shell32::RealShellExecuteExA(emu),
        "RealShellExecuteExW" => shell32::RealShellExecuteExW(emu),
        "RealShellExecuteW" => shell32::RealShellExecuteW(emu),
        "RegenerateUserEnvironment" => shell32::RegenerateUserEnvironment(emu),
        "RunAsNewUser_RunDLLW" => shell32::RunAsNewUser_RunDLLW(emu),
        "SHAssocEnumHandlers" => shell32::SHAssocEnumHandlers(emu),
        "SHAssocEnumHandlersForProtocolByApplication" => {
            shell32::SHAssocEnumHandlersForProtocolByApplication(emu)
        }
        "SHCreateAssociationRegistration" => shell32::SHCreateAssociationRegistration(emu),
        "SHCreateLocalServerRunDll" => shell32::SHCreateLocalServerRunDll(emu),
        "SHCreateProcessAsUserW" => shell32::SHCreateProcessAsUserW(emu),
        "SHCreateQueryCancelAutoPlayMoniker" => shell32::SHCreateQueryCancelAutoPlayMoniker(emu),
        "SHELL32_CMountPoint_DoAutorun" => shell32::SHELL32_CMountPoint_DoAutorun(emu),
        "SHELL32_CMountPoint_DoAutorunPrompt" => shell32::SHELL32_CMountPoint_DoAutorunPrompt(emu),
        "SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy" => {
            shell32::SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy(emu)
        }
        "SHELL32_CMountPoint_ProcessAutoRunFile" => {
            shell32::SHELL32_CMountPoint_ProcessAutoRunFile(emu)
        }
        "SHELL32_CloseAutoplayPrompt" => shell32::SHELL32_CloseAutoplayPrompt(emu),
        "SHELL32_CommandLineFromMsiDescriptor" => {
            shell32::SHELL32_CommandLineFromMsiDescriptor(emu)
        }
        "SHELL32_GenerateAppID" => shell32::SHELL32_GenerateAppID(emu),
        "SHELL32_GetAppIDRoot" => shell32::SHELL32_GetAppIDRoot(emu),
        "SHELL32_VerifySaferTrust" => shell32::SHELL32_VerifySaferTrust(emu),
        "SHEnumerateUnreadMailAccountsW" => shell32::SHEnumerateUnreadMailAccountsW(emu),
        "SHEvaluateSystemCommandTemplate" => shell32::SHEvaluateSystemCommandTemplate(emu),
        "SHExecuteErrorMessageBox" => shell32::SHExecuteErrorMessageBox(emu),
        "SHGetUnreadMailCountW" => shell32::SHGetUnreadMailCountW(emu),
        "SHHelpShortcuts_RunDLL" => shell32::SHHelpShortcuts_RunDLL(emu),
        "SHHelpShortcuts_RunDLLA" => shell32::SHHelpShortcuts_RunDLLA(emu),
        "SHHelpShortcuts_RunDLLW" => shell32::SHHelpShortcuts_RunDLLW(emu),
        "SHRestricted" => shell32::SHRestricted(emu),
        "SHSetUnreadMailCountW" => shell32::SHSetUnreadMailCountW(emu),
        "SetCurrentProcessExplicitAppUserModelID" => {
            shell32::SetCurrentProcessExplicitAppUserModelID(emu)
        }
        "ShellExec_RunDLL" => shell32::ShellExec_RunDLL(emu),
        "ShellExec_RunDLLA" => shell32::ShellExec_RunDLLA(emu),
        "ShellExec_RunDLLW" => shell32::ShellExec_RunDLLW(emu),
        "ShellExecuteEx" => shell32::ShellExecuteEx(emu),
        "ShellExecuteExA" => shell32::ShellExecuteExA(emu),
        "ShellExecuteExW" => shell32::ShellExecuteExW(emu),
        "ShellHookProc" => shell32::ShellHookProc(emu),
        "WOWShellExecute" => shell32::WOWShellExecute(emu),
        "WaitForExplorerRestartW" => shell32::WaitForExplorerRestartW(emu),
        _ => return false,
    }
    true
}
