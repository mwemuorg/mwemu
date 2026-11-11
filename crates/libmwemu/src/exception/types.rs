use crate::maps::Maps;
use crate::windows::constants::errors::*;
use crate::windows::constants::exceptions::*;
#[derive(Clone, Copy, Debug)]
pub enum ExceptionType {
    Int3,                      // int 3 breakpoint
    Div0,                      // division by zero
    SignChangeOnDivision,      // sign change exception on division
    PopfCannotReadStack,       // popf cannot read stack
    WritingWord,               // exception writing word
    SettingRipToNonMappedAddr, // setting rip to non mapped addr
    QWordDereferencing,        // error dereferencing qword
    DWordDereferencing,        // error dereferencing dword
    WordDereferencing,         // error dereferencing word
    ByteDereferencing,         // error dereferencing byte
    BadAddressDereferencing,   // exception dereferencing bad address
    SettingXmmOperand,         // exception setting xmm operand
    ReadingXmmOperand,         // exception reading xmm operand
    HeapNoMemory,              // HeapAlloc failure with HEAP_GENERATE_EXCEPTIONS
    IllegalInstruction,        // #UD — unimplemented or invalid opcode
}

impl PartialEq for ExceptionType {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

impl std::fmt::Display for ExceptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExceptionType::Int3 => write!(f, "int 3"),
            ExceptionType::Div0 => write!(f, "division by zero"),
            ExceptionType::SignChangeOnDivision => write!(f, "sign change exception on division"),
            ExceptionType::PopfCannotReadStack => write!(f, "popf cannot read stack"),
            ExceptionType::WritingWord => write!(f, "exception writing word"),
            ExceptionType::SettingRipToNonMappedAddr => write!(f, "setting rip to non mapped addr"),
            ExceptionType::QWordDereferencing => write!(f, "error dereferencing qword"),
            ExceptionType::DWordDereferencing => write!(f, "error dereferencing dword"),
            ExceptionType::WordDereferencing => write!(f, "error dereferencing word"),
            ExceptionType::ByteDereferencing => write!(f, "error dereferencing byte"),
            ExceptionType::BadAddressDereferencing => {
                write!(f, "exception dereferencing bad address")
            }
            ExceptionType::SettingXmmOperand => write!(f, "exception setting xmm operand"),
            ExceptionType::ReadingXmmOperand => write!(f, "exception reading xmm operand"),
            ExceptionType::HeapNoMemory => write!(f, "heap out of memory"),
            ExceptionType::IllegalInstruction => write!(f, "illegal instruction (#UD)"),
        }
    }
}

pub fn exception_type_code(ex_type: ExceptionType) -> u32 {
    match ex_type {
        ExceptionType::Int3 => STATUS_BREAKPOINT,
        ExceptionType::Div0 => STATUS_INTEGER_DIVIDE_BY_ZERO,
        ExceptionType::SignChangeOnDivision => STATUS_INTEGER_OVERFLOW,
        ExceptionType::PopfCannotReadStack => STATUS_POPF_CANNOT_READ_STACK,
        ExceptionType::WritingWord => STATUS_WRITING_WORD,
        ExceptionType::SettingRipToNonMappedAddr => STATUS_READING_RIP,
        ExceptionType::QWordDereferencing => STATUS_QWORD_DEREFERENCING,
        ExceptionType::DWordDereferencing => STATUS_DWORD_DEREFERENCING,
        ExceptionType::WordDereferencing => STATUS_WORD_DEREFERENCING,
        ExceptionType::ByteDereferencing => STATUS_BYTE_DEREFERENCING,
        ExceptionType::BadAddressDereferencing => STATUS_BAD_ADDRESS_DEREFERENCING,
        ExceptionType::SettingXmmOperand => STATUS_SETTING_XMM_OPERAND,
        ExceptionType::ReadingXmmOperand => STATUS_READING_XMM_OPERAND,
        ExceptionType::HeapNoMemory => STATUS_NO_MEMORY as u32,
        ExceptionType::IllegalInstruction => STATUS_ILLEGAL_INSTRUCTION,
    }
}

pub fn exception_type_from_code(code: u32) -> ExceptionType {
    match code {
        STATUS_BREAKPOINT => ExceptionType::Int3,
        STATUS_INTEGER_DIVIDE_BY_ZERO => ExceptionType::Div0,
        STATUS_INTEGER_OVERFLOW => ExceptionType::SignChangeOnDivision,
        STATUS_POPF_CANNOT_READ_STACK => ExceptionType::PopfCannotReadStack,
        STATUS_WRITING_WORD => ExceptionType::WritingWord,
        STATUS_READING_RIP => ExceptionType::SettingRipToNonMappedAddr,
        STATUS_QWORD_DEREFERENCING => ExceptionType::QWordDereferencing,
        STATUS_DWORD_DEREFERENCING => ExceptionType::DWordDereferencing,
        STATUS_WORD_DEREFERENCING => ExceptionType::WordDereferencing,
        STATUS_BYTE_DEREFERENCING => ExceptionType::ByteDereferencing,
        STATUS_BAD_ADDRESS_DEREFERENCING => ExceptionType::BadAddressDereferencing,
        STATUS_SETTING_XMM_OPERAND => ExceptionType::SettingXmmOperand,
        STATUS_READING_XMM_OPERAND => ExceptionType::ReadingXmmOperand,
        x if x == STATUS_NO_MEMORY as u32 => ExceptionType::HeapNoMemory,
        STATUS_ILLEGAL_INSTRUCTION => ExceptionType::IllegalInstruction,
        _ => ExceptionType::BadAddressDereferencing,
    }
}

