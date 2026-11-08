use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod get_module_handle_w;
use get_module_handle_w::GetModuleHandleW;
mod get_token_information;
use get_token_information::GetTokenInformation;
mod is_char_alpha_numeric_a;
use is_char_alpha_numeric_a::IsCharAlphaNumericA;
mod load_string_w;
use load_string_w::LoadStringW;
mod local_alloc;
use local_alloc::LocalAlloc;
mod path_combine_a;
use path_combine_a::PathCombineA;
mod path_combine_w;
use path_combine_w::PathCombineW;
mod set_unhandled_exception_filter;
use set_unhandled_exception_filter::SetUnhandledExceptionFilter;
mod _initterm;
use _initterm::_initterm;
mod _initterm_e;
use _initterm_e::_initterm_e;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "GetModuleHandleW" => GetModuleHandleW(emu),
        "LoadStringW" => LoadStringW(emu),
        "SetUnhandledExceptionFilter" => SetUnhandledExceptionFilter(emu),
        "_initterm" => _initterm(emu),
        "_initterm_e" => _initterm_e(emu),
        "LocalAlloc" => LocalAlloc(emu),
        "GetTokenInformation" => GetTokenInformation(emu),
        "PathCombineA" => PathCombineA(emu),
        "PathCombineW" => PathCombineW(emu),
        "IsCharAlphaNumericA" => IsCharAlphaNumericA(emu),

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}
