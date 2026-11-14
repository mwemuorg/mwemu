use crate::color;
use crate::emu::Emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use iced_x86::Instruction;

pub fn swapgs(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let gs = emu.regs().gs;
    let kernel_gs = emu.regs().kernel_gs_base;
    emu.regs_mut().gs = kernel_gs;
    emu.regs_mut().kernel_gs_base = gs;
    log::trace!("swapgs: gs=0x{:x} <-> kernel_gs_base=0x{:x}", kernel_gs, gs);
    true
}

pub fn sgdt(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(a) => a,
        None => return false,
    };
    let base = emu.regs().gdt_base;
    let limit = emu.regs().gdt_limit;
    emu.maps.write_word(addr, limit);
    emu.maps.write_qword(addr + 2, base);
    log::trace!(
        "sgdt: stored base=0x{:x} limit=0x{:x} at 0x{:x}",
        base,
        limit,
        addr
    );
    true
}

pub fn sidt(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(a) => a,
        None => return false,
    };
    let base = emu.regs().idt_base;
    let limit = emu.regs().idt_limit;
    emu.maps.write_word(addr, limit);
    emu.maps.write_qword(addr + 2, base);
    log::trace!(
        "sidt: stored base=0x{:x} limit=0x{:x} at 0x{:x}",
        base,
        limit,
        addr
    );
    true
}

pub fn lgdt(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(a) => a,
        None => return false,
    };
    let limit = emu.maps.read_word(addr).unwrap_or(0);
    let base = emu.maps.read_qword(addr + 2).unwrap_or(0);
    emu.regs_mut().gdt_limit = limit;
    emu.regs_mut().gdt_base = base;
    log::info!("lgdt: base=0x{:x} limit=0x{:x}", base, limit);
    true
}

pub fn lidt(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let addr = match emu.get_operand_value(ins, 0, false) {
        Some(a) => a,
        None => return false,
    };
    let limit = emu.maps.read_word(addr).unwrap_or(0);
    let base = emu.maps.read_qword(addr + 2).unwrap_or(0);
    emu.regs_mut().idt_limit = limit;
    emu.regs_mut().idt_base = base;
    log::info!(
        "lidt: base=0x{:x} limit=0x{:x} — driver hooking IDT",
        base,
        limit
    );
    true
}

pub fn sldt(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let val = emu.regs().ldt_selector as u64;
    emu.set_operand_value(ins, 0, val)
}

pub fn ltr(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let val = match emu.get_operand_value(ins, 0, true) {
        Some(v) => v,
        None => return false,
    };
    emu.regs_mut().tr_selector = val as u16;
    log::info!("ltr: selector=0x{:x}", val);
    true
}

pub fn str_inst(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let val = emu.regs().tr_selector as u64;
    emu.set_operand_value(ins, 0, val)
}

pub fn invlpg(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("LightGray"), &DecodedInstruction::X86(*ins));
    }
    true
}

pub fn wbinvd(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("LightGray"), &DecodedInstruction::X86(*ins));
    }
    true
}

pub fn invd(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("LightGray"), &DecodedInstruction::X86(*ins));
    }
    true
}

pub fn clts(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("LightGray"), &DecodedInstruction::X86(*ins));
    }
    emu.regs_mut().cr0 &= !0x8;
    true
}

pub fn xsetbv(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    let _ecx = emu.regs().rcx as u32;
    let _val = (emu.regs().rdx << 32) | (emu.regs().rax & 0xffffffff);
    log::trace!("xsetbv: xcr{} = 0x{:x}", _ecx, _val);
    true
}

pub fn rdpid(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("Red"), &DecodedInstruction::X86(*ins));
    }
    emu.set_operand_value(ins, 0, 0)
}

pub fn monitor(emu: &mut Emu, ins: &Instruction, _instruction_sz: usize, _rep_step: bool) -> bool {
    if emu.cfg.verbose >= 2 {
        emu.show_instruction(color!("LightGray"), &DecodedInstruction::X86(*ins));
    }
    true
}
