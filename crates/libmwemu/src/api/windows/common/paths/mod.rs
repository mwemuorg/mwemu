mod cch;
mod layout;
mod legacy;

use crate::emu::Emu;
use crate::winapi::common::memory::read_array;
use crate::winapi::environment_store::Encoding;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::{MAX_PATH, PATHCCH_MAX_CCH};

fn read(emu: &Emu, address: u64, codec: Encoding, capacity: usize) -> Result<Vec<u16>, u64> {
    if address == 0 {
        return Err(HRESULT_E_INVALID_ARG);
    }
    let mut units = Vec::new();
    for index in 0..capacity.min(PATHCCH_MAX_CCH) {
        let p = address
            .checked_add((index * codec.width()) as u64)
            .ok_or(E_POINTER)?;
        let u = if codec.width() == 2 {
            u16::from_le_bytes(read_array(emu, p).ok_or(E_POINTER)?)
        } else {
            codec.decode_byte(emu.maps.read_byte(p).ok_or(E_POINTER)?)
        };
        if u == 0 {
            return Ok(units);
        }
        units.push(u);
    }
    Err(HRESULT_INSUFFICIENT_BUFFER)
}

fn write(
    emu: &mut Emu,
    address: u64,
    codec: Encoding,
    capacity: usize,
    text: &[u16],
) -> Result<(), u64> {
    let bytes = codec.terminated(text);
    if bytes.len() / codec.width() > capacity {
        return Err(HRESULT_INSUFFICIENT_BUFFER);
    }
    if address == 0
        || !emu.maps.validate_write_range(address, bytes.len() as u64)
        || !emu.maps.write_bytes(address, &bytes)
    {
        return Err(E_POINTER);
    }
    Ok(())
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    if library == "kernelbase" && cch::dispatch(api, emu) {
        return true;
    }
    legacy::dispatch(api, library, emu)
}
