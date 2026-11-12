use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptCATAdminEnumCatalogFromHash(emu: &mut emu::Emu) {
    let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pb_hash = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cb_hash = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptCATAdminEnumCatalogFromHash 0x{:x}",
        h_cat_admin
    );
    let hndl = helper::handler_create("CryptCATAdminEnumCatalogFromHash");
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 5);
}
