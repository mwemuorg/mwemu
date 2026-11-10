use crate::emu;
use crate::windows::constants::errors::*;

pub fn CallNamedPipeA(emu: &mut emu::Emu) {
    let lpNamedPipeName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpInBuffer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _nInBufferSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpOutBuffer = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _nOutBufferSize = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _lpBytesRead = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _nTimeOut = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!CallNamedPipeA {}", lpNamedPipeName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}

pub fn CallNamedPipeW(emu: &mut emu::Emu) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!CallNamedPipeW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
