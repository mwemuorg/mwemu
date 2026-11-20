//! Guest-call diagnostics belong at the ABI boundary, outside the memory operations.
use std::fmt::Write;

use crate::api::abi::windows::{
    ArgumentError,
    CallingConvention::{Stdcall, Thiscall},
    WindowsAbi,
    WindowsAbi::{WinApi, WinInterrupt, WinSyscall, WinSysenter},
};
use crate::emu::Emu;
use crate::winapi::common::memory::read_array;
use crate::winapi::environment_store::Encoding;
use crate::winapi::signatures;

/// Format a NUL-terminated input for a trace without changing guest fault state.
pub(crate) fn input_string(emu: &Emu, address: u64, encoding: Encoding) -> String {
    const MAX_TRACE_UNITS: usize = 128;
    if address == 0 {
        return "NULL".into();
    }
    let mut units = Vec::new();
    for index in 0..=MAX_TRACE_UNITS {
        let unit = address
            .checked_add((index * encoding.width()) as u64)
            .and_then(|at| {
                if matches!(encoding, Encoding::Wide) {
                    read_array(emu, at).map(u16::from_le_bytes)
                } else {
                    emu.maps
                        .read_byte(at)
                        .map(|byte| encoding.decode_byte(byte))
                }
            });
        match unit {
            Some(0) => return format!("{:?}", String::from_utf16_lossy(&units)),
            Some(value) if index < MAX_TRACE_UNITS => units.push(value),
            Some(_) => break,
            None => {
                return format!(
                    "<unreadable at {address:#x}+{:#x}>",
                    index * encoding.width()
                );
            }
        }
    }
    format!("{:?}…", String::from_utf16_lossy(&units))
}

fn write_value(arguments: &mut String, value: Result<u64, ArgumentError>) {
    match value {
        Ok(value) => {
            let _ = write!(arguments, "{value:#x}");
        }
        Err(error) => {
            let _ = write!(arguments, "<{:?}>", error.fault);
        }
    }
}

