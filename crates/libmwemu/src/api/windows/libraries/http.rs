use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
mod urls;
pub use urls::{
    InternetCanonicalizeUrlA, InternetCanonicalizeUrlW, InternetCombineUrlA, InternetCombineUrlW,
};

use crate::emu::Emu;
use crate::emu::object_handle::internet::{InternetLibrary, InternetObject};
use crate::winapi::common::fallback::unsupported_exports;
use crate::winapi::common::trace;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

fn result(emu: &mut Emu, arguments: usize, value: Result<u64, u64>) {
    let call = WinApi(Stdcall);
    let value = value.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, arguments);
}

fn session(emu: &mut Emu, api: &str, library: InternetLibrary, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [agent, access, proxy, bypass, flags] = api_try!(emu, call.args(emu));
    let agent = trace::input_string(emu, agent, encoding);
    log_red!(
        emu,
        "{api} agent={agent} access_type={access:#x} proxy={proxy:#x} bypass={bypass:#x} flags={flags:#x}"
    );
    let session = emu
        .handle_management
        .open_internet(library, InternetObject::Session, None);
    result(emu, 5, Ok(session));
}

pub fn WinHttpOpen(emu: &mut Emu) {
    session(
        emu,
        "winhttp!WinHttpOpen",
        InternetLibrary::WinHttp,
        Encoding::Wide,
    );
}

pub fn InternetOpenA(emu: &mut Emu) {
    session(
        emu,
        "wininet!InternetOpenA",
        InternetLibrary::WinInet,
        Encoding::Ansi,
    );
}

pub fn InternetOpenW(emu: &mut Emu) {
    session(
        emu,
        "wininet!InternetOpenW",
        InternetLibrary::WinInet,
        Encoding::Wide,
    );
}

fn text(emu: &Emu, ptr: u64, encoding: Encoding) -> Option<String> {
    String::from_utf16(&encoding.read(emu, ptr).ok()?).ok()
}

fn connection(emu: &mut Emu, api: &str, library: InternetLibrary, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [parent, server, port] = api_try!(emu, call.args(emu));
    let (count, reserved) = if library == InternetLibrary::WinHttp {
        (4, api_try!(emu, call.arg_u32(emu, 3)))
    } else {
        let service = api_try!(emu, call.arg_u32(emu, 5));
        let _context = api_try!(emu, call.arg(emu, 7));
        if service != INTERNET_SERVICE_HTTP {
            policy::unsupported(emu, "wininet", api, emu.pc());
            return;
        }
        (8, 0)
    };
    let server = text(emu, server, encoding);
    let port = port as u16;
    let module = if library == InternetLibrary::WinHttp {
        "winhttp"
    } else {
        "wininet"
    };
    log_red!(
        emu,
        "{module}!{api} session={parent:#x} server={server:?} port={port}"
    );
    let value = match (emu.handle_management.internet(parent, library), server) {
        (None, _) => Err(ERROR_INVALID_HANDLE),
        (Some(InternetObject::Session), Some(server)) if !server.is_empty() && reserved == 0 => {
            Ok(emu.handle_management.open_internet(
                library,
                InternetObject::Connection { server, port },
                Some(parent),
            ))
        }
        (Some(InternetObject::Session), _) => Err(ERROR_INVALID_PARAMETER),
        _ => Err(ERROR_INTERNET_INCORRECT_HANDLE_TYPE),
    };
    result(emu, count, value);
}

pub fn WinHttpConnect(emu: &mut Emu) {
    connection(
        emu,
        "WinHttpConnect",
        InternetLibrary::WinHttp,
        Encoding::Wide,
    );
}

pub fn InternetConnectA(emu: &mut Emu) {
    connection(
        emu,
        "InternetConnectA",
        InternetLibrary::WinInet,
        Encoding::Ansi,
    );
}

pub fn InternetConnectW(emu: &mut Emu) {
    connection(
        emu,
        "InternetConnectW",
        InternetLibrary::WinInet,
        Encoding::Wide,
    );
}

fn request(emu: &mut Emu, api: &str, library: InternetLibrary, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [parent, verb, path, _version, _referrer, accept_types, flags] =
        api_try!(emu, call.args(emu));
    let count = if library == InternetLibrary::WinHttp {
        7
    } else {
        let _context = api_try!(emu, call.arg(emu, 7));
        8
    };
    let verb = trace::input_string(emu, verb, encoding);
    let path = if path == 0 {
        Some(String::new())
    } else {
        text(emu, path, encoding)
    };
    log_red!(
        emu,
        "{api} connection={parent:#x} verb={verb} path={path:?} accept_types={accept_types:#x} flags={flags:#x}"
    );
    let value = match (emu.handle_management.internet(parent, library), path) {
        (None, _) => Err(ERROR_INVALID_HANDLE),
        (Some(InternetObject::Connection { server, port }), Some(path)) => {
            let (scheme, default_port) = if flags & INTERNET_FLAG_SECURE != 0 {
                ("https", INTERNET_DEFAULT_HTTPS_PORT)
            } else {
                ("http", INTERNET_DEFAULT_HTTP_PORT)
            };
            let suffix = if *port == 0 || *port == default_port {
                String::new()
            } else {
                format!(":{port}")
            };
            let slash = if path.starts_with('/') { "" } else { "/" };
            let url = format!("{scheme}://{server}{suffix}{slash}{path}");
            Ok(emu.handle_management.open_internet(
                library,
                InternetObject::Request { url },
                Some(parent),
            ))
        }
        (Some(InternetObject::Connection { .. }), None) => Err(ERROR_INVALID_PARAMETER),
        _ => Err(ERROR_INTERNET_INCORRECT_HANDLE_TYPE),
    };
    result(emu, count, value);
}

