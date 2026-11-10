use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

#[path = "../create_act_ctx.rs"]
mod create_act_ctx;
pub use create_act_ctx::*;

#[path = "../update_resource.rs"]
mod update_resource;
pub use update_resource::*;

#[path = "../load_library_ex.rs"]
mod load_library_ex;
pub use load_library_ex::*;

#[path = "../load_library.rs"]
mod load_library;
pub use load_library::*;

#[path = "../find_resource.rs"]
mod find_resource;
pub use find_resource::*;

#[path = "../end_update_resource.rs"]
mod end_update_resource;
pub use end_update_resource::*;

#[path = "../begin_update_resource.rs"]
mod begin_update_resource;
pub use begin_update_resource::*;

#[path = "../free_library.rs"]
mod free_library;
pub use free_library::*;

#[path = "../free_resource.rs"]
mod free_resource;
pub use free_resource::*;

#[path = "../get_proc_address.rs"]
mod get_proc_address;
pub use get_proc_address::*;

#[path = "../load_resource.rs"]
mod load_resource;
pub use load_resource::*;

#[path = "../lock_resource.rs"]
mod lock_resource;
pub use lock_resource::*;

#[path = "../sizeof_resource.rs"]
mod sizeof_resource;
pub use sizeof_resource::*;

#[path = "../activate_act_ctx_worker.rs"]
mod activate_act_ctx_worker;
pub use activate_act_ctx_worker::*;

#[path = "../check_for_read_only_resource.rs"]
mod check_for_read_only_resource;
pub use check_for_read_only_resource::*;

#[path = "../create_act_ctx_w_worker.rs"]
mod create_act_ctx_w_worker;
pub use create_act_ctx_w_worker::*;

#[path = "../deactivate_act_ctx_worker.rs"]
mod deactivate_act_ctx_worker;
pub use deactivate_act_ctx_worker::*;

#[path = "../find_act_ctx_section_guid_worker.rs"]
mod find_act_ctx_section_guid_worker;
pub use find_act_ctx_section_guid_worker::*;

#[path = "../get_current_act_ctx_worker.rs"]
mod get_current_act_ctx_worker;
pub use get_current_act_ctx_worker::*;

#[path = "../query_act_ctx_settings_w_worker.rs"]
mod query_act_ctx_settings_w_worker;
pub use query_act_ctx_settings_w_worker::*;

#[path = "../query_act_ctx_w_worker.rs"]
mod query_act_ctx_w_worker;
pub use query_act_ctx_w_worker::*;

#[path = "../resource_data_match.rs"]
mod resource_data_match;
pub use resource_data_match::*;

#[path = "../sxs_manifest.rs"]
mod sxs_manifest;
pub use sxs_manifest::*;

