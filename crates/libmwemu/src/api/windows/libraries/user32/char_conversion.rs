use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::memory::read_guest_bytes;
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::{ERROR_INVALID_ADDRESS, ERROR_NOACCESS};

const MAX_CONVERSION_UNITS: usize = 1 << 20;

fn convert(emu: &mut Emu, api: &str, from: Encoding, to: Encoding, counted: bool) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let destination = api_try!(emu, call.arg(emu, 1));
    let length = if counted {
        Some(api_try!(emu, call.arg_u32(emu, 2)) as usize)
    } else {
        None
    };
    if length.is_some_and(|n| n > MAX_CONVERSION_UNITS) {
        policy::unsupported(emu, "user32", api, emu.pc());
        return;
    }
    log_red!(
        emu,
        "user32!{api} source={source:#x} destination={destination:#x}"
    );
    let result = (|| {
        if source == 0 || destination == 0 {
            return Err(ERROR_NOACCESS);
        }
        if source == destination && from.width() != to.width() {
            return Err(ERROR_INVALID_ADDRESS);
        }
        let units = if let Some(n) = length {
            let size = n * from.width();
            let bytes = read_guest_bytes(emu, source, size).ok_or(ERROR_NOACCESS)?;
            if matches!(from, Encoding::Wide) {
                bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .copied()
                    .map(u16::from_le_bytes)
                    .collect()
            } else {
                bytes.iter().map(|b| from.decode_byte(*b)).collect()
            }
        } else {
            let mut units = from.read(emu, source)?;
            units.push(0);
            units
        };
        let bytes = to.encode(&units);
        if !bytes.is_empty()
            && (!emu
                .maps
                .validate_write_range(destination, bytes.len() as u64)
                || !emu.maps.write_bytes(destination, &bytes))
        {
            return Err(ERROR_NOACCESS);
        }
        Ok(())
    })();
    if let Err(error) = result {
        emu.set_last_error(error);
    }
    call.set_return(emu, result.is_ok().into());
    call.cleanup_args(emu, 2 + usize::from(counted));
}

pub fn CharToOemA(emu: &mut Emu) {
    convert(emu, "CharToOemA", Encoding::Ansi, Encoding::Oem, false);
}

pub fn CharToOemW(emu: &mut Emu) {
    convert(emu, "CharToOemW", Encoding::Wide, Encoding::Oem, false);
}

pub fn CharToOemBuffA(emu: &mut Emu) {
    convert(emu, "CharToOemBuffA", Encoding::Ansi, Encoding::Oem, true);
}

pub fn CharToOemBuffW(emu: &mut Emu) {
    convert(emu, "CharToOemBuffW", Encoding::Wide, Encoding::Oem, true);
}

pub fn OemToCharA(emu: &mut Emu) {
    convert(emu, "OemToCharA", Encoding::Oem, Encoding::Ansi, false);
}

pub fn OemToCharW(emu: &mut Emu) {
    convert(emu, "OemToCharW", Encoding::Oem, Encoding::Wide, false);
}

pub fn OemToCharBuffA(emu: &mut Emu) {
    convert(emu, "OemToCharBuffA", Encoding::Oem, Encoding::Ansi, true);
}

pub fn OemToCharBuffW(emu: &mut Emu) {
    convert(emu, "OemToCharBuffW", Encoding::Oem, Encoding::Wide, true);
}
