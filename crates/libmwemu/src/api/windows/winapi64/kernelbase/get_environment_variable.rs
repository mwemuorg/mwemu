use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetEnvironmentVariableA(emu: &mut emu::Emu) {
    get_environment_variable(emu, "GetEnvironmentVariableA", false);
}

pub fn GetEnvironmentVariableW(emu: &mut emu::Emu) {
    get_environment_variable(emu, "GetEnvironmentVariableW", true);
}

fn get_environment_variable(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let name_ptr = api_try!(emu, call.arg(emu, 0));
    let buf_ptr = api_try!(emu, call.arg(emu, 1));
    let buf_size = api_try!(emu, call.arg(emu, 2));
    let name = strings::read_text(emu, name_ptr, wide);
    if wide {
        let value = match name.to_uppercase().as_str() {
            "SYSTEMROOT" | "WINDIR" => "C:\\Windows".to_string(),
            "TEMP" | "TMP" => "C:\\Users\\user\\AppData\\Local\\Temp".to_string(),
            "PROGRAMFILES" => "C:\\Program Files".to_string(),
            "PROGRAMFILES(X86)" => "C:\\Program Files (x86)".to_string(),
            "COMSPEC" => "C:\\Windows\\system32\\cmd.exe".to_string(),
            "USERPROFILE" => "C:\\Users\\user".to_string(),
            "APPDATA" => "C:\\Users\\user\\AppData\\Roaming".to_string(),
            "LOCALAPPDATA" => "C:\\Users\\user\\AppData\\Local".to_string(),
            "SYSTEMDRIVE" => "C:".to_string(),
            "PATH" => "C:\\Windows\\system32;C:\\Windows".to_string(),
            "COMPUTERNAME" => "DESKTOP-MWEMU".to_string(),
            "USERNAME" => "user".to_string(),
            "OS" => "Windows_NT".to_string(),
            "NUMBER_OF_PROCESSORS" => "4".to_string(),
            "PROCESSOR_ARCHITECTURE" => "x86".to_string(),
            _ => String::new(),
        };
        if value.is_empty() {
            emu.set_last_error(203);
            call.set_return(emu, 0);
        } else if buf_ptr != 0 && buf_size > 0 {
            strings::write_string(emu, buf_ptr, &value, wide);
            call.set_return(emu, value.len() as u64);
        } else {
            call.set_return(emu, value.len() as u64 + 1);
        }
        log_red!(emu, "kernelbase!{api} '{}' => '{}'", name, value);
    } else {
        let name_upper = name.to_uppercase();
        let value = match name_upper.as_str() {
            "PATH" => Some("C:\\Windows\\system32;C:\\Windows"),
            "TEMP" | "TMP" => Some("C:\\Users\\User\\AppData\\Local\\Temp"),
            "USERPROFILE" => Some("C:\\Users\\User"),
            "SYSTEMROOT" | "WINDIR" => Some("C:\\Windows"),
            "SYSTEMDRIVE" => Some("C:"),
            "COMPUTERNAME" => Some("DESKTOP-MWEMU"),
            "USERNAME" => Some("User"),
            "APPDATA" => Some("C:\\Users\\User\\AppData\\Roaming"),
            "LOCALAPPDATA" => Some("C:\\Users\\User\\AppData\\Local"),
            "PROGRAMFILES" => Some("C:\\Program Files"),
            "PROGRAMDATA" => Some("C:\\ProgramData"),
            "HOMEDRIVE" => Some("C:"),
            "HOMEPATH" => Some("\\Users\\User"),
            "COMSPEC" => Some("C:\\Windows\\system32\\cmd.exe"),
            "NUMBER_OF_PROCESSORS" => Some("4"),
            "PROCESSOR_ARCHITECTURE" => Some("AMD64"),
            "OS" => Some("Windows_NT"),
            _ => None,
        };
        if let Some(val) = value {
            log_red!(emu, "kernelbase!{api} '{}' => '{}'", name, val);
            if buf_ptr != 0 && buf_size > val.len() as u64 {
                strings::write_string(emu, buf_ptr, val, wide);
                call.set_return(emu, val.len() as u64);
            } else {
                call.set_return(emu, val.len() as u64 + 1);
            }
        } else {
            log_red!(emu, "kernelbase!{api} '{}' => NOT FOUND", name);
            emu.set_last_error(203); // ERROR_ENVVAR_NOT_FOUND
            call.set_return(emu, 0);
        }
    }
}
