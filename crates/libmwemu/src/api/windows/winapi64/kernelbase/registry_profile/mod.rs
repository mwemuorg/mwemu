use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernelbase;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "RegCloseKey" => kernelbase::RegCloseKey(emu),
        "RegCreateKeyExA" => kernelbase::RegCreateKeyExA(emu),
        "RegCreateKeyExW" => kernelbase::RegCreateKeyExW(emu),
        "RegDeleteKeyExA" => kernelbase::RegDeleteKeyExA(emu),
        "RegDeleteKeyExW" => kernelbase::RegDeleteKeyExW(emu),
        "RegDeleteTreeA" => kernelbase::RegDeleteTreeA(emu),
        "RegDeleteTreeW" => kernelbase::RegDeleteTreeW(emu),
        "RegDeleteValueA" => kernelbase::RegDeleteValueA(emu),
        "RegDeleteValueW" => kernelbase::RegDeleteValueW(emu),
        "RegEnumKeyExA" => kernelbase::RegEnumKeyExA(emu),
        "RegEnumKeyExW" => kernelbase::RegEnumKeyExW(emu),
        "RegEnumValueA" => kernelbase::RegEnumValueA(emu),
        "RegEnumValueW" => kernelbase::RegEnumValueW(emu),
        "RegFlushKey" => kernelbase::RegFlushKey(emu),
        "RegGetValueA" => kernelbase::RegGetValueA(emu),
        "RegGetValueW" => kernelbase::RegGetValueW(emu),
        "RegOpenCurrentUser" => kernelbase::RegOpenCurrentUser(emu),
        "RegOpenKeyExA" => kernelbase::RegOpenKeyExA(emu),
        "RegOpenKeyExW" => kernelbase::RegOpenKeyExW(emu),
        "RegQueryInfoKeyA" => kernelbase::RegQueryInfoKeyA(emu),
        "RegQueryInfoKeyW" => kernelbase::RegQueryInfoKeyW(emu),
        "RegQueryValueExA" => kernelbase::RegQueryValueExA(emu),
        "RegQueryValueExW" => kernelbase::RegQueryValueExW(emu),
        "RegSetValueExA" => kernelbase::RegSetValueExA(emu),
        "RegSetValueExW" => kernelbase::RegSetValueExW(emu),
        "RegisterApplicationRestart" => kernelbase::RegisterApplicationRestart(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