pub(in crate::api::windows) enum Parameter {
    Word(&'static str),
    AnsiString(&'static str),
    Pointer(&'static str),
    Dword(&'static str),
    Byte(&'static str),
    WideString(&'static str),
    Qword(&'static str),
}

fn fallback_parameters(library: &str, name: &str) -> Option<&'static [Parameter]> {
    use Parameter::{Byte, Dword, Pointer, Qword, WideString};

    let library = library.trim_end_matches(".dll");
    if matches!(library, "ntdll" | "kernel32" | "kernelbase" | "ntoskrnl")
        && let Some(native) = name.strip_prefix("Nt").or_else(|| name.strip_prefix("Zw"))
    {
        return match native {
            "SetInformationProcess" => Some(&[
                Pointer("process_handle"),
                Dword("process_information_class"),
                Pointer("process_information"),
                Dword("process_information_length"),
            ]),
            "QueryInformationProcess" => Some(&[
                Pointer("process_handle"),
                Dword("process_information_class"),
                Pointer("process_information"),
                Dword("process_information_length"),
                Pointer("return_length"),
            ]),
            "QuerySystemInformation" => Some(&[
                Dword("system_information_class"),
                Pointer("system_information"),
                Dword("system_information_length"),
                Pointer("return_length"),
            ]),
            _ => None,
        };
    }
    match (library, name) {
        (
            "advapi32" | "ntdll" | "kernel32" | "kernelbase",
            "EtwEventUnregister" | "EventUnregister",
        ) => Some(&[Qword("reg_handle")]),
        ("libgcc_s_dw2-1", "__register_frame_info") => Some(&[Pointer("begin"), Pointer("object")]),
        ("libgcc_s_dw2-1", "__deregister_frame_info") => Some(&[Pointer("begin")]),
        ("advapi32", "BaseRegGetVersion") => Some(&[Pointer("hkey"), Pointer("version")]),
        ("advapi32", "BaseRegUnLoadKey") => Some(&[Pointer("hkey"), Pointer("subkey")]),
        ("advapi32", "BaseRegLoadKey") => {
            Some(&[Pointer("hkey"), Pointer("subkey"), Pointer("file")])
        }
        ("advapi32", "BaseRegRestoreKey") => {
            Some(&[Pointer("hkey"), Pointer("file"), Dword("flags")])
        }
        ("advapi32", "BaseRegSaveKeyEx") => Some(&[
            Pointer("hkey"),
            Pointer("file"),
            Pointer("security_attributes"),
            Dword("flags"),
        ]),
        ("advapi32", "BaseRegSetKeySecurity") => Some(&[
            Pointer("hkey"),
            Dword("security_information"),
            Pointer("security_descriptor"),
        ]),
        ("advapi32", "BaseRegSetValue") => Some(&[
            Pointer("hkey"),
            Pointer("value_name"),
            Dword("value_type"),
            Pointer("data"),
            Dword("data_size"),
        ]),
        ("advapi32", "BaseRegCreateKey") => Some(&[
            Pointer("hkey"),
            Pointer("subkey"),
            Pointer("class"),
            Dword("options"),
            Dword("desired_access"),
            Pointer("security_attributes"),
            Pointer("result"),
            Pointer("disposition"),
        ]),
        ("advapi32", "EncryptionDisable") => Some(&[WideString("dir_path"), Dword("disable")]),
        ("advapi32", "LsaDelete") => Some(&[Pointer("object")]),
        ("advapi32", "LsaDeleteTrustedDomain") => {
            Some(&[Pointer("policy_handle"), Pointer("trusted_domain_sid")])
        }
        ("advapi32", "LsaRemoveAccountRights") => Some(&[
            Pointer("policy_handle"),
            Pointer("account_sid"),
            Byte("all_rights"),
            Pointer("user_rights"),
            Dword("count_of_rights"),
        ]),
        ("advapi32", "LsaRemovePrivilegesFromAccount") => Some(&[
            Pointer("account_handle"),
            Byte("all_privileges"),
            Pointer("privileges"),
        ]),
        ("advapi32", "PerfDeleteCounters") => Some(&[
            Pointer("query"),
            Pointer("counters"),
            Dword("counters_size"),
        ]),
        ("advapi32", "PerfDeleteInstance") => Some(&[Pointer("provider"), Pointer("instance")]),
        ("advapi32", "RemoveTraceCallback") => Some(&[Pointer("guid")]),
        ("advapi32", "RemoveUsersFromEncryptedFile") => {
            Some(&[WideString("file_name"), Pointer("hashes")])
        }
        ("advapi32", "UninstallApplication") => {
            Some(&[WideString("product_code"), Dword("status")])
        }
        ("advapi32", "UnregisterTraceGuids") => Some(&[Qword("registration_handle")]),
        ("crypt32", "CryptSIPAddProvider") => Some(&[Pointer("provider")]),
        ("crypt32", "CryptSIPRemoveProvider") => Some(&[Pointer("provider_id")]),
        ("advapi32" | "cryptbase", "SystemFunction040" | "SystemFunction041") => Some(&[
            Pointer("memory"),
            Dword("memory_size"),
            Dword("option_flags"),
        ]),
        ("advapi32" | "ntdll", "MD4Init" | "MD4Final" | "MD5Init" | "MD5Final") => {
            Some(&[Pointer("context")])
        }
        ("advapi32" | "ntdll", "MD4Update" | "MD5Update") => {
            Some(&[Pointer("context"), Pointer("input"), Dword("length")])
        }
        ("shell32", "DllGetVersion") => Some(&[Pointer("version_info")]),
        ("winmm" | "kernel32" | "kernelbase", "timeBeginPeriod" | "timeEndPeriod") => {
            Some(&[Dword("period")])
        }
        _ => None,
    }
}

fn format_parameters(emu: &Emu, call: WindowsAbi, parameters: &[Parameter]) -> String {
    let is64 = emu.cfg.arch.is_64bits();
    let mut arguments = String::new();
    let mut slot = 0;
    for parameter in parameters {
        if slot != 0 {
            arguments.push_str(", ");
        }
        let (label, result, words) = match parameter {
            Parameter::Word(name) => (name, call.arg_u16(emu, slot).map(u64::from), 1),
            Parameter::Byte(name) => (name, call.arg_u8(emu, slot).map(u64::from), 1),
            Parameter::WideString(name) | Parameter::AnsiString(name) => {
                let _ = write!(arguments, "{name}=");
                match call.arg(emu, slot) {
                    Ok(address) => arguments.push_str(&input_string(
                        emu,
                        address,
                        Encoding::from_wide(matches!(parameter, Parameter::WideString(_))),
                    )),
                    Err(error) => write_value(&mut arguments, Err(error)),
                }
                slot += 1;
                continue;
            }
            Parameter::Pointer(name) => (name, call.arg(emu, slot), 1),
            Parameter::Dword(name) => (name, call.arg_u32(emu, slot).map(u64::from), 1),
            Parameter::Qword(name) => (name, call.arg_u64(emu, slot), 2),
        };
        let _ = write!(arguments, "{label}=");
        write_value(&mut arguments, result);
        slot += if is64 { 1 } else { words };
    }
    arguments
}

pub(crate) fn export(emu: &Emu, call: WindowsAbi, library: &str, name: &str) {
    if emu.cfg.verbose < 1 || !log::log_enabled!(log::Level::Trace) {
        return;
    }
    if let Some(parameters) = signatures::lookup(library, name).and_then(|s| s.parameters) {
        let arguments = format_parameters(emu, call, parameters);
        log_red!(emu, "{library}!{name}({arguments})");
    } else {
        log_red!(emu, "{library}!{name}");
    }
}

pub(crate) fn unsupported(
    emu: &Emu,
    call: WindowsAbi,
    library: &str,
    name: &str,
    parameter_count: Option<u32>,
    x86_words: u32,
) {
    let is64 = emu.cfg.arch.is_64bits();
    if let Some(parameters) =
        fallback_parameters(library, name).or_else(|| signatures::lookup(library, name)?.parameters)
    {
        let arguments = format_parameters(emu, call, parameters);
        log::trace!("unsupported {library}!{name}({arguments})");
    } else {
        match parameter_count {
            Some(count) if !is64 => log::trace!(
                "unsupported {library}!{name} ({count} parameters, {x86_words} x86 stack DWORDs)"
            ),
            Some(count) => log::trace!("unsupported {library}!{name} ({count} parameters)"),
            None if !is64 && matches!(call, WinApi(Stdcall | Thiscall)) => {
                log::trace!("unsupported {library}!{name} (x86 cleanup={x86_words} stack DWORDs)")
            }
            None => log::trace!("unsupported {library}!{name}"),
        }
    }
}

pub(crate) fn api(
    emu: &mut Emu,
    name: &str,
    parameters: &str,
    returns_value: bool,
    handler: impl FnOnce(&mut Emu),
) {
    let call = WinApi(Stdcall);
    invoke(emu, call, name, parameters, returns_value, handler);
}

pub(crate) fn invoke(
    emu: &mut Emu,
    call: WindowsAbi,
    name: &str,
    parameters: &str,
    returns_value: bool,
    handler: impl FnOnce(&mut Emu),
) {
    if !log::log_enabled!(log::Level::Trace) || (matches!(call, WinApi(_)) && emu.cfg.verbose < 1) {
        handler(emu);
        return;
    }
    // Capture inputs before stdcall cleanup, output writes or return-register updates.
    let mut arguments = String::new();
    for (index, parameter) in parameters.split_whitespace().enumerate() {
        if index != 0 {
            arguments.push_str(", ");
        }
        let _ = write!(&mut arguments, "{parameter}=");
        write_value(&mut arguments, call.arg(emu, index));
    }
    let number = emu.regs().rax;
    handler(emu);
    let result = if emu.api_policy.stopped() {
        " (stopped)".to_string()
    } else if returns_value {
        format!(" = 0x{:x}", emu.regs().rax)
    } else {
        String::new()
    };
    match call {
        WinApi(_) => {
            log_red!(emu, "{}({}){}", name, arguments, result);
        }
        WinSyscall | WinSysenter | WinInterrupt => {
            log_orange!(
                emu,
                "syscall 0x{:x}: {}({}){}",
                number,
                name,
                arguments,
                result
            );
        }
    }
}
