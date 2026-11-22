use goblin::pe::exception::{
    RuntimeFunction, StackFrameOffset, UnwindHandler, UnwindInfo, UnwindOperation,
};
use iced_x86::{Decoder, DecoderOptions, Mnemonic, OpKind, Register};

mod dispatch;
mod scope;
mod search;
mod target;
pub(crate) use dispatch::{dispatch_exception_api, raise_exception_api};
pub(crate) use scope::c_specific_handler;
pub(crate) use search::next_exception_handler;
pub(crate) use target::unwind;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::arch::x86::context::context64::Context64;
use crate::emu::Emu;
use crate::maps::{Maps, mem64::Permission};
use crate::winapi::policy::{ApiStopReason, api_try, invalid_unwind};
use crate::windows::constants::exceptions::{
    CONTEXT_CONTROL, CONTEXT_INTEGER, UNW_FLAG_CHAININFO, UNW_FLAG_EHANDLER, UNW_FLAG_UHANDLER,
};

const RSP: usize = 4;
const REGISTER_COUNT: usize = 16;
const WORD: u64 = 8;
const XMM_SIZE: u64 = 16;
const UNWIND_HEADER: usize = 4;
const MAX_CHAIN_DEPTH: usize = 32;

const FRAME_LIMIT: usize = 4096;

type FrameResult<T> = Result<T, ApiStopReason>;

fn add(address: u64, offset: u64) -> FrameResult<u64> {
    address
        .checked_add(offset)
        .ok_or(ApiStopReason::InvalidFrame)
}

fn load(maps: &Maps, address: u64) -> FrameResult<u64> {
    maps.read_qword(address).ok_or(ApiStopReason::InvalidMemory)
}

struct Frame {
    context: u64,
    pointers: u64,
    registers: [u64; REGISTER_COUNT],
    establisher: u64,
    handler_data: Option<u64>,
    writes: Vec<(u64, Vec<u8>)>,
}

impl Frame {
    fn read(maps: &Maps, context: u64, pointers: u64) -> FrameResult<Self> {
        let mut registers = [0; REGISTER_COUNT];
        for (index, value) in registers.iter_mut().enumerate() {
            *value = load(
                maps,
                add(context, Context64::RAX_OFFSET + index as u64 * WORD)?,
            )?;
        }
        Ok(Self {
            context,
            pointers,
            registers,
            establisher: 0,
            handler_data: None,
            writes: Vec::new(),
        })
    }

    fn store(&mut self, address: u64, value: u64) {
        self.writes.push((address, value.to_le_bytes().to_vec()));
    }

    fn saved_pointer(&mut self, index: usize, source: u64, integer: bool) -> FrameResult<()> {
        if self.pointers != 0 {
            // KNONVOLATILE_CONTEXT_POINTERS places the 16 XMM pointers before the GPR pointers.
            let slot = index + if integer { REGISTER_COUNT } else { 0 };
            self.store(add(self.pointers, slot as u64 * WORD)?, source);
        }
        Ok(())
    }

    fn restore_register(&mut self, maps: &Maps, index: usize, source: u64) -> FrameResult<()> {
        let value = load(maps, source)?;
        self.registers[index] = value;
        self.store(
            add(self.context, Context64::RAX_OFFSET + index as u64 * WORD)?,
            value,
        );
        self.saved_pointer(index, source, true)
    }

    fn pop_register(&mut self, maps: &Maps, index: usize) -> FrameResult<()> {
        let sp = self.registers[RSP];
        self.restore_register(maps, index, sp)?;
        self.registers[RSP] = add(sp, WORD)?;
        Ok(())
    }

    fn return_to_caller(&mut self, maps: &Maps, extra: u64) -> FrameResult<()> {
        let sp = self.registers[RSP];
        self.store(add(self.context, Context64::RIP_OFFSET)?, load(maps, sp)?);
        self.registers[RSP] = add(sp, WORD + extra)?;
        Ok(())
    }

