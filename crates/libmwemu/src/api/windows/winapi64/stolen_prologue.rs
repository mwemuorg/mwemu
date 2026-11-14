/*
    Anti-hook trampolines copy an API's first instructions, run them, then jump
    to API+N so hooks and breakpoints on the entry point never fire.

    The winapi stubs can still serve such a call: decode the skipped bytes of
    the real API, undo what they did and dispatch the export as if it had been
    entered normally:

    - pushes and `sub rsp` are popped, so rsp points at the return address;
    - callee-saved registers the prologue overwrote after saving them (push or
      spill into the home area) are reloaded from their save slots;
    - flag-only instructions (test/cmp) need no undo.

    Anything else, in particular a write to an argument register, cannot be
    undone and is refused.

    Some exports are only a `jmp` into the real function body (ucrtbase's
    _time64 -> common_time); a trampoline that follows that jump skips the
    body's prologue instead, so the jump target counts as an entry of the
    export too.
*/

use iced_x86::{Decoder, DecoderOptions, Instruction, Mnemonic, OpKind, Register};

use crate::emu;

/// Longest stolen prologue we try to unwind.
const MAX_STOLEN_BYTES: u64 = 64;

/// Register arguments of APIs whose stolen prologue writes argument
/// registers. Registers past the count carry nothing the stub reads, so the
/// skipped instructions may overwrite them; unlisted APIs keep all four.
const API_ARGUMENT_COUNT: &[(&str, usize)] = &[
    ("GetCurrentProcess", 0),
    ("GetCurrentProcessId", 0),
    ("GetCurrentThread", 0),
    ("GetCurrentThreadId", 0),
    ("GetLastError", 0),
    ("GetProcessHeap", 0),
    ("GetTickCount", 0),
    ("GetTickCount64", 0),
    ("IsDebuggerPresent", 0),
    ("GetCursorPos", 1),
    ("GetPhysicalCursorPos", 1),
    ("_time64", 1),
    ("fopen_s", 3),
    ("WriteProfileStringA", 3),
    ("WriteProfileStringW", 3),
];

/// Home (shadow) area the caller reserves above the return address.
const HOME_AREA_START: i64 = 0x08;
const HOME_AREA_END: i64 = 0x28;

/// rcx, rdx, r8, r9 in Microsoft x64 argument order.
const ARGUMENT_REGISTERS: [Register; 4] =
    [Register::RCX, Register::RDX, Register::R8, Register::R9];

/// What the skipped instructions did, relative to the rsp at the API entry.
#[derive(Default)]
struct Prologue {
    /// bytes pushed or reserved below the entry rsp
    rsp_delta: u64,
    /// where each saved register went, as an offset from the entry rsp
    saved: Vec<(Register, i64)>,
    /// callee-saved registers overwritten after being saved
    clobbered: Vec<Register>,
    /// r11 as an offset from the entry rsp, after `mov r11, rsp`
    r11_offset: Option<i64>,
    /// register arguments the API reads (see API_ARGUMENT_COUNT)
    argument_count: usize,
}

/// If `addr` sits right after the first instructions of an exported API, undo
/// those instructions (rsp points at the caller's return address again) and
/// return the export's entry address.
pub fn unwind(emu: &mut emu::Emu, addr: u64) -> Option<u64> {
    let (code, entry) = export_entry_before(emu, addr)?;
    let argument_count = argument_count(emu, entry);
    let skipped = emu.maps.try_read_bytes(code, (addr - code) as usize)?;
    let prologue = decode_prologue(&skipped, code, argument_count)?;

    let entry_rsp = emu.regs().rsp + prologue.rsp_delta;
    for reg in &prologue.clobbered {
        let value = emu
            .maps
            .read_qword(entry_rsp.wrapping_add_signed(prologue.slot(*reg)?))?;
        emu.regs_mut().set_reg(*reg, value);
    }
    emu.regs_mut().rsp = entry_rsp;
    Some(entry)
}

