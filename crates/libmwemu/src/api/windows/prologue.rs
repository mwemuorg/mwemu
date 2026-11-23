use iced_x86::{Code, Decoder, DecoderOptions, Instruction, Mnemonic, OpKind, Register};

use crate::emu::Emu;

pub(super) const MAX_PREFIX_BYTES: u64 = 64;

pub(super) fn direct_entry_before(emu: &Emu, addr: u64) -> Option<u64> {
    let section = emu.maps.get_mem_by_addr(addr)?;
    let (library, _) = section.get_name().rsplit_once('.')?;
    let module = emu.export_indexes.get_by_name(library)?;
    if module.by_address.contains_key(&addr) {
        return None;
    }
    let start = section
        .get_base()
        .max(addr.saturating_sub(MAX_PREFIX_BYTES));
    (start..addr)
        .rev()
        .find(|entry| module.by_address.contains_key(entry))
}

pub(super) fn instructions(
    bitness: u32,
    bytes: &[u8],
    ip: u64,
) -> impl Iterator<Item = Instruction> + '_ {
    Decoder::with_ip(bitness, bytes, ip, DecoderOptions::NONE).into_iter()
}

fn mov_registers(ins: &Instruction, dst: Register, src: Register) -> bool {
    ins.mnemonic() == Mnemonic::Mov
        && ins.op0_kind() == OpKind::Register
        && ins.op1_kind() == OpKind::Register
        && ins.op0_register() == dst
        && ins.op1_register() == src
}

/// x86 ntdll service entries start with MOV EAX, imm32. A copied service
/// selector changes neither the argument frame nor any nonvolatile register.
fn resumed_nt_service(emu: &Emu, entry: u64, addr: u64) -> bool {
    const MOV_EAX_IMMEDIATE_SIZE: u64 = 1 + size_of::<u32>() as u64;
    if addr - entry != MOV_EAX_IMMEDIATE_SIZE {
        return false;
    }
    let Some(name) = emu.syscall_name_by_real.get(&emu.regs().get_eax()) else {
        return false;
    };
    emu.export_indexes
        .get_by_name("ntdll")
        .and_then(|index| index.resolve_name(&name.to_ascii_lowercase()))
        .and_then(|export| export.direct_address())
        == Some(entry)
}

/// Recover a complete EBP frame only when its saved slots agree with the mapped prologue.
pub(crate) fn unwind_x86(emu: &mut Emu, addr: u64) -> Option<u64> {
    let entry = direct_entry_before(emu, addr)?;
    if resumed_nt_service(emu, entry, addr) {
        return Some(entry);
    }
    let bytes = emu.maps.try_read_bytes(entry, (addr - entry) as usize)?;
    let mut prefix = instructions(32, &bytes, entry);
    let mut first = prefix.next()?;
    if mov_registers(&first, Register::EDI, Register::EDI) {
        first = prefix.next()?;
    }
    if first.code() != Code::Push_r32
        || first.op0_register() != Register::EBP
        || !mov_registers(&prefix.next()?, Register::EBP, Register::ESP)
    {
        return None;
    }

    let frame = emu.regs().rbp as u32;
    let mut sp = frame;
    for ins in prefix {
        let expected = match ins.code() {
            Code::Pushd_imm8 => ins.immediate8to32() as u32,
            Code::Pushd_imm32 => ins.immediate32(),
            _ => return None,
        };
        sp = sp.checked_sub(size_of::<u32>() as u32)?;
        if emu.maps.read_dword(sp as u64)? != expected {
            return None;
        }
    }
    if sp as u64 != emu.regs().get_esp() {
        return None;
    }
    let saved_ebp = emu.maps.read_dword(frame as u64)?;
    let entry_sp = frame.checked_add(size_of::<u32>() as u32)?;
    emu.maps.read_dword(entry_sp as u64)?;

    emu.regs_mut().rbp = saved_ebp as u64;
    emu.regs_mut().set_esp(entry_sp as u64);
    Some(entry)
}