#[path = "../zombify_act_ctx_worker.rs"]
mod zombify_act_ctx_worker;
pub use zombify_act_ctx_worker::*;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "FindResourceA" => kernel32::FindResourceA(emu),
        "FindResourceW" => kernel32::FindResourceW(emu),
        "FreeLibrary" => kernel32::FreeLibrary(emu),
        "FreeResource" => kernel32::FreeResource(emu),
        "GetProcAddress" => kernel32::GetProcAddress(emu),
        "LoadLibraryA" => kernel32::LoadLibraryA(emu),
        "LoadLibraryExA" => kernel32::LoadLibraryExA(emu),
        "LoadLibraryExW" => kernel32::LoadLibraryExW(emu),
        "LoadLibraryW" => kernel32::LoadLibraryW(emu),
        "LoadResource" => kernel32::LoadResource(emu),
        "LockResource" => kernel32::LockResource(emu),
        "Module32First" => kernel32::Module32First(emu),
        "Module32FirstW" => kernel32::Module32FirstW(emu),
        "Module32Next" => kernel32::Module32Next(emu),
        "Module32NextW" => kernel32::Module32NextW(emu),
        "SizeofResource" => kernel32::SizeofResource(emu),
        "ActivateActCtxWorker" => kernel32::ActivateActCtxWorker(emu),
        "BasepNotifyLoadStringResource" => kernel32::BasepNotifyLoadStringResource(emu),
        "BasepRegenerateActCtxWithLanguage" => kernel32::BasepRegenerateActCtxWithLanguage(emu),
        "BeginUpdateResourceA" => kernel32::BeginUpdateResourceA(emu),
        "BeginUpdateResourceW" => kernel32::BeginUpdateResourceW(emu),
        "CheckForReadOnlyResource" => kernel32::CheckForReadOnlyResource(emu),
        "CreateActCtxA" => kernel32::CreateActCtxA(emu),
        "CreateActCtxWWorker" => kernel32::CreateActCtxWWorker(emu),
        "DeactivateActCtxWorker" => kernel32::DeactivateActCtxWorker(emu),
        "EndUpdateResourceA" => kernel32::EndUpdateResourceA(emu),
        "EndUpdateResourceW" => kernel32::EndUpdateResourceW(emu),
        "FindActCtxSectionGuidWorker" => kernel32::FindActCtxSectionGuidWorker(emu),
        "FindActCtxSectionStringA" => kernel32::FindActCtxSectionStringA(emu),
        "FindActCtxSectionStringWWorker" => kernel32::FindActCtxSectionStringWWorker(emu),
        "GetBinaryTypeW" => kernel32::GetBinaryTypeW(emu),
        "GetCurrentActCtxWorker" => kernel32::GetCurrentActCtxWorker(emu),
        "GetDllDirectoryA" => kernel32::GetDllDirectoryA(emu),
        "GetDllDirectoryW" => kernel32::GetDllDirectoryW(emu),
        "LoadAppInitDllsImplementation" => kernel32::LoadAppInitDllsImplementation(emu),
        "NtWow64CsrBasepCreateActCtx" => kernel32::NtWow64CsrBasepCreateActCtx(emu),
        "QueryActCtxSettingsWWorker" => kernel32::QueryActCtxSettingsWWorker(emu),
        "QueryActCtxWWorker" => kernel32::QueryActCtxWWorker(emu),
        "ResourceDataMatch" => kernel32::ResourceDataMatch(emu),
        "SetDllDirectoryA" => kernel32::SetDllDirectoryA(emu),
        "SetDllDirectoryW" => kernel32::SetDllDirectoryW(emu),
        "UpdateResourceA" => kernel32::UpdateResourceA(emu),
        "UpdateResourceW" => kernel32::UpdateResourceW(emu),
        "ZombifyActCtxWorker" => kernel32::ZombifyActCtxWorker(emu),
        "ActivateActCtx" => kernel32::ActivateActCtx(emu),
        "AddDllDirectory" => kernel32::AddDllDirectory(emu),
        "AddRefActCtx" => kernel32::AddRefActCtx(emu),
        "AddRefActCtxWorker" => kernel32::AddRefActCtxWorker(emu),
        "BaseQueryModuleData" => kernel32::BaseQueryModuleData(emu),
        "BasepQueryModuleChpeSettings" => kernel32::BasepQueryModuleChpeSettings(emu),
        "CheckForReadOnlyResourceFilter" => kernel32::CheckForReadOnlyResourceFilter(emu),
        "CreateActCtxW" => kernel32::CreateActCtxW(emu),
        "CreateMemoryResourceNotification" => kernel32::CreateMemoryResourceNotification(emu),
        "DeactivateActCtx" => kernel32::DeactivateActCtx(emu),
        "DelayLoadFailureHook" => kernel32::DelayLoadFailureHook(emu),
        "DisableThreadLibraryCalls" => kernel32::DisableThreadLibraryCalls(emu),
        "EnumResourceLanguagesA" => kernel32::EnumResourceLanguagesA(emu),
        "EnumResourceLanguagesExA" => kernel32::EnumResourceLanguagesExA(emu),
        "EnumResourceLanguagesExW" => kernel32::EnumResourceLanguagesExW(emu),
        "EnumResourceLanguagesW" => kernel32::EnumResourceLanguagesW(emu),
        "EnumResourceNamesA" => kernel32::EnumResourceNamesA(emu),
        "EnumResourceNamesExA" => kernel32::EnumResourceNamesExA(emu),
        "EnumResourceNamesExW" => kernel32::EnumResourceNamesExW(emu),
        "EnumResourceNamesW" => kernel32::EnumResourceNamesW(emu),
        "EnumResourceTypesA" => kernel32::EnumResourceTypesA(emu),
        "EnumResourceTypesExA" => kernel32::EnumResourceTypesExA(emu),
        "EnumResourceTypesExW" => kernel32::EnumResourceTypesExW(emu),
        "EnumResourceTypesW" => kernel32::EnumResourceTypesW(emu),
        "FindActCtxSectionGuid" => kernel32::FindActCtxSectionGuid(emu),
        "FindActCtxSectionStringW" => kernel32::FindActCtxSectionStringW(emu),
        "FindResourceExA" => kernel32::FindResourceExA(emu),
        "FindResourceExW" => kernel32::FindResourceExW(emu),
        "FreeLibraryAndExitThread" => kernel32::FreeLibraryAndExitThread(emu),
        "FreeLibraryWhenCallbackReturns" => kernel32::FreeLibraryWhenCallbackReturns(emu),
        "GetBinaryType" => kernel32::GetBinaryType(emu),
        "GetBinaryTypeA" => kernel32::GetBinaryTypeA(emu),
        "GetCurrentActCtx" => kernel32::GetCurrentActCtx(emu),
        "K32EnumProcessModules" => kernel32::K32EnumProcessModules(emu),
        "K32EnumProcessModulesEx" => kernel32::K32EnumProcessModulesEx(emu),
        "K32GetModuleBaseNameA" => kernel32::K32GetModuleBaseNameA(emu),
        "K32GetModuleBaseNameW" => kernel32::K32GetModuleBaseNameW(emu),
        "K32GetModuleFileNameExA" => kernel32::K32GetModuleFileNameExA(emu),
        "K32GetModuleFileNameExW" => kernel32::K32GetModuleFileNameExW(emu),
        "K32GetModuleInformation" => kernel32::K32GetModuleInformation(emu),
        "LoadAppInitDlls" => kernel32::LoadAppInitDlls(emu),
        "LoadModule" => kernel32::LoadModule(emu),
        "QueryActCtxSettingsW" => kernel32::QueryActCtxSettingsW(emu),
        "QueryActCtxW" => kernel32::QueryActCtxW(emu),
        "QueryMemoryResourceNotification" => kernel32::QueryMemoryResourceNotification(emu),
        "ReleaseActCtx" => kernel32::ReleaseActCtx(emu),
        "ReleaseActCtxWorker" => kernel32::ReleaseActCtxWorker(emu),
        "RemoveDllDirectory" => kernel32::RemoveDllDirectory(emu),
        "ResolveDelayLoadedAPI" => kernel32::ResolveDelayLoadedAPI(emu),
        "ResolveDelayLoadsFromDll" => kernel32::ResolveDelayLoadsFromDll(emu),
        "SetDefaultDllDirectories" => kernel32::SetDefaultDllDirectories(emu),
        "ZombifyActCtx" => kernel32::ZombifyActCtx(emu),
        "RtlAddFunctionTable" => kernel32::RtlAddFunctionTable(emu),
        "RtlDeleteFunctionTable" => kernel32::RtlDeleteFunctionTable(emu),
        "RtlInstallFunctionTableCallback" => kernel32::RtlInstallFunctionTableCallback(emu),
        "RtlLookupFunctionEntry" => kernel32::RtlLookupFunctionEntry(emu),
        _ => return false,
    }
    true
}
