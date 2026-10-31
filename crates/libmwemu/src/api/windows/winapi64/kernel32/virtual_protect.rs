use crate::emu;
use crate::winapi::common::virtual_protect;
use crate::windows::constants;

/*
BOOL VirtualProtect(
  [in]  LPVOID lpAddress,
  [in]  SIZE_T dwSize,
  [in]  DWORD  flNewProtect,
  [out] PDWORD lpflOldProtect
);
*/
pub fn VirtualProtect(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let new_prot = emu.regs().r8 as u32;
    let old_prot_ptr = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!VirtualProtect addr: 0x{:x} sz: {} prot: {}",
        addr,
        size,
        new_prot
    );

    if !protect(emu, addr, size, new_prot, old_prot_ptr) {
        return;
    }

    let is_exec = (new_prot & 0xF0) != 0;
    if is_exec {
        let base = emu.maps.get_addr_base(addr).unwrap_or(addr);
        emu.init_tls_from_pe64_in_memory(base);
    }
}

/// Shared body of VirtualProtect[Ex]: TRUE in rax and the previous protection
/// in `*old_prot_ptr`, or FALSE with the last error set.
pub fn protect(emu: &mut emu::Emu, addr: u64, size: u64, new_prot: u32, old_prot_ptr: u64) -> bool {
    if old_prot_ptr == 0 {
        emu.set_last_error(constants::ERROR_INVALID_PARAMETER);
        emu.regs_mut().rax = 0;
        return false;
    }
    match virtual_protect::protect(&mut emu.maps, addr, size, new_prot) {
        Some(old) => {
            emu.maps.write_dword(old_prot_ptr, old);
            emu.regs_mut().rax = 1;
            true
        }
        None => {
            emu.set_last_error(constants::ERROR_INVALID_ADDRESS);
            emu.regs_mut().rax = 0;
            false
        }
    }
}
