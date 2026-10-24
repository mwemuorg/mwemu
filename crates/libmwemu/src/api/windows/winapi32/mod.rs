pub mod advapi32;
mod bcrypt;
mod crypt32;
mod dnsapi;
mod iphlpapi;
pub mod kernel32;
mod kernelbase;
mod libgcc;
mod mscoree;
mod msvcrt;
pub mod ntdll;
mod ntoskrnl;
mod ole32;
mod oleaut32;
mod shell32;
mod shlwapi;
mod urlmon;
mod user32;
mod wincrt;
mod winhttp;
mod wininet;
mod ws2_32;

use crate::emu;

/// The `i`-th (0-based) stdcall argument of the API being emulated. The
/// gateway already consumed the return address, so arguments start at `esp`.
pub(crate) fn arg(emu: &emu::Emu, i: u64) -> u32 {
    emu.maps
        .read_dword(emu.regs().get_esp() + i * 4)
        .unwrap_or(0)
}

/// stdcall: the callee removes its `n` arguments from the stack.
pub(crate) fn stdcall_ret(emu: &mut emu::Emu, n: u32) {
    for _ in 0..n {
        emu.stack_pop32(false);
    }
}

pub fn gateway(addr: u32, name: &str, emu: &mut emu::Emu) {
    emu.regs_mut().sanitize32();
    match name {
        "kernel32.text" => kernel32::gateway(addr, emu),
        "kernel32.rdata" => kernel32::gateway(addr, emu),
        "ntdll.text" => ntdll::gateway(addr, emu),
        "user32.text" => user32::gateway(addr, emu),
        "ws2_32.text" => ws2_32::gateway(addr, emu),
        "wininet.text" => wininet::gateway(addr, emu),
        "advapi32.text" => advapi32::gateway(addr, emu),
        "bcrypt.text" => bcrypt::gateway(addr, emu),
        "ole32.text" => ole32::gateway(addr, emu),
        "winhttp.text" => winhttp::gateway(addr, emu),
        "crypt32.text" => crypt32::gateway(addr, emu),
        "dnsapi.text" => dnsapi::gateway(addr, emu),
        "mscoree.text" => mscoree::gateway(addr, emu),
        "msvcrt.text" => msvcrt::gateway(addr, emu),
        "shell32.text" => shell32::gateway(addr, emu),
        "shlwapi.text" => shlwapi::gateway(addr, emu),
        "urlmon.text" => urlmon::gateway(addr, emu),
        "oleaut32.text" => oleaut32::gateway(addr, emu),
        "kernelbase.text" => kernelbase::gateway(addr, emu),
        "iphlpapi.text" => iphlpapi::gateway(addr, emu),
        "libgcc_s_dw2-1.text" => libgcc::gateway(addr, emu),
        "api-ms-win-crt-runtime-l1-1-0.text" => wincrt::gateway(addr, emu),
        "ntoskrnl.text" => ntoskrnl::gateway(addr, emu),
        "ntoskrnlPAGE" => ntoskrnl::gateway(addr, emu),
        "not_loaded" => {
            // TODO: banzai check?
            emu.pe32.as_ref().unwrap().import_addr_to_name(addr)
        }
        _ => {
            log::error!(
                "/!\\ winapi32 gateway: trying to execute on {} at 0x{:x}",
                name,
                addr
            );
            return;
        }
    };
    emu.call_stack_mut().pop();
}
