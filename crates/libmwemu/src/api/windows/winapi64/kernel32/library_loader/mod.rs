pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    ActivateActCtx, BeginUpdateResourceA, BeginUpdateResourceW, CreateActCtxA, CreateActCtxW,
    FreeLibrary, LoadLibraryA, LoadLibraryExA, LoadLibraryExW, LoadLibraryW,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

pub use crate::api::windows::libraries::kernel32::library_loader::*;

pub use crate::winapi::common::modules::main_module_path;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "ActivateActCtx" => kernel32::ActivateActCtx(emu),
        "BeginUpdateResourceA" => kernel32::BeginUpdateResourceA(emu),
        "BeginUpdateResourceW" => kernel32::BeginUpdateResourceW(emu),
        "CreateActCtxA" => kernel32::CreateActCtxA(emu),
        "CreateActCtxW" => kernel32::CreateActCtxW(emu),
        "FindActCtxSectionStringA" => kernel32::FindActCtxSectionStringA(emu),
        "FindActCtxSectionStringW" => kernel32::FindActCtxSectionStringW(emu),
        "FreeLibrary" => kernel32::FreeLibrary(emu),
        "GetProcAddress" => kernel32::GetProcAddress(emu),
        "Module32First" => kernel32::Module32First(emu),
        "Module32FirstW" => kernel32::Module32FirstW(emu),
        "Module32Next" => kernel32::Module32Next(emu),
        "Module32NextW" => kernel32::Module32NextW(emu),
        "LoadLibraryA" => kernel32::LoadLibraryA(emu),
        "LoadLibraryExA" => kernel32::LoadLibraryExA(emu),
        "LoadLibraryExW" => kernel32::LoadLibraryExW(emu),
        "LoadLibraryW" => kernel32::LoadLibraryW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