// EXCEPTION_RECORD guest layout.
const RECORD_FLAGS: u64 = 4;
const RECORD_NESTED: u64 = 8;
const RECORD_ADDRESS32: u64 = 12;
const RECORD_ADDRESS64: u64 = 16;
const RECORD_COUNT32: u64 = 16;
const RECORD_COUNT64: u64 = 24;
const RECORD_INFORMATION32: u64 = 20;
const RECORD_INFORMATION64: u64 = 32;

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub(crate) enum MemoryAccess {
    Read = EXCEPTION_READ_FAULT,
    Write = EXCEPTION_WRITE_FAULT,
    Execute = EXCEPTION_EXECUTE_FAULT,
}

/// Guest-visible exception data, independent of the diagnostic hook's ExceptionType.
#[derive(Clone, Debug)]
pub(crate) struct ExceptionRecord {
    pub code: u32,
    pub flags: u32,
    pub nested: u64,
    pub address: u64,
    pub information: Vec<u64>,
}

impl ExceptionRecord {
    pub fn new(ex_type: ExceptionType, address: u64) -> Self {
        use ExceptionType::*;
        let code = match ex_type {
            PopfCannotReadStack
            | WritingWord
            | SettingRipToNonMappedAddr
            | QWordDereferencing
            | DWordDereferencing
            | WordDereferencing
            | ByteDereferencing
            | BadAddressDereferencing
            | SettingXmmOperand
            | ReadingXmmOperand => STATUS_ACCESS_VIOLATION as u32,
            _ => exception_type_code(ex_type),
        };
        Self {
            code,
            flags: 0,
            nested: 0,
            address,
            information: Vec::new(),
        }
    }

    pub fn access_violation(pc: u64, address: u64, access: MemoryAccess) -> Self {
        Self {
            information: vec![access as u64, address],
            ..Self::new(ExceptionType::BadAddressDereferencing, pc)
        }
    }

    pub fn save(&self, maps: &mut Maps, address: u64, is_64: bool) {
        maps.write_dword(address, self.code);
        maps.write_dword(address + RECORD_FLAGS, self.flags);
        let count = self.information.len().min(EXCEPTION_MAXIMUM_PARAMETERS);
        let (information, stride) = if is_64 {
            maps.write_qword(address + RECORD_NESTED, self.nested);
            maps.write_qword(address + RECORD_ADDRESS64, self.address);
            maps.write_dword(address + RECORD_COUNT64, count as u32);
            (RECORD_INFORMATION64, size_of::<u64>() as u64)
        } else {
            maps.write_dword(address + RECORD_NESTED, self.nested as u32);
            maps.write_dword(address + RECORD_ADDRESS32, self.address as u32);
            maps.write_dword(address + RECORD_COUNT32, count as u32);
            (RECORD_INFORMATION32, size_of::<u32>() as u64)
        };
        for (i, &value) in self.information.iter().take(count).enumerate() {
            let target = address + information + i as u64 * stride;
            if is_64 {
                maps.write_qword(target, value);
            } else {
                maps.write_dword(target, value as u32);
            }
        }
    }

    pub fn load(maps: &Maps, address: u64, is_64: bool) -> Option<Self> {
        let (nested, fault_pc, count, information, stride) = if is_64 {
            (
                maps.read_qword(address + RECORD_NESTED)?,
                maps.read_qword(address + RECORD_ADDRESS64)?,
                maps.read_dword(address + RECORD_COUNT64)?,
                RECORD_INFORMATION64,
                size_of::<u64>() as u64,
            )
        } else {
            (
                maps.read_dword(address + RECORD_NESTED)? as u64,
                maps.read_dword(address + RECORD_ADDRESS32)? as u64,
                maps.read_dword(address + RECORD_COUNT32)?,
                RECORD_INFORMATION32,
                size_of::<u32>() as u64,
            )
        };
        if count as usize > EXCEPTION_MAXIMUM_PARAMETERS {
            return None;
        }
        let values = (0..count)
            .map(|i| {
                let target = address + information + i as u64 * stride;
                if is_64 {
                    maps.read_qword(target)
                } else {
                    maps.read_dword(target).map(u64::from)
                }
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self {
            code: maps.read_dword(address)?,
            flags: maps.read_dword(address + RECORD_FLAGS)?,
            nested,
            address: fault_pc,
            information: values,
        })
    }
}
