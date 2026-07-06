use crate::emu::Emu;
use yaxpeax_arm::armv8::a64::{Instruction, Opcode};

pub mod helpers;
pub mod instructions;

/// Emulate a single decoded AArch64 instruction. Returns true on success.
pub fn emulate_instruction(emu: &mut Emu, ins: &Instruction) -> bool {
    match ins.opcode {
        // --- Data Processing (register/immediate) ---
        Opcode::ADD => instructions::add::execute(emu, ins, false),
        Opcode::ADDS => instructions::add::execute(emu, ins, true),
        Opcode::SUB => instructions::sub::execute(emu, ins, false),
        Opcode::SUBS => instructions::sub::execute(emu, ins, true),
        Opcode::AND => instructions::and::execute(emu, ins, false),
        Opcode::ANDS => instructions::and::execute(emu, ins, true),
        Opcode::ORR => instructions::orr::execute(emu, ins),
        Opcode::EOR => instructions::eor::execute(emu, ins),
        Opcode::ORN => instructions::orn::execute(emu, ins),
        Opcode::BIC => instructions::bic::execute(emu, ins),
        Opcode::MOVZ => instructions::movz::execute(emu, ins),
        Opcode::MOVK => instructions::movk::execute(emu, ins),
        Opcode::MOVN => instructions::movn::execute(emu, ins),
        Opcode::MADD => instructions::madd::execute(emu, ins),
        Opcode::MSUB => instructions::msub::execute(emu, ins),
        Opcode::SDIV => instructions::sdiv::execute(emu, ins),
        Opcode::UDIV => instructions::udiv::execute(emu, ins),
        Opcode::ADR => instructions::adr::execute(emu, ins),
        Opcode::ADRP => instructions::adrp::execute(emu, ins),
        Opcode::CLZ => instructions::clz::execute(emu, ins),
        Opcode::LSLV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Lsl),
        Opcode::LSRV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Lsr),
        Opcode::ASRV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Asr),
        Opcode::RORV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Ror),
        Opcode::EXTR => instructions::extr::execute(emu, ins),
        Opcode::RBIT => instructions::rbit::execute(emu, ins),
        Opcode::REV => instructions::rev::execute(emu, ins),
        Opcode::REV16 => instructions::rev16::execute(emu, ins),
        Opcode::REV32 => instructions::rev32::execute(emu, ins),

        // --- Loads ---
        Opcode::LDR => instructions::ldr::execute(emu, ins),
        Opcode::LDRB => instructions::ldrb::execute(emu, ins),
        Opcode::LDRH => instructions::ldrh::execute(emu, ins),
        Opcode::LDRSB => instructions::ldrsb::execute(emu, ins),
        Opcode::LDRSH => instructions::ldrsh::execute(emu, ins),
        Opcode::LDRSW => instructions::ldrsw::execute(emu, ins),
        Opcode::LDP => instructions::ldp::execute(emu, ins),
        Opcode::LDXR => instructions::ldr::execute(emu, ins), // treat as LDR for now
        Opcode::LDAR => instructions::ldr::execute(emu, ins), // treat as LDR for now
        Opcode::LDUR => instructions::ldur::execute(emu, ins),
        Opcode::LDURB => instructions::ldurb::execute(emu, ins),
        Opcode::LDURH => instructions::ldurh::execute(emu, ins),
        Opcode::LDURSB => instructions::ldursb::execute(emu, ins),
        Opcode::LDURSH => instructions::ldursh::execute(emu, ins),
        Opcode::LDURSW => instructions::ldursw::execute(emu, ins),

        // --- Stores ---
        Opcode::STR => instructions::str::execute(emu, ins),
        Opcode::STRB => instructions::strb::execute(emu, ins),
        Opcode::STRH => instructions::strh::execute(emu, ins),
        Opcode::STP => instructions::stp::execute(emu, ins),
        Opcode::STXR => instructions::stxr::execute(emu, ins),
        Opcode::STLR => instructions::str::execute(emu, ins), // treat as STR for now
        Opcode::STUR => instructions::stur::execute(emu, ins),
        Opcode::STURB => instructions::sturb::execute(emu, ins),
        Opcode::STURH => instructions::sturh::execute(emu, ins),

        // --- Branches ---
        Opcode::B => instructions::b::execute(emu, ins),
        Opcode::BL => instructions::bl::execute(emu, ins),
        Opcode::BR => instructions::br::execute(emu, ins),
        Opcode::BLR => instructions::blr::execute(emu, ins),
        Opcode::RET => instructions::ret::execute(emu, ins),
        Opcode::CBZ => instructions::cbz::execute(emu, ins, true),
        Opcode::CBNZ => instructions::cbz::execute(emu, ins, false),
        Opcode::TBZ => instructions::tbz::execute(emu, ins, true),
        Opcode::TBNZ => instructions::tbz::execute(emu, ins, false),
        Opcode::Bcc(cond) => instructions::bcc::execute(emu, ins, cond),

        // --- Conditional select ---
        Opcode::CSEL => instructions::csel::execute(emu, ins),
        Opcode::CSINC => instructions::csinc::execute(emu, ins),
        Opcode::CSINV => instructions::csinv::execute(emu, ins),
        Opcode::CSNEG => instructions::csneg::execute(emu, ins),

        // --- System ---
        Opcode::SVC => instructions::svc::execute(emu, ins),
        Opcode::MRS => instructions::mrs::execute(emu, ins),
        Opcode::MSR => instructions::msr::execute(emu, ins),
        Opcode::HINT => true, // NOP is encoded as HINT
        Opcode::DMB(_) | Opcode::DSB(_) | Opcode::ISB => true, // barriers are no-ops in emulation
        Opcode::CLREX => true,
        // ARMv8.3 Pointer Authentication: signs / authenticates a pointer in
        // place against a CPU key. We don't model the keys, so treat them as
        // no-ops — common in macOS arm64 prologues (`pacibsp`) and epilogues
        // (`autibsp`, `retab`). Without this we'd bail out at the first
        // instruction of /bin/ls.
        Opcode::PACIA
        | Opcode::PACIB
        | Opcode::PACDA
        | Opcode::PACDB
        | Opcode::PACIZA
        | Opcode::PACIZB
        | Opcode::PACDZA
        | Opcode::PACDZB
        | Opcode::PACGA
        | Opcode::PACIASP
        | Opcode::PACIAZ
        | Opcode::PACIA1716
        | Opcode::PACIA171615
        | Opcode::PACIASPPC
        | Opcode::PACNBIASPPC
        | Opcode::PACIBSP
        | Opcode::PACIBZ
        | Opcode::PACIB1716
        | Opcode::PACIB171615
        | Opcode::PACIBSPPC
        | Opcode::PACNBIBSPPC
        | Opcode::PACM
        | Opcode::AUTIA
        | Opcode::AUTIB
        | Opcode::AUTDA
        | Opcode::AUTDB
        | Opcode::AUTIZA
        | Opcode::AUTIZB
        | Opcode::AUTDZA
        | Opcode::AUTDZB
        | Opcode::AUTIASP
        | Opcode::AUTIAZ
        | Opcode::AUTIA1716
        | Opcode::AUTIA171615
        | Opcode::AUTIASPPC
        | Opcode::AUTIASPPCR
        | Opcode::AUTIBSP
        | Opcode::AUTIBZ
        | Opcode::AUTIB1716
        | Opcode::AUTIB171615
        | Opcode::AUTIBSPPC
        | Opcode::AUTIBSPPCR
        | Opcode::XPACI
        | Opcode::XPACD
        | Opcode::XPACLRI => true,

        _ => {
            log::warn!(
                "unimplemented aarch64 instruction: {} at 0x{:x}",
                ins,
                emu.regs_aarch64().pc
            );
            false
        }
    }
}
