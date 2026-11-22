use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetConsoleCP" => kernelbase::GetConsoleCP(emu),
        "GetConsoleInputExeNameA" => kernelbase::GetConsoleInputExeNameA(emu),
        "GetConsoleInputExeNameW" => kernelbase::GetConsoleInputExeNameW(emu),
        "GetConsoleMode" => kernelbase::GetConsoleMode(emu),
        "GetConsoleOutputCP" => kernelbase::GetConsoleOutputCP(emu),
        "ReadConsoleInputExA" => kernelbase::ReadConsoleInputExA(emu),
        "ReadConsoleInputExW" => kernelbase::ReadConsoleInputExW(emu),
        "SetConsoleInputExeNameA" => kernelbase::SetConsoleInputExeNameA(emu),
        "SetConsoleInputExeNameW" => kernelbase::SetConsoleInputExeNameW(emu),
        "SetConsoleMode" => kernelbase::SetConsoleMode(emu),
        "SetLastConsoleEventActive" => kernelbase::SetLastConsoleEventActive(emu),
        "WriteConsoleA" => kernelbase::WriteConsoleA(emu),
        "WriteConsoleW" => kernelbase::WriteConsoleW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
