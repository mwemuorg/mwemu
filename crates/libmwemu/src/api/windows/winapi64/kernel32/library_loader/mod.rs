use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod load_library_ex;
pub use load_library_ex::{LoadLibraryExA, LoadLibraryExW};

mod load_library;
pub use load_library::{LoadLibraryA, LoadLibraryW};

mod find_resource;
pub use find_resource::{FindResourceA, FindResourceW};

mod create_act_ctx;
pub use create_act_ctx::{CreateActCtxA, CreateActCtxW};

mod begin_update_resource;
pub use begin_update_resource::{BeginUpdateResourceA, BeginUpdateResourceW};

mod activate_act_ctx;
pub use activate_act_ctx::ActivateActCtx;

mod free_resource;
pub use free_resource::FreeResource;

pub use crate::winapi::common::modules::main_module_path;

mod get_proc_address;
pub use get_proc_address::GetProcAddress;

mod load_resource;
pub use load_resource::LoadResource;

mod lock_resource;
pub use lock_resource::LockResource;

mod sizeof_resource;
pub use sizeof_resource::SizeofResource;

mod free_library;
pub use free_library::FreeLibrary;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "ActivateActCtx" => kernel32::ActivateActCtx(emu),
        "BeginUpdateResourceA" => kernel32::BeginUpdateResourceA(emu),
        "BeginUpdateResourceW" => kernel32::BeginUpdateResourceW(emu),
        "CreateActCtxA" => kernel32::CreateActCtxA(emu),
        "CreateActCtxW" => kernel32::CreateActCtxW(emu),
        "FindActCtxSectionStringA" => kernel32::FindActCtxSectionStringA(emu),
        "FindActCtxSectionStringW" => kernel32::FindActCtxSectionStringW(emu),
        "FindResourceA" => kernel32::FindResourceA(emu),
        "FindResourceW" => kernel32::FindResourceW(emu),
        "FreeLibrary" => kernel32::FreeLibrary(emu),
        "FreeResource" => kernel32::FreeResource(emu),
        "GetProcAddress" => kernel32::GetProcAddress(emu),
        "Module32First" => kernel32::Module32First(emu),
        "Module32FirstW" => kernel32::Module32FirstW(emu),
        "Module32Next" => kernel32::Module32Next(emu),
        "Module32NextW" => kernel32::Module32NextW(emu),
        "LoadLibraryA" => kernel32::LoadLibraryA(emu),
        "LoadLibraryExA" => kernel32::LoadLibraryExA(emu),
        "LoadLibraryExW" => kernel32::LoadLibraryExW(emu),
        "LoadLibraryW" => kernel32::LoadLibraryW(emu),
        "LoadResource" => kernel32::LoadResource(emu),
        "LockResource" => kernel32::LockResource(emu),
        "SizeofResource" => kernel32::SizeofResource(emu),
        _ => return false,
    }
    true
}
