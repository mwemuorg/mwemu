use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::files::*;

pub fn QueryDosDeviceA(emu: &mut Emu) {
    invoke(emu, false, "QueryDosDeviceA");
}

pub fn QueryDosDeviceW(emu: &mut Emu) {
    invoke(emu, true, "QueryDosDeviceW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let input = api_try!(emu, call.arg(emu, 0));
    let output = api_try!(emu, call.arg(emu, 1));
    let capacity = api_try!(emu, call.arg_u32(emu, 2));
    let device = strings::read_text(emu, input, wide);
    log_red!(
        emu,
        "kernel32!{} device:'{}' target:0x{:x} capacity:{}",
        api,
        device,
        output,
        capacity
    );
    let text = if input == 0 {
        "C:"
    } else if device.eq_ignore_ascii_case("C:") {
        SYSTEM_VOLUME_DEVICE
    } else {
        emu.set_last_error(ERROR_FILE_NOT_FOUND);
        call.set_return(emu, 0);
        return;
    };
    let bytes = strings::encode_text(&format!("{text}\0\0"), wide);
    let count = bytes.len()
        / if wide {
            size_of::<u16>()
        } else {
            size_of::<u8>()
        };
    let return_value = if (capacity as usize) < count || output == 0 {
        emu.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        0
    } else if !emu.maps.validate_write_range(output, bytes.len() as u64)
        || !emu.maps.write_bytes(output, &bytes)
    {
        emu.set_last_error(ERROR_NOACCESS);
        0
    } else {
        count as u64
    };
    call.set_return(emu, return_value);
}
