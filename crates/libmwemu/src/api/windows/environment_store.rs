/*
    Environment block of the emulated process.

    Get/SetEnvironmentVariable, ExpandEnvironmentStrings and
    GetEnvironmentStrings share it, so a variable the process sets is seen
    again and one it never set is absent. It starts with the variables a
    Windows user session has, built from the configured user and host names
    and the same profile paths the shell folder APIs return. Names compare
    case-insensitively, like Windows.
*/

use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::config::Config;

const PROFILE: &str = "C:\\Users\\User";

lazy_static! {
    static ref VARIABLES: Mutex<Option<Vec<(String, String)>>> = Mutex::new(None);
}

fn defaults(cfg: &Config) -> Vec<(String, String)> {
    let local = format!("{}\\AppData\\Local", PROFILE);
    let temp = format!("{}\\Temp", local);
    [
        ("ALLUSERSPROFILE", "C:\\ProgramData".to_string()),
        ("APPDATA", format!("{}\\AppData\\Roaming", PROFILE)),
        ("CommonProgramFiles", "C:\\Program Files\\Common Files".to_string()),
        ("COMPUTERNAME", cfg.host_name.to_ascii_uppercase()),
        ("ComSpec", "C:\\Windows\\system32\\cmd.exe".to_string()),
        ("HOMEDRIVE", "C:".to_string()),
        ("HOMEPATH", "\\Users\\User".to_string()),
        ("LOCALAPPDATA", local),
        ("NUMBER_OF_PROCESSORS", "4".to_string()),
        ("OS", "Windows_NT".to_string()),
        (
            "Path",
            "C:\\Windows\\system32;C:\\Windows;C:\\Windows\\System32\\Wbem".to_string(),
        ),
        ("PATHEXT", ".COM;.EXE;.BAT;.CMD;.VBS;.JS;.WSF;.MSC".to_string()),
        ("PROCESSOR_ARCHITECTURE", "AMD64".to_string()),
        ("ProgramData", "C:\\ProgramData".to_string()),
        ("ProgramFiles", "C:\\Program Files".to_string()),
        ("ProgramFiles(x86)", "C:\\Program Files (x86)".to_string()),
        ("PUBLIC", "C:\\Users\\Public".to_string()),
        ("SystemDrive", "C:".to_string()),
        ("SystemRoot", "C:\\Windows".to_string()),
        ("TEMP", temp.clone()),
        ("TMP", temp),
        ("USERDOMAIN", cfg.host_name.to_ascii_uppercase()),
        ("USERNAME", cfg.user_name.clone()),
        ("USERPROFILE", PROFILE.to_string()),
        ("windir", "C:\\Windows".to_string()),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect()
}

fn with_variables<T>(cfg: &Config, f: impl FnOnce(&mut Vec<(String, String)>) -> T) -> T {
    let mut guard = VARIABLES.lock().unwrap();
    f(guard.get_or_insert_with(|| defaults(cfg)))
}

pub fn get(cfg: &Config, name: &str) -> Option<String> {
    with_variables(cfg, |vars| {
        vars.iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.clone())
    })
}

/// Set a variable; None removes it.
pub fn set(cfg: &Config, name: &str, value: Option<&str>) {
    with_variables(cfg, |vars| {
        let idx = vars.iter().position(|(k, _)| k.eq_ignore_ascii_case(name));
        match (idx, value) {
            (Some(i), Some(v)) => vars[i].1 = v.to_string(),
            (None, Some(v)) => vars.push((name.to_string(), v.to_string())),
            (Some(i), None) => {
                vars.remove(i);
            }
            (None, None) => {}
        }
    })
}

/// Every variable, in the order they were defined.
pub fn all(cfg: &Config) -> Vec<(String, String)> {
    with_variables(cfg, |vars| vars.clone())
}

/// Replace each `%NAME%` with its value; unknown names stay as written.
pub fn expand(cfg: &Config, text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find('%') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        match after.find('%') {
            Some(end) => {
                let name = &after[..end];
                match get(cfg, name).filter(|_| !name.is_empty()) {
                    Some(value) => out.push_str(&value),
                    None => {
                        out.push('%');
                        out.push_str(name);
                        out.push('%');
                    }
                }
                rest = &after[end + 1..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}