    fn apply_code(&mut self, maps: &Maps, op: UnwindOperation, fixed_sp: u64) -> FrameResult<bool> {
        use UnwindOperation::*;
        let address = |offset| match offset {
            StackFrameOffset::FP(bytes) | StackFrameOffset::RSP(bytes) => {
                add(fixed_sp, bytes as u64)
            }
        };
        match op {
            PushNonVolatile(reg) => self.pop_register(maps, reg.0 as usize)?,
            Alloc(bytes) => self.registers[RSP] = add(self.registers[RSP], bytes as u64)?,
            SetFPRegister => self.registers[RSP] = fixed_sp,
            SaveNonVolatile(reg, offset) => {
                self.restore_register(maps, reg.0 as usize, address(offset)?)?
            }
            SaveXMM128(reg, offset) => {
                // Goblin encodes XMM0..XMM15 as 17..32, following the GPRs and RIP.
                let index = reg
                    .0
                    .checked_sub(17)
                    .filter(|&i| i < REGISTER_COUNT as u8)
                    .ok_or(ApiStopReason::InvalidFrame)?;
                let source = address(offset)?;
                let data = maps
                    .try_read_bytes(source, XMM_SIZE as usize)
                    .ok_or(ApiStopReason::InvalidMemory)?;
                self.writes.push((
                    add(
                        self.context,
                        Context64::XMM0_OFFSET + index as u64 * XMM_SIZE,
                    )?,
                    data.into_owned(),
                ));
                self.saved_pointer(index as usize, source, false)?;
            }
            PushMachineFrame(error_code) => {
                let stack = add(self.registers[RSP], u64::from(error_code) * WORD)?;
                self.store(
                    add(self.context, Context64::RIP_OFFSET)?,
                    load(maps, stack)?,
                );
                self.registers[RSP] = load(maps, add(stack, 3 * WORD)?)?;
                return Ok(true);
            }
            _ => return Err(ApiStopReason::InvalidFrame),
        }
        Ok(false)
    }

