use crate::emu;
use crate::winapi::winapi64::kernel32::InitializeCriticalSection;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "RtlInitializeCriticalSection" => InitializeCriticalSection(emu),
        "RtlInitializeCriticalSectionAndSpinCount" => RtlInitializeCriticalSectionAndSpinCount(emu),
        "RtlEnterCriticalSection" => RtlEnterCriticalSection(emu),
        "RtlLeaveCriticalSection" => RtlLeaveCriticalSection(emu),
        "RtlInitializeCriticalSectionEx" => RtlInitializeCriticalSectionEx(emu),
        _ => return false,
    }
    true
}

fn RtlInitializeCriticalSectionAndSpinCount(emu: &mut emu::Emu) {
    let crit_sect = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlInitializeCriticalSectionAndSpinCount error reading crit_sect param");
    let spin_count = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlInitializeCriticalSectionAndSpinCount error reading spin_count param");

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionAndSpinCount");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

fn RtlEnterCriticalSection(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlEnterCriticalSection error reading hndl param") as u64;

    log_red!(emu, "ntdll!RtlEnterCriticalSection");

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn RtlLeaveCriticalSection(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlLeaveCriticalSection error reading hndl param") as u64;

    log_red!(emu, "ntdll!RtlLeaveCriticalSection");

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}

fn RtlInitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let crit_sect_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!RtlInitializeCriticalSectionEx error reading crit_sect_ptr")
        as u64;
    let spin_count = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!RtlInitializeCriticalSectionEx error reading spin_count");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!RtlInitializeCriticalSectionEx error reading flags");

    log_red!(emu, "ntdll!RtlInitializeCriticalSectionEx");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
