use crate::api::windows::common::fallback;
use crate::api::windows::common::memory as memory_api;
use crate::emu;
use crate::winapi::common::heap as heap_api;
use crate::winapi::common::memory_ops;
use crate::winapi::common::sections;
use crate::winapi::common::{files, handles, modules, strings, threads};
use crate::winapi::winapi32::kernel32;

mod get_token_information;
use get_token_information::GetTokenInformation;
mod load_string_w;
use load_string_w::LoadStringW;
mod local_alloc;
use local_alloc::LocalAlloc;
mod path_combine;
use path_combine::PathCombineA;
use path_combine::PathCombineW;
mod set_unhandled_exception_filter;
use set_unhandled_exception_filter::SetUnhandledExceptionFilter;
mod _initterm;
use _initterm::_initterm;
mod _initterm_e;
use _initterm_e::_initterm_e;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if sections::dispatch(api, emu) {
        return String::new();
    }
    if memory_ops::dispatch(api, emu) {
        return String::new();
    }
    if heap_api::dispatch(api, emu) {
        return String::new();
    }
    if memory_api::dispatch(api, emu) {
        return String::new();
    }
    if modules::dispatch(api, "kernelbase", emu)
        || files::dispatch(api, "kernelbase", emu)
        || strings::dispatch(api, "kernelbase", emu)
        || handles::dispatch(api, "kernelbase", emu)
        || threads::dispatch(api, "kernelbase", emu)
    {
        return String::new();
    }
    match api {
        "FlushInstructionCache" => {
            memory_api::flush_instruction_cache(emu, "kernelbase!FlushInstructionCache")
        }

        "LoadStringW" => LoadStringW(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "_initterm" => _initterm(emu),
        "_initterm_e" => _initterm_e(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "GetTokenInformation" => GetTokenInformation(emu),
        "PathCombineA" => PathCombineA(emu),
        "PathCombineW" => PathCombineW(emu),

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}
