use crate::emu::Emu;
use crate::winapi::winapi32::shell32;

mod real_shell_execute_ex;
pub use real_shell_execute_ex::{RealShellExecuteExA, RealShellExecuteExW};

mod real_shell_execute;
pub use real_shell_execute::{RealShellExecuteA, RealShellExecuteW};

mod find_executable;
pub use find_executable::{FindExecutableA, FindExecutableW};

mod do_environment_subst;
pub use do_environment_subst::{DoEnvironmentSubstA, DoEnvironmentSubstW};

mod app_compat_run_dllw;
pub use app_compat_run_dllw::AppCompat_RunDLLW;

mod assoc_create_for_classes;
pub use assoc_create_for_classes::AssocCreateForClasses;

mod assoc_elem_create_for_key;
pub use assoc_elem_create_for_key::AssocElemCreateForKey;

mod assoc_get_details_of_prop_key;
pub use assoc_get_details_of_prop_key::AssocGetDetailsOfPropKey;

mod command_line_to_argv_w;
pub use command_line_to_argv_w::CommandLineToArgvW;

mod control_run_dll;
pub use control_run_dll::{Control_RunDLL, Control_RunDLLA, Control_RunDLLW};

mod control_run_dll_as_user_w;
pub use control_run_dll_as_user_w::Control_RunDLLAsUserW;

mod create_storage_item_from_shell_item_full_trust_caller_for_package_with_process_handle;
pub use create_storage_item_from_shell_item_full_trust_caller_for_package_with_process_handle::CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle;

mod dll_can_unload_now;
pub use dll_can_unload_now::DllCanUnloadNow;

mod dll_get_activation_factory;
pub use dll_get_activation_factory::DllGetActivationFactory;

mod dll_get_class_object;
pub use dll_get_class_object::DllGetClassObject;

mod dll_get_version;
pub use dll_get_version::DllGetVersion;

mod dll_install;
pub use dll_install::DllInstall;

mod dll_register_server;
pub use dll_register_server::DllRegisterServer;

mod dll_unregister_server;
pub use dll_unregister_server::DllUnregisterServer;

mod get_current_process_explicit_app_user_model_id;
pub use get_current_process_explicit_app_user_model_id::GetCurrentProcessExplicitAppUserModelID;

mod is_desktop_explorer_process;
pub use is_desktop_explorer_process::IsDesktopExplorerProcess;

mod is_process_an_explorer;
pub use is_process_an_explorer::IsProcessAnExplorer;

mod is_user_an_admin;
pub use is_user_an_admin::IsUserAnAdmin;

mod launch_ms_help_run_dllw;
pub use launch_ms_help_run_dllw::LaunchMSHelp_RunDLLW;

mod open_as_run_dll;
pub use open_as_run_dll::{OpenAs_RunDLL, OpenAs_RunDLLA, OpenAs_RunDLLW};

mod options_run_dll;
pub use options_run_dll::{Options_RunDLL, Options_RunDLLA, Options_RunDLLW};

mod prepare_disc_for_burn_run_dll_w;
pub use prepare_disc_for_burn_run_dll_w::PrepareDiscForBurnRunDllW;

mod printers_get_command_run_dll;
pub use printers_get_command_run_dll::{
    PrintersGetCommand_RunDLL, PrintersGetCommand_RunDLLA, PrintersGetCommand_RunDLLW,
};

mod regenerate_user_environment;
pub use regenerate_user_environment::RegenerateUserEnvironment;

mod run_as_new_user_run_dllw;
pub use run_as_new_user_run_dllw::RunAsNewUser_RunDLLW;

mod sh_assoc_enum_handlers;
pub use sh_assoc_enum_handlers::SHAssocEnumHandlers;

mod sh_assoc_enum_handlers_for_protocol_by_application;
pub use sh_assoc_enum_handlers_for_protocol_by_application::SHAssocEnumHandlersForProtocolByApplication;

mod sh_create_association_registration;
pub use sh_create_association_registration::SHCreateAssociationRegistration;

mod sh_create_local_server_run_dll;
pub use sh_create_local_server_run_dll::SHCreateLocalServerRunDll;

mod sh_create_process_as_user_w;
pub use sh_create_process_as_user_w::SHCreateProcessAsUserW;

mod sh_create_query_cancel_auto_play_moniker;
pub use sh_create_query_cancel_auto_play_moniker::SHCreateQueryCancelAutoPlayMoniker;

mod shell32_c_mount_point_do_autorun;
pub use shell32_c_mount_point_do_autorun::SHELL32_CMountPoint_DoAutorun;

mod shell32_c_mount_point_do_autorun_prompt;
pub use shell32_c_mount_point_do_autorun_prompt::SHELL32_CMountPoint_DoAutorunPrompt;

mod shell32_c_mount_point_is_auto_run_drive_and_enabled_by_policy;
pub use shell32_c_mount_point_is_auto_run_drive_and_enabled_by_policy::SHELL32_CMountPoint_IsAutoRunDriveAndEnabledByPolicy;

mod shell32_c_mount_point_process_auto_run_file;
pub use shell32_c_mount_point_process_auto_run_file::SHELL32_CMountPoint_ProcessAutoRunFile;

mod shell32_close_autoplay_prompt;
pub use shell32_close_autoplay_prompt::SHELL32_CloseAutoplayPrompt;

mod shell32_command_line_from_msi_descriptor;
pub use shell32_command_line_from_msi_descriptor::SHELL32_CommandLineFromMsiDescriptor;

mod shell32_generate_app_id;
pub use shell32_generate_app_id::SHELL32_GenerateAppID;

mod shell32_get_app_id_root;
pub use shell32_get_app_id_root::SHELL32_GetAppIDRoot;

mod shell32_verify_safer_trust;
pub use shell32_verify_safer_trust::SHELL32_VerifySaferTrust;

mod sh_enumerate_unread_mail_accounts_w;
pub use sh_enumerate_unread_mail_accounts_w::SHEnumerateUnreadMailAccountsW;

mod sh_evaluate_system_command_template;
pub use sh_evaluate_system_command_template::SHEvaluateSystemCommandTemplate;

mod sh_execute_error_message_box;
pub use sh_execute_error_message_box::SHExecuteErrorMessageBox;

mod sh_get_unread_mail_count_w;
pub use sh_get_unread_mail_count_w::SHGetUnreadMailCountW;

mod sh_help_shortcuts_run_dll;
pub use sh_help_shortcuts_run_dll::{
    SHHelpShortcuts_RunDLL, SHHelpShortcuts_RunDLLA, SHHelpShortcuts_RunDLLW,
};

mod sh_restricted;
pub use sh_restricted::SHRestricted;

mod sh_set_unread_mail_count_w;
pub use sh_set_unread_mail_count_w::SHSetUnreadMailCountW;

mod set_current_process_explicit_app_user_model_id;
pub use set_current_process_explicit_app_user_model_id::SetCurrentProcessExplicitAppUserModelID;

mod shell_exec_run_dll;
pub use shell_exec_run_dll::{ShellExec_RunDLL, ShellExec_RunDLLA, ShellExec_RunDLLW};

mod shell_execute_ex;
pub use shell_execute_ex::{ShellExecuteEx, ShellExecuteExA, ShellExecuteExW};

mod shell_hook_proc;
pub use shell_hook_proc::ShellHookProc;

mod wow_shell_execute;
pub use wow_shell_execute::WOWShellExecute;

mod wait_for_explorer_restart_w;
pub use wait_for_explorer_restart_w::WaitForExplorerRestartW;

mod shell_execute;
pub use shell_execute::{ShellExecuteA, ShellExecuteW};

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
