use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CallNamedPipeA(emu: &mut emu::Emu) {
    let lpNamedPipeName = WinApiAbi::arg(emu, 0) as u32;
    let _lpInBuffer = WinApiAbi::arg(emu, 1) as u32;
    let _nInBufferSize = WinApiAbi::arg(emu, 2) as u32;
    let _lpOutBuffer = WinApiAbi::arg(emu, 3) as u32;
    let _nOutBufferSize = WinApiAbi::arg(emu, 4) as u32;
    let _lpBytesRead = WinApiAbi::arg(emu, 5) as u32;
    let _nTimeOut = WinApiAbi::arg(emu, 6) as u32;

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!CallNamedPipeA {}", lpNamedPipeName);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}

pub fn CallNamedPipeW(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let p4 = WinApiAbi::arg(emu, 4);
    let p5 = WinApiAbi::arg(emu, 5);
    let p6 = WinApiAbi::arg(emu, 6);
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
