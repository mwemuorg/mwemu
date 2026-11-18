use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dllptr = api_try!(emu, call.arg(emu, 0));
    let dll = emu.maps.read_string(dllptr);

    let return_value = load_library(emu, &dll);
    call.set_return(emu, return_value);

    log_red!(
        emu,
        "** {} kernel32!LoadLibraryA  '{}' =0x{:x} rip: 0x{:x}",
        emu.pos,
        &dll,
        emu.regs().get_eax() as u32,
        emu.regs().rip
    );

    call.cleanup_args(emu, 1);

    //TODO: instead returning the base, return a handle that have linked the dll name
}

pub fn LoadLibraryW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dllptr = api_try!(emu, call.arg(emu, 0));
    let dll = emu.maps.read_wide_string(dllptr);
    log_red!(emu, "LoadLibraryW  '{}'", dll);

    //if dll == "ntdll.dll" {
    //  emu.regs_mut().rax = emu.maps.get_mem("ntdll").get_base();
    //}

    let return_value = load_library(emu, &dll);
    call.set_return(emu, return_value);
    call.cleanup_args(emu, 1);
}