    fn commit(mut self, maps: &mut Maps) -> FrameResult<()> {
        self.store(
            add(self.context, Context64::RAX_OFFSET + RSP as u64 * WORD)?,
            self.registers[RSP],
        );
        if self
            .writes
            .iter()
            .any(|(at, data)| !maps.can_access_range(*at, data.len(), Permission::WRITE))
        {
            return Err(ApiStopReason::InvalidMemory);
        }
        for (at, data) in self.writes {
            if !maps.write_bytes(at, &data) {
                return Err(ApiStopReason::InvalidMemory);
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct Epilog {
    frame_register: Option<usize>,
    displacement: u64,
    pops: Vec<usize>,
    return_bytes: u64,
}

fn epilog(
    maps: &Maps,
    start: u64,
    end: u64,
    mut pc: u64,
    frame_register: u8,
) -> FrameResult<Option<Epilog>> {
    let mut plan = Epilog::default();
    for step in 0..REGISTER_COUNT + 2 {
        let length = end.saturating_sub(pc).min(15) as usize;
        if length == 0 {
            return Ok(None);
        }
        let bytes = maps
            .try_read_bytes(pc, length)
            .ok_or(ApiStopReason::InvalidMemory)?;
        let ins = Decoder::with_ip(64, &bytes, pc, DecoderOptions::NONE).decode();
        match ins.mnemonic() {
            Mnemonic::Add
                if step == 0
                    && ins.op0_register() == Register::RSP
                    && matches!(
                        ins.op1_kind(),
                        OpKind::Immediate8to64 | OpKind::Immediate32to64
                    ) =>
            {
                plan.displacement = ins.immediate(1);
            }
            Mnemonic::Lea
                if step == 0
                    && ins.op0_register() == Register::RSP
                    && ins.memory_index() == Register::None
                    && ins.memory_base().is_gpr64()
                    && frame_register != 0
                    && ins.memory_base().number() == frame_register as usize =>
            {
                plan.frame_register = Some(frame_register as usize);
                plan.displacement = ins.memory_displacement64();
            }
            Mnemonic::Pop
                if ins.op0_register().is_gpr64() && ins.op0_register() != Register::RSP =>
            {
                plan.pops.push(ins.op0_register().number());
            }
            Mnemonic::Ret => {
                if ins.op_count() != 0 {
                    plan.return_bytes = ins.immediate16() as u64;
                }
                return Ok(Some(plan));
            }
            Mnemonic::Jmp => {
                let tail = match ins.op0_kind() {
                    OpKind::NearBranch64 => !(start..end).contains(&ins.near_branch_target()),
                    OpKind::Register => bytes.first().is_some_and(|rex| rex & 0xf8 == 0x48),
                    OpKind::Memory => bytes
                        .iter()
                        .position(|&b| b == 0xff)
                        .and_then(|i| bytes.get(i + 1))
                        .is_some_and(|modrm| modrm >> 6 == 0),
                    _ => false,
                };
                return Ok(tail.then_some(plan));
            }
            _ => return Ok(None),
        }
        pc = ins.next_ip();
    }
    Ok(None)
}

struct UnwindRequest {
    kind: u64,
    image: u64,
    pc: u64,
    entry: u64,
}

impl UnwindRequest {
    fn walk(&self, maps: &Maps, frame: &mut Frame) -> FrameResult<u64> {
        let dword = |at| maps.read_dword(at).ok_or(ApiStopReason::InvalidMemory);
        let start = add(self.image, dword(self.entry)? as u64)?;
        let end = add(self.image, dword(add(self.entry, 4)?)? as u64)?;
        if !(start..end).contains(&self.pc) {
            return Err(ApiStopReason::InvalidFrame);
        }
        let mut unwind_rva = dword(add(self.entry, 8)?)?;
        let mut machine_frame = false;
        for depth in 0..MAX_CHAIN_DEPTH {
            let header = add(self.image, unwind_rva as u64)?;
            let bytes = maps
                .try_read_bytes(header, UNWIND_HEADER)
                .ok_or(ApiStopReason::InvalidMemory)?;
            let flags = bytes[0] >> 3;
            if bytes[0] & 7 != 1
                || flags & !(UNW_FLAG_CHAININFO | UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER) != 0
                || flags & UNW_FLAG_CHAININFO != 0 && flags != UNW_FLAG_CHAININFO
            {
                return Err(ApiStopReason::InvalidFrame);
            }
            let trailer = UNWIND_HEADER + (bytes[2] as usize).next_multiple_of(2) * 2;
            let size = trailer
                + match flags {
                    UNW_FLAG_CHAININFO => size_of::<RuntimeFunction>(),
                    0 => 0,
                    _ => 4,
                };
            let bytes = maps
                .try_read_bytes(header, size)
                .ok_or(ApiStopReason::InvalidMemory)?;
            let info = UnwindInfo::parse(&bytes, 0).map_err(|_| ApiStopReason::InvalidFrame)?;
            let offset = if depth == 0 {
                self.pc - start
            } else {
                u64::MAX
            };
            let in_prolog = offset < info.size_of_prolog as u64;
            let codes = info
                .unwind_codes()
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| ApiStopReason::InvalidFrame)?;
            let fp_set = info.frame_register.0 != 0
                && (!in_prolog
                    || codes.iter().any(|code| {
                        code.operation == UnwindOperation::SetFPRegister
                            && code.code_offset as u64 <= offset
                    }));
            let fixed_sp = if fp_set {
                frame.registers[info.frame_register.0 as usize]
                    .checked_sub(info.frame_register_offset as u64)
                    .ok_or(ApiStopReason::InvalidFrame)?
            } else {
                frame.registers[RSP]
            };
            if depth == 0 {
                frame.establisher = fixed_sp;
                if !in_prolog
                    && let Some(plan) = epilog(maps, start, end, self.pc, info.frame_register.0)?
                {
                    let base = frame.registers[plan.frame_register.unwrap_or(RSP)];
                    frame.registers[RSP] = base
                        .checked_add_signed(plan.displacement as i64)
                        .ok_or(ApiStopReason::InvalidFrame)?;
                    for index in plan.pops {
                        frame.pop_register(maps, index)?;
                    }
                    frame.return_to_caller(maps, plan.return_bytes)?;
                    return Ok(0);
                }
            }
            for code in codes {
                if code.code_offset as u64 <= offset {
                    if machine_frame {
                        return Err(ApiStopReason::InvalidFrame);
                    }
                    machine_frame = frame.apply_code(maps, code.operation, fixed_sp)?;
                }
            }
            if let Some(chain) = info.chained_info {
                unwind_rva = chain.unwind_info_address;
                continue;
            }
            if !machine_frame {
                frame.return_to_caller(maps, 0)?;
            }
            if !in_prolog
                && self.kind & flags as u64 != 0
                && let Some(handler) = info.handler
            {
                let (UnwindHandler::ExceptionHandler(rva, _)
                | UnwindHandler::TerminationHandler(rva, _)) = handler;
                frame.handler_data = Some(add(header, trailer as u64 + 4)?);
                return add(self.image, rva as u64);
            }
            return Ok(0);
        }
        Err(ApiStopReason::InvalidFrame)
    }
}

pub(crate) fn capture_context(emu: &mut Emu, destination: u64, pc: u64) -> FrameResult<()> {
    if !emu
        .maps
        .validate_write_range(destination, Context64::GUEST_SIZE)
    {
        return Err(ApiStopReason::InvalidMemory);
    }
    let mut regs = *emu.regs();
    regs.rip = pc;
    let flags = emu.flags_snapshot().dump();
    let parts = CONTEXT_CONTROL | CONTEXT_INTEGER;
    let mxcsr = emu.fpu().mxcsr;
    emu.maps
        .write_dword(destination + Context64::FLAGS_OFFSET, parts);
    Context64::new(&regs).save_selected(destination, &mut emu.maps, parts, flags);
    emu.maps
        .write_dword(destination + Context64::MXCSR_OFFSET, mxcsr);
    emu.maps
        .write_dword(destination + Context64::FXSAVE_MXCSR_OFFSET, mxcsr);
    for index in 0..REGISTER_COUNT {
        let value = regs.get_xmm_reg(Register::XMM0 + index as u32);
        emu.maps.write_bytes(
            destination + Context64::XMM0_OFFSET + index as u64 * XMM_SIZE,
            &value.to_le_bytes(),
        );
    }
    Ok(())
}

pub(super) fn restore_context(emu: &mut Emu, source: u64) -> FrameResult<()> {
    if !emu
        .maps
        .can_access_range(source, Context64::GUEST_SIZE as usize, Permission::READ)
    {
        return Err(ApiStopReason::InvalidMemory);
    }
    let mut regs = *emu.regs();
    let mut saved = Context64::new(&regs);
    saved.load_selected(source, &emu.maps, CONTEXT_CONTROL | CONTEXT_INTEGER);
    saved.sync(&mut regs);
    for index in 0..REGISTER_COUNT {
        let at = source + Context64::XMM0_OFFSET + index as u64 * XMM_SIZE;
        let value = emu
            .maps
            .read_128bits_le(at)
            .ok_or(ApiStopReason::InvalidMemory)?;
        regs.set_xmm_reg(Register::XMM0 + index as u32, value);
    }
    let flags = emu
        .maps
        .read_dword(source + Context64::EFLAGS_OFFSET)
        .ok_or(ApiStopReason::InvalidMemory)?;
    let mxcsr = emu
        .maps
        .read_dword(source + Context64::MXCSR_OFFSET)
        .ok_or(ApiStopReason::InvalidMemory)?;
    *emu.regs_mut() = regs;
    emu.flags_overwrite_mut().load(flags);
    emu.fpu_mut().mxcsr = mxcsr;
    emu.force_reload = true;
    Ok(())
}

pub(super) fn copy_context(maps: &mut Maps, source: u64, destination: u64) -> FrameResult<()> {
    if !maps.validate_write_range(destination, Context64::GUEST_SIZE) {
        return Err(ApiStopReason::InvalidMemory);
    }
    let bytes = maps
        .try_read_bytes(source, Context64::GUEST_SIZE as usize)
        .ok_or(ApiStopReason::InvalidMemory)?
        .into_owned();
    if !maps.write_bytes(destination, &bytes) {
        return Err(ApiStopReason::InvalidMemory);
    }
    Ok(())
}

struct StackStep {
    frame: Frame,
    image: u64,
    entry: u64,
    handler: u64,
}

fn walk_frame(emu: &Emu, context: u64, pc: u64, kind: u8) -> FrameResult<StackStep> {
    let mut frame = Frame::read(&emu.maps, context, 0)?;
    let (image, entry) = emu.runtime_function_entry(pc).unwrap_or_default();
    let handler = if entry == 0 {
        frame.establisher = frame.registers[RSP];
        frame.return_to_caller(&emu.maps, 0)?;
        0
    } else {
        UnwindRequest {
            kind: kind as u64,
            image,
            pc,
            entry,
        }
        .walk(&emu.maps, &mut frame)?
    };
    Ok(StackStep {
        frame,
        image,
        entry,
        handler,
    })
}

pub(super) struct DispatcherContext {
    pc: u64,
    image: u64,
    entry: u64,
    establisher: u64,
    target: u64,
    context: u64,
    handler: u64,
    data: u64,
    history: u64,
}

impl DispatcherContext {
    const CONTROL_PC: u64 = 0;
    const IMAGE_BASE: u64 = WORD;
    const FUNCTION_ENTRY: u64 = 2 * WORD;
    pub(super) const ESTABLISHER_FRAME: u64 = 3 * WORD;
    const TARGET_IP: u64 = 4 * WORD;
    const CONTEXT_RECORD: u64 = 5 * WORD;
    const LANGUAGE_HANDLER: u64 = 6 * WORD;
    const HANDLER_DATA: u64 = 7 * WORD;
    const HISTORY_TABLE: u64 = 8 * WORD;
    const SCOPE_INDEX: u64 = 9 * WORD;
    const SIZE: u64 = Self::SCOPE_INDEX + WORD;

    fn write(&self, maps: &mut Maps, address: u64) -> FrameResult<()> {
        if !maps.validate_write_range(address, Self::SIZE) {
            return Err(ApiStopReason::InvalidMemory);
        }
        // The final DWORD pair is ScopeIndex followed by alignment padding.
        let fields = [
            (Self::CONTROL_PC, self.pc),
            (Self::IMAGE_BASE, self.image),
            (Self::FUNCTION_ENTRY, self.entry),
            (Self::ESTABLISHER_FRAME, self.establisher),
            (Self::TARGET_IP, self.target),
            (Self::CONTEXT_RECORD, self.context),
            (Self::LANGUAGE_HANDLER, self.handler),
            (Self::HANDLER_DATA, self.data),
            (Self::HISTORY_TABLE, self.history),
            (Self::SCOPE_INDEX, 0),
        ];
        for (offset, value) in fields {
            maps.write_qword(address + offset, value);
        }
        Ok(())
    }
}

pub(crate) fn virtual_unwind(emu: &mut Emu, library: &str) {
    let call = WinApi(Stdcall);
    let [
        kind,
        image,
        pc,
        entry,
        context,
        handler_data,
        establisher,
        pointers,
    ] = api_try!(emu, call.args(emu));
    let request = UnwindRequest {
        kind,
        image,
        pc,
        entry,
    };
    log_red!(
        emu,
        "{library}!RtlVirtualUnwind control_pc={pc:#x} context={context:#x}"
    );
    let result = Frame::read(&emu.maps, context, pointers).and_then(|mut frame| {
        let handler = request.walk(&emu.maps, &mut frame)?;
        frame.store(establisher, frame.establisher);
        if let Some(data) = frame.handler_data {
            frame.store(handler_data, data);
        }
        frame.commit(&mut emu.maps)?;
        Ok(handler)
    });
    match result {
        Ok(handler) => call.set_return(emu, handler),
        Err(reason) => invalid_unwind(emu, library, "RtlVirtualUnwind", reason),
    }
}
