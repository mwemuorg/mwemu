use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::ERROR_SUCCESS;

pub fn RegOpenKeyExA(emu: &mut emu::Emu) {
    let subkey = emu.maps.read_string(arg(emu, 1) as u64);
    let result_ptr = arg(emu, 4) as u64;
    log_red!(emu, "advapi32!RegOpenKeyExA {}", subkey);
    if result_ptr != 0 {
        let hndl = helper::handler_create(&format!("registry://{}", subkey));
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    emu.regs_mut().rax = ERROR_SUCCESS;
    stdcall_ret(emu, 5);
}
