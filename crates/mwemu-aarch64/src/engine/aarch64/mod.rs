use crate::emu::Emu;
use instructions::atomic::RmwOp;
use instructions::fp_arith::{self, FpBinary, FpFused, FpUnary};
use instructions::fp_convert::{self, Rounding};
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
        Opcode::UMULH => {
            let a = helpers::read_reg(emu, &ins.operands[1]);
            let b = helpers::read_reg(emu, &ins.operands[2]);
            let result = ((a as u128) * (b as u128)) >> 64;
            helpers::write_reg(emu, &ins.operands[0], result as u64);
            true
        }
        Opcode::SMULH => {
            let a = helpers::read_reg(emu, &ins.operands[1]) as i64;
            let b = helpers::read_reg(emu, &ins.operands[2]) as i64;
            let result = ((a as i128) * (b as i128)) >> 64;
            helpers::write_reg(emu, &ins.operands[0], result as u64);
            true
        }
        Opcode::SMADDL => instructions::maddl::execute(emu, ins, true, false),
        Opcode::UMADDL => instructions::maddl::execute(emu, ins, false, false),
        Opcode::SMSUBL => instructions::maddl::execute(emu, ins, true, true),
        Opcode::UMSUBL => instructions::maddl::execute(emu, ins, false, true),
        Opcode::ADC => instructions::adc::execute(emu, ins, false, false),
        Opcode::ADCS => instructions::adc::execute(emu, ins, false, true),
        Opcode::SBC => instructions::adc::execute(emu, ins, true, false),
        Opcode::SBCS => instructions::adc::execute(emu, ins, true, true),
        Opcode::EON => instructions::eon::execute(emu, ins),
        Opcode::BICS => instructions::bics::execute(emu, ins),
        Opcode::BFM => instructions::bfm::execute(emu, ins),
        Opcode::CLS => instructions::cls::execute(emu, ins),
        Opcode::SDIV => instructions::sdiv::execute(emu, ins),
        Opcode::UDIV => instructions::udiv::execute(emu, ins),
        Opcode::ADR => instructions::adr::execute(emu, ins),
        Opcode::ADRP => instructions::adrp::execute(emu, ins),
        Opcode::CLZ => instructions::clz::execute(emu, ins),
        Opcode::LSLV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Lsl),
        Opcode::LSRV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Lsr),
        Opcode::ASRV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Asr),
        Opcode::RORV => instructions::shift::execute(emu, ins, helpers::ShiftOp::Ror),
        Opcode::SBFM => instructions::sbfm::execute(emu, ins),
        Opcode::UBFM => instructions::ubfm::execute(emu, ins),
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
        Opcode::LDPSW => instructions::ldpsw::execute(emu, ins),
        Opcode::LDNP => instructions::ldp::execute(emu, ins),
        Opcode::LDXR | Opcode::LDAXR | Opcode::LDAR | Opcode::LDAPR | Opcode::LDLAR => {
            instructions::exclusive::load(emu, ins, None)
        }
        Opcode::LDXRB | Opcode::LDAXRB | Opcode::LDARB | Opcode::LDAPRB => {
            instructions::exclusive::load(emu, ins, Some(1))
        }
        Opcode::LDXRH | Opcode::LDAXRH | Opcode::LDARH | Opcode::LDAPRH => {
            instructions::exclusive::load(emu, ins, Some(2))
        }
        Opcode::LDXP | Opcode::LDAXP => instructions::exclusive::load_pair(emu, ins),
        Opcode::LDAPUR => instructions::ldur::execute(emu, ins),
        Opcode::LDAPURB => instructions::ldurb::execute(emu, ins),
        Opcode::LDAPURH => instructions::ldurh::execute(emu, ins),
        Opcode::LDAPURSB => instructions::ldursb::execute(emu, ins),
        Opcode::LDAPURSH => instructions::ldursh::execute(emu, ins),
        Opcode::LDAPURSW => instructions::ldursw::execute(emu, ins),
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
        Opcode::STNP => instructions::stp::execute(emu, ins),
        Opcode::STXR | Opcode::STLXR => instructions::exclusive::store(emu, ins, None),
        Opcode::STXRB | Opcode::STLXRB => instructions::exclusive::store(emu, ins, Some(1)),
        Opcode::STXRH | Opcode::STLXRH => instructions::exclusive::store(emu, ins, Some(2)),
        Opcode::STXP | Opcode::STLXP => instructions::exclusive::store_pair(emu, ins),
        Opcode::STLR | Opcode::STLLR => instructions::exclusive::store_release(emu, ins, None),
        Opcode::STLRB => instructions::exclusive::store_release(emu, ins, Some(1)),
        Opcode::STLRH => instructions::exclusive::store_release(emu, ins, Some(2)),
        Opcode::STLUR => instructions::stur::execute(emu, ins),
        Opcode::STLURB => instructions::sturb::execute(emu, ins),
        Opcode::STLURH => instructions::sturh::execute(emu, ins),

        // --- LSE atomics ---
        Opcode::CAS(_) => instructions::atomic::cas(emu, ins, None),
        Opcode::CASB(_) => instructions::atomic::cas(emu, ins, Some(1)),
        Opcode::CASH(_) => instructions::atomic::cas(emu, ins, Some(2)),
        Opcode::CASP(_) => instructions::atomic::casp(emu, ins),
        Opcode::SWP(_) => atomic_rmw(emu, ins, None, RmwOp::Swp),
        Opcode::SWPB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Swp),
        Opcode::SWPH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Swp),
        Opcode::LDADD(_) => atomic_rmw(emu, ins, None, RmwOp::Add),
        Opcode::LDADDB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Add),
        Opcode::LDADDH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Add),
        Opcode::LDCLR(_) => atomic_rmw(emu, ins, None, RmwOp::Clr),
        Opcode::LDCLRB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Clr),
        Opcode::LDCLRH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Clr),
        Opcode::LDEOR(_) => atomic_rmw(emu, ins, None, RmwOp::Eor),
        Opcode::LDEORB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Eor),
        Opcode::LDEORH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Eor),
        Opcode::LDSET(_) => atomic_rmw(emu, ins, None, RmwOp::Set),
        Opcode::LDSETB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Set),
        Opcode::LDSETH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Set),
        Opcode::LDSMAX(_) => atomic_rmw(emu, ins, None, RmwOp::Smax),
        Opcode::LDSMAXB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Smax),
        Opcode::LDSMAXH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Smax),
        Opcode::LDSMIN(_) => atomic_rmw(emu, ins, None, RmwOp::Smin),
        Opcode::LDSMINB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Smin),
        Opcode::LDSMINH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Smin),
        Opcode::LDUMAX(_) => atomic_rmw(emu, ins, None, RmwOp::Umax),
        Opcode::LDUMAXB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Umax),
        Opcode::LDUMAXH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Umax),
        Opcode::LDUMIN(_) => atomic_rmw(emu, ins, None, RmwOp::Umin),
        Opcode::LDUMINB(_) => atomic_rmw(emu, ins, Some(1), RmwOp::Umin),
        Opcode::LDUMINH(_) => atomic_rmw(emu, ins, Some(2), RmwOp::Umin),
        Opcode::STUR => instructions::stur::execute(emu, ins),
        Opcode::STURB => instructions::sturb::execute(emu, ins),
        Opcode::STURH => instructions::sturh::execute(emu, ins),

        // --- Branches ---
        Opcode::B => instructions::b::execute(emu, ins),
        Opcode::BL => instructions::bl::execute(emu, ins),
        Opcode::BR | Opcode::BRAA | Opcode::BRAAZ | Opcode::BRAB | Opcode::BRABZ => {
            instructions::br::execute(emu, ins)
        }
        Opcode::BLR | Opcode::BLRAA | Opcode::BLRAAZ | Opcode::BLRAB | Opcode::BLRABZ => {
            instructions::blr::execute(emu, ins)
        }
        Opcode::RET | Opcode::RETAA | Opcode::RETAB => instructions::ret::execute(emu, ins),
        Opcode::CBZ => instructions::cbz::execute(emu, ins, true),
        Opcode::CBNZ => instructions::cbz::execute(emu, ins, false),
        Opcode::TBZ => instructions::tbz::execute(emu, ins, true),
        Opcode::TBNZ => instructions::tbz::execute(emu, ins, false),
        Opcode::Bcc(cond) => instructions::bcc::execute(emu, ins, cond),

        // --- Conditional compare ---
        Opcode::CCMP => instructions::ccmp::execute(emu, ins, true),
        Opcode::CCMN => instructions::ccmp::execute(emu, ins, false),

        // --- Conditional select ---
        Opcode::CSEL => instructions::csel::execute(emu, ins),
        Opcode::CSINC => instructions::csinc::execute(emu, ins),
        Opcode::CSINV => instructions::csinv::execute(emu, ins),
        Opcode::CSNEG => instructions::csneg::execute(emu, ins),

        // --- System ---
        Opcode::SVC => instructions::svc::execute(emu, ins),
        Opcode::MRS => instructions::mrs::execute(emu, ins),
        Opcode::MSR => instructions::msr::execute(emu, ins),
        // --- SIMD/NEON ---
        Opcode::MOVI | Opcode::FMOV | Opcode::DUP | Opcode::UMOV | Opcode::INS | Opcode::NOT => {
            instructions::simd::execute(emu, ins)
        }

        // --- Scalar floating point ---
        Opcode::FADD => fp_arith::binary(emu, ins, FpBinary::Add),
        Opcode::FSUB => fp_arith::binary(emu, ins, FpBinary::Sub),
        Opcode::FMUL => fp_arith::binary(emu, ins, FpBinary::Mul),
        Opcode::FDIV => fp_arith::binary(emu, ins, FpBinary::Div),
        Opcode::FMAX => fp_arith::binary(emu, ins, FpBinary::Max),
        Opcode::FMIN => fp_arith::binary(emu, ins, FpBinary::Min),
        Opcode::FMAXNM => fp_arith::binary(emu, ins, FpBinary::MaxNm),
        Opcode::FMINNM => fp_arith::binary(emu, ins, FpBinary::MinNm),
        Opcode::FNMUL => fp_arith::binary(emu, ins, FpBinary::NMul),
        Opcode::FABS => fp_arith::unary(emu, ins, FpUnary::Abs),
        Opcode::FNEG => fp_arith::unary(emu, ins, FpUnary::Neg),
        Opcode::FSQRT => fp_arith::unary(emu, ins, FpUnary::Sqrt),
        Opcode::FCVT => fp_arith::unary(emu, ins, FpUnary::Mov),
        Opcode::FMADD => fp_arith::fused(emu, ins, FpFused::MAdd),
        Opcode::FMSUB => fp_arith::fused(emu, ins, FpFused::MSub),
        Opcode::FNMADD => fp_arith::fused(emu, ins, FpFused::NMAdd),
        Opcode::FNMSUB => fp_arith::fused(emu, ins, FpFused::NMSub),
        Opcode::FCMP | Opcode::FCMPE => instructions::fp_compare::fcmp(emu, ins),
        Opcode::FCCMP | Opcode::FCCMPE => instructions::fp_compare::fccmp(emu, ins),
        Opcode::FCSEL => instructions::fp_compare::fcsel(emu, ins),
        Opcode::SCVTF => fp_convert::int_to_fp(emu, ins, true),
        Opcode::UCVTF => fp_convert::int_to_fp(emu, ins, false),
        Opcode::FCVTNS => fp_convert::fp_to_int(emu, ins, true, Rounding::Nearest),
        Opcode::FCVTPS => fp_convert::fp_to_int(emu, ins, true, Rounding::Up),
        Opcode::FCVTMS => fp_convert::fp_to_int(emu, ins, true, Rounding::Down),
        Opcode::FCVTAS => fp_convert::fp_to_int(emu, ins, true, Rounding::Away),
        Opcode::FCVTZS => fp_convert::fp_to_int(emu, ins, true, Rounding::Zero),
        Opcode::FCVTNU => fp_convert::fp_to_int(emu, ins, false, Rounding::Nearest),
        Opcode::FCVTPU => fp_convert::fp_to_int(emu, ins, false, Rounding::Up),
        Opcode::FCVTMU => fp_convert::fp_to_int(emu, ins, false, Rounding::Down),
        Opcode::FCVTAU => fp_convert::fp_to_int(emu, ins, false, Rounding::Away),
        Opcode::FCVTZU => fp_convert::fp_to_int(emu, ins, false, Rounding::Zero),
        Opcode::FRINTN | Opcode::FRINTX | Opcode::FRINTI => {
            fp_convert::frint(emu, ins, Rounding::Nearest)
        }
        Opcode::FRINTP => fp_convert::frint(emu, ins, Rounding::Up),
        Opcode::FRINTM => fp_convert::frint(emu, ins, Rounding::Down),
        Opcode::FRINTA => fp_convert::frint(emu, ins, Rounding::Away),
        Opcode::FRINTZ => fp_convert::frint(emu, ins, Rounding::Zero),

        Opcode::PRFM | Opcode::PRFUM => true, // prefetch hints have no architectural effect
        Opcode::HINT => true,                 // NOP is encoded as HINT
        Opcode::DMB(_) | Opcode::DSB(_) | Opcode::ISB => true, // barriers are no-ops in emulation
        Opcode::CLREX => {
            let cur = emu.current_thread_id;
            emu.threads[cur].exclusive = None;
            true
        }
        // ARMv8.3 Pointer Authentication — signing (PAC*): inserts a
        // signature into the high bits.  We don't model keys, so these are
        // no-ops.
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
        | Opcode::PACM => true,

        // ARMv8.3 Pointer Authentication — authentication (AUT*) and
        // strip (XPAC*): remove the PAC signature bits from a register.
        Opcode::AUTIA | Opcode::AUTIB | Opcode::AUTDA | Opcode::AUTDB => {
            instructions::pac::execute_aut_reg(emu, ins)
        }
        Opcode::AUTIZA | Opcode::AUTIZB | Opcode::AUTDZA | Opcode::AUTDZB => {
            instructions::pac::execute_aut_reg(emu, ins)
        }
        Opcode::AUTIASP
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
        | Opcode::AUTIBSPPCR => instructions::pac::execute_aut_implicit(emu, ins),
        Opcode::XPACI | Opcode::XPACD => instructions::pac::execute_aut_reg(emu, ins),
        Opcode::XPACLRI => {
            let lr = emu.regs_aarch64().x[30];
            emu.regs_aarch64_mut().x[30] = helpers::pac_strip(lr);
            true
        }

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

fn atomic_rmw(emu: &mut Emu, ins: &Instruction, bytes: Option<u64>, op: RmwOp) -> bool {
    instructions::atomic::rmw(emu, ins, bytes, op)
}
