use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "ActivateActCtx" => kernelbase::ActivateActCtx(emu),
        "AddDllDirectory" => kernelbase::AddDllDirectory(emu),
        "CreateActCtxW" => kernelbase::CreateActCtxW(emu),
        "FindActCtxSectionStringW" => kernelbase::FindActCtxSectionStringW(emu),
        "FreeLibrary" => kernelbase::FreeLibrary(emu),
        "FreeLibraryWhenCallbackReturns" => kernelbase::FreeLibraryWhenCallbackReturns(emu),
        "GetProcAddress" => kernelbase::GetProcAddress(emu),
        "LoadLibraryA" => kernelbase::LoadLibraryA(emu),
        "LoadLibraryExA" => kernelbase::LoadLibraryExA(emu),
        "LoadLibraryExW" => kernelbase::LoadLibraryExW(emu),
        "LoadLibraryW" => kernelbase::LoadLibraryW(emu),
        "RemoveDllDirectory" => kernelbase::RemoveDllDirectory(emu),
        "ResolveDelayLoadedAPI" => kernelbase::ResolveDelayLoadedAPI(emu),
        "ResolveDelayLoadsFromDll" => kernelbase::ResolveDelayLoadsFromDll(emu),
        "SetDefaultDllDirectories" => kernelbase::SetDefaultDllDirectories(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