pub fn WinHttpOpenRequest(emu: &mut Emu) {
    request(
        emu,
        "winhttp!WinHttpOpenRequest",
        InternetLibrary::WinHttp,
        Encoding::Wide,
    );
}

pub fn HttpOpenRequestA(emu: &mut Emu) {
    request(
        emu,
        "wininet!HttpOpenRequestA",
        InternetLibrary::WinInet,
        Encoding::Ansi,
    );
}

pub fn HttpOpenRequestW(emu: &mut Emu) {
    request(
        emu,
        "wininet!HttpOpenRequestW",
        InternetLibrary::WinInet,
        Encoding::Wide,
    );
}

fn close(emu: &mut Emu, api: &str, library: InternetLibrary) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{api} handle={handle:#x}");
    let closed = emu.handle_management.close_internet(handle, library);
    result(emu, 1, closed.then_some(1).ok_or(ERROR_INVALID_HANDLE));
}

pub fn WinHttpCloseHandle(emu: &mut Emu) {
    close(emu, "winhttp!WinHttpCloseHandle", InternetLibrary::WinHttp);
}

pub fn InternetCloseHandle(emu: &mut Emu) {
    close(emu, "wininet!InternetCloseHandle", InternetLibrary::WinInet);
}

fn query_option(emu: &mut Emu, api: &str, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let [handle, option, buffer, length] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "wininet!{api} handle={handle:#x} option={option} buffer={buffer:#x} length={length:#x}"
    );
    let Some(object) = emu
        .handle_management
        .internet(handle, InternetLibrary::WinInet)
    else {
        if handle == 0 {
            policy::unsupported(emu, "wininet", api, emu.pc());
        } else {
            result(emu, 4, Err(ERROR_INVALID_HANDLE));
        }
        return;
    };
    let (bytes, terminator) = match (option as u32, object) {
        (INTERNET_OPTION_HANDLE_TYPE, object) => {
            let kind = match object {
                InternetObject::Session => INTERNET_HANDLE_TYPE_INTERNET,
                InternetObject::Connection { .. } => INTERNET_HANDLE_TYPE_CONNECT_HTTP,
                InternetObject::Request { .. } => INTERNET_HANDLE_TYPE_HTTP_REQUEST,
            };
            (kind.to_le_bytes().to_vec(), 0)
        }
        (INTERNET_OPTION_URL, InternetObject::Request { url }) => {
            let units: Vec<_> = url.encode_utf16().chain([0]).collect();
            (encoding.encode(&units), encoding.width())
        }
        _ => {
            policy::unsupported(emu, "wininet", api, emu.pc());
            return;
        }
    };
    let needed = bytes.len() as u32;
    let value = match emu.maps.read_dword(length) {
        None => Err(ERROR_INVALID_PARAMETER),
        Some(capacity) if buffer == 0 || capacity < needed => {
            if emu.maps.write_dword(length, needed) {
                Err(ERROR_INSUFFICIENT_BUFFER)
            } else {
                Err(ERROR_INVALID_PARAMETER)
            }
        }
        Some(_)
            if emu.maps.write_bytes(buffer, &bytes)
                && emu.maps.write_dword(length, needed - terminator as u32) =>
        {
            Ok(1)
        }
        _ => Err(ERROR_INVALID_PARAMETER),
    };
    result(emu, 4, value);
}

pub fn InternetQueryOptionA(emu: &mut Emu) {
    query_option(emu, "InternetQueryOptionA", Encoding::Ansi);
}

pub fn InternetQueryOptionW(emu: &mut Emu) {
    query_option(emu, "InternetQueryOptionW", Encoding::Wide);
}

pub fn WinHttpSendRequest(emu: &mut Emu) {
    policy::unsupported(emu, "winhttp", "WinHttpSendRequest", emu.pc());
}

pub fn WinHttpReceiveResponse(emu: &mut Emu) {
    policy::unsupported(emu, "winhttp", "WinHttpReceiveResponse", emu.pc());
}

pub fn WinHttpReadData(emu: &mut Emu) {
    policy::unsupported(emu, "winhttp", "WinHttpReadData", emu.pc());
}

pub fn InternetReadFile(emu: &mut Emu) {
    policy::unsupported(emu, "wininet", "InternetReadFile", emu.pc());
}

pub fn InternetReadFileExA(emu: &mut Emu) {
    policy::unsupported(emu, "wininet", "InternetReadFileExA", emu.pc());
}

pub fn InternetReadFileExW(emu: &mut Emu) {
    policy::unsupported(emu, "wininet", "InternetReadFileExW", emu.pc());
}

