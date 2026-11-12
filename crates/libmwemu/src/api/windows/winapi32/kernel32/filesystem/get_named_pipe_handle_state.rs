use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetNamedPipeHandleStateA(emu: &mut emu::Emu) {
    let hNamedPipe = WinApiAbi::arg(emu, 0) as u32;
    let _lpState = WinApiAbi::arg(emu, 1) as u32;
    let _lpCurInstances = WinApiAbi::arg(emu, 2) as u32;
    let _lpMaxCollectionCount = WinApiAbi::arg(emu, 3) as u32;
    let _lpCollectDataTimeout = WinApiAbi::arg(emu, 4) as u32;
    let lpUserName = WinApiAbi::arg(emu, 5) as u32;
    let nMaxUserNameSize = WinApiAbi::arg(emu, 6) as u32;

    let hNamedPipe = emu.maps.read_string(hNamedPipe as u64);
    let lpUserName = emu.maps.read_string(lpUserName as u64);
    let nMaxUserNameSize = emu.maps.read_string(nMaxUserNameSize as u64);

    log_red!(
        emu,
        "kernel32!GetNamedPipeHandleStateA {} {} {}",
        hNamedPipe,
        lpUserName,
        nMaxUserNameSize
    );

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}

pub fn GetNamedPipeHandleStateW(emu: &mut emu::Emu) {
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
        "kernel32!GetNamedPipeHandleStateW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
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
