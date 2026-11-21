use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::policy::api_try;
use crate::windows::constants::security::{
    GENERIC_ALL, GENERIC_EXECUTE, GENERIC_READ, GENERIC_WRITE,
};

const GENERIC_RIGHTS: [u32; 4] = [GENERIC_READ, GENERIC_WRITE, GENERIC_EXECUTE, GENERIC_ALL];

pub fn AreAllAccessesGranted(emu: &mut Emu) {
    granted(emu, true);
}

pub fn AreAnyAccessesGranted(emu: &mut Emu) {
    granted(emu, false);
}

fn granted(emu: &mut Emu, all: bool) {
    let call = WinApi(Stdcall);
    let granted = api_try!(emu, call.arg_u32(emu, 0));
    let desired = api_try!(emu, call.arg_u32(emu, 1));
    let overlap = granted & desired;
    let name = if all {
        "AreAllAccessesGranted"
    } else {
        "AreAnyAccessesGranted"
    };
    log_red!(emu, "{name} granted={granted:#x} desired={desired:#x}");
    let result = if all {
        overlap == desired
    } else {
        overlap != 0
    };
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 2);
}

pub fn MapGenericMask(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let destination = api_try!(emu, call.arg(emu, 0));
    let mapping = api_try!(emu, call.arg(emu, 1));
    let size = size_of::<u32>();
    log_red!(
        emu,
        "MapGenericMask mask={destination:#x} mapping={mapping:#x}"
    );
    let Some(mut mask) = emu.maps.read_dword(destination) else {
        emu.memory_exception(
            ExceptionType::BadAddressDereferencing,
            destination,
            size,
            MemoryAccess::Read,
        );
        return;
    };
    for (i, flag) in GENERIC_RIGHTS.into_iter().enumerate() {
        if mask & flag != 0 {
            let address = mapping.saturating_add((i * size) as u64);
            let Some(specific) = emu.maps.read_dword(address) else {
                emu.memory_exception(
                    ExceptionType::BadAddressDereferencing,
                    address,
                    size,
                    MemoryAccess::Read,
                );
                return;
            };
            mask |= specific;
        }
    }
    mask &= !(GENERIC_READ | GENERIC_WRITE | GENERIC_EXECUTE | GENERIC_ALL);
    if !emu.maps.write_dword(destination, mask) {
        emu.memory_exception(
            ExceptionType::BadAddressDereferencing,
            destination,
            size,
            MemoryAccess::Write,
        );
        return;
    }
    call.cleanup_args(emu, 2);
}
