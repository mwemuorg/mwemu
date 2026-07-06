use iced_x86::Code;

/// Architecture-neutral decoded instruction.
///
/// Wraps the native instruction type for each supported ISA so that hooks,
/// display, tracing, and the run loop can operate on a single type without
/// caring which architecture is active.
///
/// The enum is `Copy` (both variants are small value types) and imposes no
/// heap allocation. It is cheap to construct, but the hottest no-observer
/// execution path avoids constructing it until hooks, tracing, or display code
/// need the architecture-neutral view.
#[derive(Clone, Copy)]
pub enum DecodedInstruction {
    X86(iced_x86::Instruction),
    AArch64(yaxpeax_arm::armv8::a64::Instruction),
}

impl DecodedInstruction {
    /// Address of the instruction (IP / PC).
    ///
    /// For x86 this comes from `ins.ip()` which the decoder sets.
    /// For aarch64 the decoder doesn't embed the PC in the instruction,
    /// so callers must use the PC value they already know.  We store 0
    /// here as a sentinel; the run loop should prefer `self.pc()` for
    /// the canonical address.
    #[inline]
    pub fn address(&self) -> u64 {
        match self {
            DecodedInstruction::X86(ins) => ins.ip(),
            DecodedInstruction::AArch64(_) => 0, // caller should use emu.pc()
        }
    }

    /// Size of the encoded instruction in bytes.
    #[inline]
    pub fn size(&self) -> usize {
        match self {
            DecodedInstruction::X86(ins) => ins.len(),
            DecodedInstruction::AArch64(_) => 4, // fixed-width ISA
        }
    }

    /// Returns `true` if this is a return instruction.
    #[inline]
    pub fn is_return(&self) -> bool {
        match self {
            DecodedInstruction::X86(ins) => {
                matches!(ins.code(), Code::Retnw | Code::Retnd | Code::Retnq)
            }
            DecodedInstruction::AArch64(ins) => ins.opcode == yaxpeax_arm::armv8::a64::Opcode::RET,
        }
    }

    /// Unwrap as an x86 instruction.  Panics on aarch64.
    #[inline]
    pub fn as_x86(&self) -> &iced_x86::Instruction {
        match self {
            DecodedInstruction::X86(ins) => ins,
            DecodedInstruction::AArch64(_) => panic!("as_x86() called on aarch64 instruction"),
        }
    }

    /// Unwrap as an aarch64 instruction.  Panics on x86.
    #[inline]
    pub fn as_aarch64(&self) -> &yaxpeax_arm::armv8::a64::Instruction {
        match self {
            DecodedInstruction::AArch64(ins) => ins,
            DecodedInstruction::X86(_) => panic!("as_aarch64() called on x86 instruction"),
        }
    }

    /// Returns `true` if this is an x86 instruction.
    #[inline]
    pub fn is_x86(&self) -> bool {
        matches!(self, DecodedInstruction::X86(_))
    }

    /// Returns `true` if this is an aarch64 instruction.
    #[inline]
    pub fn is_aarch64(&self) -> bool {
        matches!(self, DecodedInstruction::AArch64(_))
    }
}

impl std::fmt::Debug for DecodedInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodedInstruction::X86(ins) => write!(f, "X86({:?})", ins),
            DecodedInstruction::AArch64(ins) => write!(f, "AArch64({})", ins),
        }
    }
}