/// Closest entry point at most MAX_STOLEN_BYTES below `addr`: where the
/// skipped bytes start, and the export to dispatch.
fn export_entry_before(emu: &emu::Emu, addr: u64) -> Option<(u64, u64)> {
    let near = |entry: u64| entry < addr && addr - entry <= MAX_STOLEN_BYTES;
    let direct = emu
        .export_indexes
        .iter_ordered()
        .flat_map(|module| module.by_address.keys().copied())
        .filter(|&entry| near(entry))
        .max()
        .map(|entry| (entry, entry));
    // exports of the module containing `addr` that are a `jmp rel32` stub
    let module = emu
        .export_indexes
        .iter_ordered()
        .filter(|module| module.base <= addr)
        .max_by_key(|module| module.base)?;
    let via_stub = module
        .by_address
        .keys()
        .filter_map(|&export| {
            let stub = emu.maps.try_read_bytes(export, 5)?;
            if stub[0] != 0xe9 {
                return None;
            }
            let rel = i32::from_le_bytes([stub[1], stub[2], stub[3], stub[4]]) as i64;
            let body = (export + 5).wrapping_add_signed(rel);
            near(body).then_some((body, export))
        })
        .max_by_key(|(body, _)| *body);
    [direct, via_stub]
        .into_iter()
        .flatten()
        .max_by_key(|(code, _)| *code)
}

fn argument_count(emu: &mut emu::Emu, entry: u64) -> usize {
    let name = super::kernel32::guess_api_name(emu, entry);
    let name = name.rsplit('!').next().unwrap_or(&name);
    API_ARGUMENT_COUNT
        .iter()
        .find(|(api, _)| *api == name)
        .map(|(_, count)| *count)
        .unwrap_or(ARGUMENT_REGISTERS.len())
}

/// `code` must decode into whole instructions that Prologue::step accepts.
fn decode_prologue(code: &[u8], ip: u64, argument_count: usize) -> Option<Prologue> {
    let mut decoder = Decoder::with_ip(64, code, ip, DecoderOptions::NONE);
    let mut prologue = Prologue {
        argument_count,
        ..Prologue::default()
    };
    let mut ins = Instruction::default();
    while decoder.can_decode() {
        decoder.decode_out(&mut ins);
        if ins.is_invalid() {
            return None;
        }
        prologue.step(&ins)?;
    }
    // the jump target must be an instruction boundary
    if decoder.position() != code.len() {
        return None;
    }
    // every overwritten callee-saved register needs a slot to reload it from
    if prologue
        .clobbered
        .iter()
        .any(|r| prologue.slot(*r).is_none())
    {
        return None;
    }
    Some(prologue)
}

impl Prologue {
    /// Account for one instruction; None when it cannot be undone.
    fn step(&mut self, ins: &Instruction) -> Option<()> {
        match ins.mnemonic() {
            Mnemonic::Push if ins.op0_kind() == OpKind::Register => {
                let reg = ins.op0_register();
                if !reg.is_gpr64() {
                    return None;
                }
                self.rsp_delta += 8;
                self.saved.push((reg, -(self.rsp_delta as i64)));
                Some(())
            }
            Mnemonic::Sub
                if ins.op0_kind() == OpKind::Register
                    && ins.op0_register() == Register::RSP
                    && is_immediate(ins.op1_kind()) =>
            {
                self.rsp_delta += ins.immediate(1);
                Some(())
            }
            Mnemonic::Mov | Mnemonic::And | Mnemonic::Or
                if ins.op0_kind() == OpKind::Memory
                    && (ins.op1_kind() == OpKind::Register || is_immediate(ins.op1_kind())) =>
            {
                self.store(ins)
            }
            Mnemonic::Mov
                if ins.op0_kind() == OpKind::Register
                    && ins.op1_kind() == OpKind::Register
                    && ins.op0_register() == Register::R11
                    && ins.op1_register() == Register::RSP =>
            {
                self.r11_offset = Some(-(self.rsp_delta as i64));
                Some(())
            }
            Mnemonic::Mov | Mnemonic::Xor
                if ins.op0_kind() == OpKind::Register
                    && (ins.op1_kind() == OpKind::Register || is_immediate(ins.op1_kind())) =>
            {
                self.write_register(ins.op0_register().full_register())
            }
            // a load only writes its destination register
            Mnemonic::Mov
                if ins.op0_kind() == OpKind::Register && ins.op1_kind() == OpKind::Memory =>
            {
                self.write_register(ins.op0_register().full_register())
            }
            Mnemonic::Test | Mnemonic::Cmp
                if ins.op0_kind() == OpKind::Register
                    && (ins.op1_kind() == OpKind::Register || is_immediate(ins.op1_kind())) =>
            {
                Some(())
            }
            _ => None,
        }
    }

