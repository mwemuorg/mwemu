use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn Thread32First(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_snapshot = api_try!(emu, call.arg(emu, 0));
    let p_entry = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "kernel32!Thread32First snap:0x{:x} entry:0x{:x}",
        h_snapshot,
        p_entry
    );

    if p_entry == 0 {
        emu.set_last_error(87);
        call.set_return(emu, 0);
        return;
    }

    // Fill THREADENTRY32 structure (28 bytes)
    // dwSize
    emu.maps.write_dword(p_entry, 28);
    // cntUsage (deprecated)
    emu.maps.write_dword(p_entry + 4, 0);
    // th32ThreadID
    let tid = emu.current_thread().id as u32;
    emu.maps.write_dword(p_entry + 8, tid);
    // th32OwnerProcessID
    emu.maps.write_dword(p_entry + 12, 0x123);
    // tpBasePri
    emu.maps.write_dword(p_entry + 16, 8); // THREAD_PRIORITY_NORMAL
    // tpDeltaPri (deprecated)
    emu.maps.write_dword(p_entry + 20, 0);
    // dwFlags (deprecated)
    emu.maps.write_dword(p_entry + 24, 0);

    emu.set_last_error(0);
    call.set_return(emu, 1); // TRUE
}
