use std::net::{Ipv4Addr, Ipv6Addr};

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::winapi::common::memory::read_array;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

fn format_address(emu: &Emu, family: u32, address: u64) -> Result<String, u64> {
    match family {
        AF_INET => read_array(emu, address)
            .map(Ipv4Addr::from)
            .map(|ip| ip.to_string())
            .ok_or(WSAEFAULT),
        AF_INET6 => read_array(emu, address)
            .map(Ipv6Addr::from)
            .map(|ip| ip.to_string())
            .ok_or(WSAEFAULT),
        _ => Err(WSAEAFNOSUPPORT),
    }
}

fn parse_address(family: u32, text: &str) -> Option<Vec<u8>> {
    match family {
        AF_INET => text.parse::<Ipv4Addr>().ok().map(|ip| ip.octets().to_vec()),
        AF_INET6 => text.parse::<Ipv6Addr>().ok().map(|ip| ip.octets().to_vec()),
        _ => None,
    }
}

fn to_text(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let family = api_try!(emu, call.arg_u32(emu, 0));
    let address = api_try!(emu, call.arg(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let capacity = api_try!(emu, call.arg(emu, 3));
    let result = (|| {
        let text = format_address(emu, family, address)?;
        if output == 0 {
            return Err(WSAEFAULT);
        }
        // Address text is ASCII; the wide entry's capacity is in UTF-16 code units.
        if capacity <= text.len() as u64 {
            return Err(WSAEINVAL);
        }
        let bytes = strings::encode_text(&format!("{text}\0"), wide);
        if !emu.maps.write_bytes(output, &bytes) {
            return Err(WSAEFAULT);
        }
        Ok(output)
    })();
    log_red!(
        emu,
        "ws2_32!{api} family={family} address={address:#x} output={output:#x} capacity={capacity} => {result:?}"
    );
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        0
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, 4);
}

fn from_text(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let family = api_try!(emu, call.arg_u32(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let result = (|| {
        if !matches!(family, AF_INET | AF_INET6) {
            return Err(WSAEAFNOSUPPORT);
        }
        if output == 0 {
            return Err(WSAEFAULT);
        }
        let text = strings::read_text_checked(emu, source, wide).ok_or(WSAEFAULT)?;
        let Some(bytes) = parse_address(family, &text) else {
            // The wide export sets an error on invalid syntax; inet_pton preserves it.
            if wide {
                emu.set_last_error(WSAEINVAL);
            }
            return Ok(0);
        };
        if !emu.maps.write_bytes(output, &bytes) {
            return Err(WSAEFAULT);
        }
        Ok(1)
    })();
    log_red!(
        emu,
        "ws2_32!{api} family={family} source={source:#x} output={output:#x} => {result:?}"
    );
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        SOCKET_ERROR
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, 3);
}

pub fn inet_ntop(emu: &mut Emu) {
    to_text(emu, "inet_ntop", false);
}

pub fn InetNtopW(emu: &mut Emu) {
    to_text(emu, "InetNtopW", true);
}

pub fn inet_pton(emu: &mut Emu) {
    from_text(emu, "inet_pton", false);
}

pub fn InetPtonW(emu: &mut Emu) {
    from_text(emu, "InetPtonW", true);
}

// inet_addr accepts the historical one- through four-part IPv4 spelling.
fn legacy_ipv4(text: &str) -> Option<Ipv4Addr> {
    if text == " " {
        return Some(Ipv4Addr::UNSPECIFIED);
    }
    let text = text.split(char::is_whitespace).next()?;
    let mut parts = text.split('.').peekable();
    let mut value = 0u32;
    let mut remaining = u32::BITS;
    while let Some(part) = parts.next() {
        let (digits, radix) =
            if let Some(hex) = part.strip_prefix("0x").or_else(|| part.strip_prefix("0X")) {
                (hex, 16)
            } else if part.starts_with('0') {
                (part, 8)
            } else {
                (part, 10)
            };
        if digits.is_empty() || !digits.bytes().all(|byte| (byte as char).is_digit(radix)) {
            return None;
        }
        let number = u32::from_str_radix(digits, radix).ok()?;
        if parts.peek().is_none() {
            if number > u32::MAX >> (u32::BITS - remaining) {
                return None;
            }
            return Some(Ipv4Addr::from(value | number));
        }
        if remaining <= u8::BITS || number > u8::MAX.into() {
            return None;
        }
        remaining -= u8::BITS;
        value |= number << remaining;
    }
    None
}

pub fn inet_addr(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let text = strings::read_text_checked(emu, source, false);
    let value = text
        .as_deref()
        .and_then(legacy_ipv4)
        .map_or(u32::MAX, |address| u32::from_le_bytes(address.octets()));
    log_red!(emu, "ws2_32!inet_addr address={text:?} result={value:#x}");
    call.set_return(emu, value.into());
    call.cleanup_args(emu, 1);
}

pub fn inet_ntoa(emu: &mut Emu) {
    const INET_ADDRSTRLEN: u64 = 16;
    let call = WinApi(Stdcall);
    let address = Ipv4Addr::from(api_try!(emu, call.arg_u32(emu, 0)).to_le_bytes());
    let key = format!("winsock_inet_ntoa_{}", emu.current_thread().id);
    let result = (|| {
        let buffer = if let Some(map) = emu.maps.get_map_by_name(&key) {
            map.get_base()
        } else {
            let buffer = emu.maps.alloc(INET_ADDRSTRLEN)?;
            emu.maps
                .create_map(&key, buffer, INET_ADDRSTRLEN, Permission::READ_WRITE)
                .ok()?;
            buffer
        };
        emu.maps
            .write_bytes(buffer, format!("{address}\0").as_bytes())
            .then_some(buffer)
    })();
    log_red!(emu, "ws2_32!inet_ntoa address={address} result={result:?}");
    call.set_return(emu, result.unwrap_or(0));
    call.cleanup_args(emu, 1);
}
