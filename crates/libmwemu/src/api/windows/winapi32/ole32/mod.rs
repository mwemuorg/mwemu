use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod marshaling;
mod objects;
mod runtime;
mod storage;

pub use marshaling::*;
pub use objects::*;
pub use runtime::*;
pub use storage::*;

mod co_create_instance;
use co_create_instance::CoCreateInstance;
mod co_initialize;
use co_initialize::CoInitialize;
mod co_initialize_ex;
use co_initialize_ex::CoInitializeEx;
mod co_task_mem_alloc;
use co_task_mem_alloc::CoTaskMemAlloc;
mod co_task_mem_free;
use co_task_mem_free::CoTaskMemFree;
mod co_uninitialize;
use co_uninitialize::CoUninitialize;
mod ole_initialize;
use ole_initialize::OleInitialize;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if objects::dispatch(api, emu)
        || runtime::dispatch(api, emu)
        || marshaling::dispatch(api, emu)
        || storage::dispatch(api, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}
