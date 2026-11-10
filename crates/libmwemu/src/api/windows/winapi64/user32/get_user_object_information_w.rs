use super::UOI_FLAGS;
use super::UOI_NAME;
use super::UOI_TYPE;
use super::WSF_VISIBLE;
use super::wide_z;
use crate::emu;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub(super) fn GetUserObjectInformationW(emu: &mut emu::Emu) {
    let hobj = emu.regs().rcx;
    let index = emu.regs().rdx & 0xffff_ffff;
    let info = emu.regs().r8;
    let length = emu.regs().r9 & 0xffff_ffff;
    let needed_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(
        emu,
        "user32!GetUserObjectInformationW hObj: 0x{:x} nIndex: {} pvInfo: 0x{:x} nLength: {}",
        hobj,
        index,
        info,
        length
    );

    let data = match index {
        // USEROBJECTFLAGS { fInherit, fReserved, dwFlags }: an interactive, visible station
        UOI_FLAGS => [0u32, 0, WSF_VISIBLE]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<u8>>(),
        UOI_NAME => {
            let name = helper::handler_get_uri(hobj);
            let name = if name.is_empty() {
                "WinSta0".to_string()
            } else {
                name
            };
            wide_z(&name)
        }
        UOI_TYPE => wide_z("WindowStation"),
        _ => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    if needed_ptr != 0 {
        emu.maps.write_dword(needed_ptr, data.len() as u32);
    }
    if info == 0 || length < data.len() as u64 {
        emu.set_last_error(ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = 0;
        return;
    }

    emu.maps.write_bytes(info, &data);
    emu.regs_mut().rax = 1;
}