    /// A store into the new frame or the home area. It changes no register
    /// and that memory is the callee's; `mov [..], r64` also records a save slot.
    fn store(&mut self, ins: &Instruction) -> Option<()> {
        if ins.memory_index() != Register::None {
            return None;
        }
        let disp = ins.memory_displacement64() as i64;
        let offset = match ins.memory_base() {
            Register::RSP => disp - self.rsp_delta as i64,
            Register::R11 => self.r11_offset? + disp,
            _ => return None,
        };
        // never the return address (+0) or the caller's stack arguments (+0x28..)
        if !(offset < 0 || (HOME_AREA_START..HOME_AREA_END).contains(&offset)) {
            return None;
        }
        if ins.mnemonic() == Mnemonic::Mov
            && ins.op1_kind() == OpKind::Register
            && ins.op1_register().is_gpr64()
        {
            self.saved.push((ins.op1_register(), offset));
        }
        Some(())
    }

    /// Writing a register: free for scratch registers, undoable for saved
    /// callee-saved ones, impossible for arguments.
    fn write_register(&mut self, reg: Register) -> Option<()> {
        if reg == Register::R11 {
            self.r11_offset = None;
        }
        if is_scratch(reg) || self.is_unused_argument(reg) {
            return Some(());
        }
        if is_callee_saved(reg) && self.slot(reg).is_some() {
            self.clobbered.push(reg);
            return Some(());
        }
        None
    }

    /// An argument register past the API's argument count.
    fn is_unused_argument(&self, reg: Register) -> bool {
        ARGUMENT_REGISTERS
            .iter()
            .position(|r| *r == reg)
            .is_some_and(|index| index >= self.argument_count)
    }

    /// Offset of the first save slot of `reg` (the caller's value).
    fn slot(&self, reg: Register) -> Option<i64> {
        self.saved.iter().find(|(r, _)| *r == reg).map(|(_, o)| *o)
    }
}

fn is_immediate(kind: OpKind) -> bool {
    matches!(
        kind,
        OpKind::Immediate8
            | OpKind::Immediate8_2nd
            | OpKind::Immediate16
            | OpKind::Immediate32
            | OpKind::Immediate64
            | OpKind::Immediate8to16
            | OpKind::Immediate8to32
            | OpKind::Immediate8to64
            | OpKind::Immediate32to64
    )
}

/// Volatile registers that carry no argument in the Microsoft x64 ABI.
fn is_scratch(reg: Register) -> bool {
    matches!(reg, Register::RAX | Register::R10 | Register::R11)
}

fn is_callee_saved(reg: Register) -> bool {
    matches!(
        reg,
        Register::RBX
            | Register::RBP
            | Register::RSI
            | Register::RDI
            | Register::R12
            | Register::R13
            | Register::R14
            | Register::R15
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// kernelbase!CloseHandle (Windows 11 26100): push rbx; sub rsp,20h;
    /// mov rbx,rcx; cmp ecx,-0Ch. Obsidium's trampoline runs these and jumps
    /// to CloseHandle+0xc. The cmp's sign-extended imm8 is an Immediate8to32.
    #[test]
    fn close_handle_prologue_with_cmp_imm8() {
        let code = [
            0x40, 0x53, 0x48, 0x83, 0xec, 0x20, 0x48, 0x8b, 0xd9, 0x83, 0xf9, 0xf4,
        ];
        let prologue = decode_prologue(&code, 0x1800459c0, 4).expect("prologue should unwind");
        assert_eq!(prologue.rsp_delta, 0x28);
        assert_eq!(prologue.clobbered, vec![Register::RBX]);
        assert_eq!(prologue.slot(Register::RBX), Some(-8));
    }

    #[test]
    fn immediate_kinds() {
        for kind in [
            OpKind::Immediate8,
            OpKind::Immediate16,
            OpKind::Immediate32,
            OpKind::Immediate64,
            OpKind::Immediate8to16,
            OpKind::Immediate8to32,
            OpKind::Immediate8to64,
            OpKind::Immediate32to64,
        ] {
            assert!(is_immediate(kind), "{kind:?}");
        }
        assert!(!is_immediate(OpKind::Register));
        assert!(!is_immediate(OpKind::Memory));
    }
}
