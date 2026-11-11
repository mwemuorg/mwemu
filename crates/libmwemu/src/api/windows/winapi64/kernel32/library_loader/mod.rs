use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../load_library_ex.rs"]
mod load_library_ex;
pub use load_library_ex::*;

#[path = "../load_library.rs"]
mod load_library;
pub use load_library::*;

#[path = "../find_resource.rs"]
mod find_resource;
pub use find_resource::*;

#[path = "../create_act_ctx.rs"]
mod create_act_ctx;
pub use create_act_ctx::*;

#[path = "../begin_update_resource.rs"]
mod begin_update_resource;
pub use begin_update_resource::*;

#[path = "../activate_act_ctx.rs"]
mod activate_act_ctx;
pub use activate_act_ctx::*;

#[path = "../free_resource.rs"]
mod free_resource;
pub use free_resource::*;

#[path = "../get_module_file_name_w.rs"]
mod get_module_file_name_w;
pub use get_module_file_name_w::*;

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

#[path = "../free_library.rs"]
mod free_library;
pub use free_library::*;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "ActivateActCtx" => kernel32::ActivateActCtx(emu),
        "BeginUpdateResourceA" => kernel32::BeginUpdateResourceA(emu),
        "BeginUpdateResourceW" => kernel32::BeginUpdateResourceW(emu),
        "CreateActCtxA" => kernel32::CreateActCtxA(emu),
        "CreateActCtxW" => kernel32::CreateActCtxW(emu),
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
