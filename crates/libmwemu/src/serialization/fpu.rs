use std::convert::TryInto as _;

use serde::{Deserialize, Serialize};

use crate::fpu::FPU;
use crate::fpu::fpu_stack::FPUStack;

#[derive(Serialize, Deserialize)]
pub struct SerializableFPU {
    pub st: FPUStack,
    pub status: u16,
    pub st_depth: u8,
    pub tag: u16,
    pub stat: u16,
    pub ctrl: u16,
    pub ip: u64,
    pub err_off: u32,
    pub err_sel: u32,
    pub code_segment: u16,
    pub data_segment: u16,
    pub operand_ptr: u64,
    pub reserved: Vec<u8>,  // not a slice
    pub reserved2: Vec<u8>, // not a slice
    pub xmm: Vec<u128>,     // not a slice
    pub mxcsr: u32,
    pub fpu_control_word: u16,
    pub opcode: u16,
    pub trace: bool,
}

impl From<FPU> for SerializableFPU {
    fn from(fpu: FPU) -> Self {
        SerializableFPU {
            st: fpu.st,
            status: fpu.status,
            st_depth: fpu.st_depth,
            tag: fpu.tag,
            stat: fpu.stat,
            ctrl: fpu.ctrl,
            ip: fpu.ip,
            err_off: fpu.err_off,
            err_sel: fpu.err_sel,
            code_segment: fpu.code_segment,
            data_segment: fpu.data_segment,
            operand_ptr: fpu.operand_ptr,
            reserved: fpu.reserved.to_vec(),
            reserved2: fpu.reserved2.to_vec(),
            xmm: fpu.xmm.to_vec(),
            mxcsr: fpu.mxcsr,
            fpu_control_word: fpu.fpu_control_word,
            opcode: fpu.opcode,
            trace: fpu.trace,
        }
    }
}

impl From<SerializableFPU> for FPU {
    fn from(serialized: SerializableFPU) -> Self {
        FPU {
            st: serialized.st,
            status: serialized.status,
            st_depth: serialized.st_depth,
            tag: serialized.tag,
            stat: serialized.stat,
            ctrl: serialized.ctrl,
            ip: serialized.ip,
            err_off: serialized.err_off,
            err_sel: serialized.err_sel,
            code_segment: serialized.code_segment,
            data_segment: serialized.data_segment,
            operand_ptr: serialized.operand_ptr,
            reserved: serialized.reserved.try_into().unwrap(),
            reserved2: serialized.reserved2.try_into().unwrap(),
            xmm: serialized.xmm.try_into().unwrap(),
            mxcsr: serialized.mxcsr,
            fpu_control_word: serialized.fpu_control_word,
            opcode: serialized.opcode,
            trace: serialized.trace,
        }
    }
}

impl Default for SerializableFPU {
    fn default() -> Self {
        Self::new()
    }
}

impl SerializableFPU {
    pub fn new() -> Self {
        SerializableFPU {
            st: FPUStack::new(),
            status: 0,
            st_depth: 0,
            tag: 0,
            stat: 0,
            ctrl: 0,
            ip: 0,
            err_off: 0,
            err_sel: 0,
            code_segment: 0,
            data_segment: 0,
            operand_ptr: 0,
            reserved: vec![0; 80],
            reserved2: vec![0; 96],
            xmm: vec![0; 16],
            mxcsr: 0,
            fpu_control_word: 0,
            opcode: 0,
            trace: false,
        }
    }
}
