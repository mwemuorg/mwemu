pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    BeginUpdateResourceA, BeginUpdateResourceW, CreateActCtxA, CreateActCtxW, FreeLibrary,
    LoadLibraryA, LoadLibraryExA, LoadLibraryExW, LoadLibraryW,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::fallback::unsupported_exports;
use crate::winapi::winapi32::kernel32;

pub use crate::api::windows::libraries::kernel32::library_loader::*;

unsupported_exports!(
    "kernel32",
    ActivateActCtxWorker,
    CheckForReadOnlyResource,
    CreateActCtxWWorker,
    DeactivateActCtxWorker,
    EndUpdateResourceA,
    EndUpdateResourceW,
    FindActCtxSectionGuidWorker,
    GetCurrentActCtxWorker,
    QueryActCtxSettingsWWorker,
    QueryActCtxWWorker,
    UpdateResourceA,
    UpdateResourceW,
    ZombifyActCtxWorker,
);

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "FreeLibrary" => kernel32::FreeLibrary(emu),
        "GetProcAddress" => kernel32::GetProcAddress(emu),
        "LoadLibraryA" => kernel32::LoadLibraryA(emu),
        "LoadLibraryExA" => kernel32::LoadLibraryExA(emu),
        "LoadLibraryExW" => kernel32::LoadLibraryExW(emu),
        "LoadLibraryW" => kernel32::LoadLibraryW(emu),
        "Module32First" => kernel32::Module32First(emu),
        "Module32FirstW" => kernel32::Module32FirstW(emu),
        "Module32Next" => kernel32::Module32Next(emu),
        "Module32NextW" => kernel32::Module32NextW(emu),
        "ActivateActCtxWorker" => kernel32::ActivateActCtxWorker(emu),
        "BasepNotifyLoadStringResource" => kernel32::BasepNotifyLoadStringResource(emu),
        "BasepRegenerateActCtxWithLanguage" => kernel32::BasepRegenerateActCtxWithLanguage(emu),
        "BeginUpdateResourceA" => kernel32::BeginUpdateResourceA(emu),
        "BeginUpdateResourceW" => kernel32::BeginUpdateResourceW(emu),
        "CheckForReadOnlyResource" => kernel32::CheckForReadOnlyResource(emu),
        "CreateActCtxA" => kernel32::CreateActCtxA(emu),
        "CreateActCtxWWorker" => kernel32::CreateActCtxWWorker(emu),
        "DeactivateActCtxWorker" => kernel32::DeactivateActCtxWorker(emu),
        "FindActCtxSectionGuidWorker" => kernel32::FindActCtxSectionGuidWorker(emu),
        "FindActCtxSectionStringA" => kernel32::FindActCtxSectionStringA(emu),
        "FindActCtxSectionStringWWorker" => kernel32::FindActCtxSectionStringWWorker(emu),
        "GetBinaryTypeW" => kernel32::GetBinaryTypeW(emu),
        "GetCurrentActCtxWorker" => kernel32::GetCurrentActCtxWorker(emu),
        "LoadAppInitDllsImplementation" => kernel32::LoadAppInitDllsImplementation(emu),
        "NtWow64CsrBasepCreateActCtx" => kernel32::NtWow64CsrBasepCreateActCtx(emu),
        "QueryActCtxSettingsWWorker" => kernel32::QueryActCtxSettingsWWorker(emu),
        "QueryActCtxWWorker" => kernel32::QueryActCtxWWorker(emu),
        "ResourceDataMatch" => kernel32::ResourceDataMatch(emu),
        "ZombifyActCtxWorker" => kernel32::ZombifyActCtxWorker(emu),
        "ActivateActCtx" => kernel32::ActivateActCtx(emu),
        "AddDllDirectory" => kernel32::AddDllDirectory(emu),
        "CreateActCtxW" => kernel32::CreateActCtxW(emu),
        "FindActCtxSectionStringW" => kernel32::FindActCtxSectionStringW(emu),
        "FreeLibraryWhenCallbackReturns" => kernel32::FreeLibraryWhenCallbackReturns(emu),
        "GetBinaryType" => kernel32::GetBinaryType(emu),
        "GetBinaryTypeA" => kernel32::GetBinaryTypeA(emu),
        "RemoveDllDirectory" => kernel32::RemoveDllDirectory(emu),
        "ResolveDelayLoadedAPI" => kernel32::ResolveDelayLoadedAPI(emu),
        "ResolveDelayLoadsFromDll" => kernel32::ResolveDelayLoadsFromDll(emu),
        "SetDefaultDllDirectories" => kernel32::SetDefaultDllDirectories(emu),
        "RtlLookupFunctionEntry" => kernel32::RtlLookupFunctionEntry(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}

unsupported_exports!("kernel32", ResourceDataMatch, SXSManifest,);