unsupported_exports!(
    "wininet",
    InternetSetOptionA,
    InternetSetOptionW,
    HttpQueryInfoA,
    HttpQueryInfoW,
    HttpSendRequestA,
    HttpSendRequestW,
    InternetErrorDlg,
    InternetCrackUrlA,
    InternetCrackUrlW
);

pub fn CreateUrlCacheGroup(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [flags, reserved] = api_try!(emu, call.args(emu));
    let group = if reserved == 0
        && flags as u32 & !(CACHEGROUP_FLAG_NONPURGEABLE | CACHEGROUP_FLAG_GIDONLY) == 0
    {
        emu.handle_management.create_cache_group()
    } else {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    };
    log_red!(
        emu,
        "wininet!CreateUrlCacheGroup flags={flags:#x} reserved={reserved:#x} group={group:#x}"
    );
    call.set_return_u64(emu, group);
    call.cleanup_args(emu, 2);
}

pub fn DeleteUrlCacheGroup(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let group = api_try!(emu, call.arg_u64(emu, 0));
    let flags_slot = if emu.cfg.arch.is_64bits() { 1 } else { 2 };
    let flags = api_try!(emu, call.arg_u32(emu, flags_slot));
    let reserved = api_try!(emu, call.arg(emu, flags_slot + 1));
    let value = if flags & !CACHEGROUP_FLAG_FLUSHURL_ONDELETE != 0 || reserved != 0 {
        Err(ERROR_INVALID_PARAMETER)
    } else if emu.handle_management.delete_cache_group(group) {
        Ok(1)
    } else {
        Err(ERROR_FILE_NOT_FOUND)
    };
    log_red!(
        emu,
        "wininet!DeleteUrlCacheGroup group={group:#x} flags={flags:#x} reserved={reserved:#x}"
    );
    result(emu, 4, value);
}

fn connected(emu: &mut Emu, api: &str, ext: bool, encoding: Encoding) {
    let call = WinApi(Stdcall);
    let destination = api_try!(emu, call.arg(emu, 0));
    let slots = if ext { 4 } else { 2 };
    let reserved = api_try!(emu, call.arg_u32(emu, slots - 1));
    let value = (|| {
        if reserved != 0 {
            return Err(ERROR_INVALID_PARAMETER);
        }
        if destination != 0
            && !emu
                .maps
                .write_dword(destination, INTERNET_CONNECTION_OFFLINE)
        {
            return Err(ERROR_INVALID_PARAMETER);
        }
        Ok(0)
    })();
    if ext && value.is_ok() {
        let name = api_try!(emu, call.arg(emu, 1));
        let capacity = api_try!(emu, call.arg_u32(emu, 2));
        if name != 0 && capacity != 0 {
            emu.maps.write_bytes(name, &encoding.encode(&[0]));
        }
    }
    log_red!(
        emu,
        "wininet!{api} flags_out={destination:#x} reserved={reserved} offline=true"
    );
    result(emu, slots, value);
}

pub fn InternetGetConnectedState(emu: &mut Emu) {
    connected(emu, "InternetGetConnectedState", false, Encoding::Ansi);
}

pub fn InternetGetConnectedStateEx(emu: &mut Emu) {
    connected(emu, "InternetGetConnectedStateEx", true, Encoding::Ansi);
}

pub fn InternetGetConnectedStateExA(emu: &mut Emu) {
    connected(emu, "InternetGetConnectedStateExA", true, Encoding::Ansi);
}

pub fn InternetGetConnectedStateExW(emu: &mut Emu) {
    connected(emu, "InternetGetConnectedStateExW", true, Encoding::Wide);
}

fn cookie(emu: &mut Emu, api: &str, encoding: Encoding, extended: bool) {
    let call = WinApi(Stdcall);
    let [url, cookie_name, data, size] = api_try!(emu, call.args(emu));
    if extended {
        let _reserved = api_try!(emu, call.arg(emu, 5));
    }
    log_red!(
        emu,
        "wininet!{api} url={url} cookie_name={cookie} data={data:#x} size={size:#x}",
        url = trace::input_string(emu, url, encoding),
        cookie = trace::input_string(emu, cookie_name, encoding)
    );
    let error = if url == 0 || size == 0 || !emu.maps.write_dword(size, 0) {
        ERROR_INVALID_PARAMETER
    } else {
        ERROR_NO_MORE_ITEMS
    };
    result(emu, if extended { 6 } else { 4 }, Err(error));
}

pub fn InternetGetCookieA(emu: &mut Emu) {
    cookie(emu, "InternetGetCookieA", Encoding::Ansi, false);
}

pub fn InternetGetCookieW(emu: &mut Emu) {
    cookie(emu, "InternetGetCookieW", Encoding::Wide, false);
}

pub fn InternetGetCookieExA(emu: &mut Emu) {
    cookie(emu, "InternetGetCookieExA", Encoding::Ansi, true);
}

pub fn InternetGetCookieExW(emu: &mut Emu) {
    cookie(emu, "InternetGetCookieExW", Encoding::Wide, true);
}
