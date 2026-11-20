//! Reviewed SDK prototypes for diagnostics and unsupported calls.
//! An x86 stack word is four bytes. A by-value argument can take several words.
use super::common::trace::Parameter;
use super::policy::ReturnType;

pub(super) struct Signature {
    pub result: ReturnType,
    pub arguments: u32,
    pub x86_words: u32,
    pub parameters: Option<&'static [Parameter]>,
}

macro_rules! signature {
    ($result:ident, $count:literal, $words:literal) => {
        Some(Signature { result: ReturnType::$result, arguments: $count, x86_words: $words, parameters: None })
    };
    ($result:ident, $count:literal, $words:literal; $($kind:ident($name:ident)),* $(,)?) => {
        Some(Signature {
            result: ReturnType::$result,
            arguments: $count,
            x86_words: $words,
            parameters: Some(&[$(Parameter::$kind(stringify!($name))),*]),
        })
    };
}

mod advapi32;
mod bcrypt;
mod crypt32;
mod gdi32;
mod kernel32;
mod kernelbase;
mod msvcrt;
mod ole32;
mod oleaut32;
mod shell32;
mod user32;
mod wininet;
mod ws2_32;

pub(super) fn lookup(library: &str, name: &str) -> Option<Signature> {
    match library.trim_end_matches(".dll") {
        "urlmon" if matches!(name, "URLDownloadToFileA" | "URLDownloadToFileW") => {
            if name.ends_with('W') {
                signature!(Hresult, 5, 5; Pointer(caller), WideString(url), WideString(file_name), Dword(reserved), Pointer(callback))
            } else {
                signature!(Hresult, 5, 5; Pointer(caller), AnsiString(url), AnsiString(file_name), Dword(reserved), Pointer(callback))
            }
        }
        "dnsapi" if matches!(name, "DnsFree" | "DnsRecordListFree") => {
            signature!(Void, 2, 2; Pointer(data), Dword(free_type))
        }
        "advapi32" => advapi32::lookup(name),
        "bcrypt" => bcrypt::lookup(name),
        "crypt32" => crypt32::lookup(name),
        "gdi32" => gdi32::lookup(name),
        "kernel32" => kernel32::lookup(name),
        "kernelbase" => kernelbase::lookup(name),
        "msvcrt" | "ucrtbase" => msvcrt::lookup(name),
        "ole32" => ole32::lookup(name),
        "oleaut32" => oleaut32::lookup(name),
        "shell32" => shell32::lookup(name),
        "user32" => user32::lookup(name),
        "wininet" => wininet::lookup(name),
        "ws2_32" => ws2_32::lookup(name),
        _ => None,
    }
}
