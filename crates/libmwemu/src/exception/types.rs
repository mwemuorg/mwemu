use crate::windows::constants::errors::*;
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
