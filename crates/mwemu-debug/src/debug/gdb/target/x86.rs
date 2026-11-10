use std::convert::TryInto;

use gdbstub::target::ext::base::single_register_access::{
    SingleRegisterAccess, SingleRegisterAccessOps,
};
use gdbstub::target::ext::base::singlethread::{
    SingleThreadBase, SingleThreadResume, SingleThreadResumeOps, SingleThreadSingleStep,
    SingleThreadSingleStepOps,
};
use gdbstub::target::ext::exec_file::{ExecFile, ExecFileOps};
use gdbstub::target::ext::libraries::{Libraries, LibrariesOps};
use gdbstub::target::ext::target_description_xml_override::{
    TargetDescriptionXmlOverride, TargetDescriptionXmlOverrideOps,
};
use gdbstub::target::{Target, TargetError, TargetResult};
use gdbstub_arch::x86::reg::X86CoreRegs;
use gdbstub_arch::x86::reg::id::{X86CoreRegId, X86SegmentRegId};

use crate::debug::gdb::registers::{read_regs_32, write_regs_32};
use crate::emu::Emu;

use crate::debug::gdb::target_xml;

use super::MwemuGdbError;
use super::shared::{copy_range, generate_library_list_xml};

/// GDB target wrapper for 32-bit emulation
pub struct MwemuTarget32<'a> {
    pub emu: &'a mut Emu,
    pub single_step: bool,
}

impl<'a> MwemuTarget32<'a> {
    pub fn new(emu: &'a mut Emu) -> Self {
        Self {
            emu,
            single_step: false,
        }
    }
}

impl Target for MwemuTarget32<'_> {
    type Arch = gdbstub_arch::x86::X86_SSE;
    type Error = MwemuGdbError;

    #[inline(always)]
    fn base_ops(&mut self) -> gdbstub::target::ext::base::BaseOps<'_, Self::Arch, Self::Error> {
        gdbstub::target::ext::base::BaseOps::SingleThread(self)
    }

    #[inline(always)]
    fn support_breakpoints(
        &mut self,
    ) -> Option<gdbstub::target::ext::breakpoints::BreakpointsOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_target_description_xml_override(
        &mut self,
    ) -> Option<TargetDescriptionXmlOverrideOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_libraries(&mut self) -> Option<LibrariesOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_exec_file(&mut self) -> Option<ExecFileOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadBase for MwemuTarget32<'_> {
    fn read_registers(&mut self, regs: &mut X86CoreRegs) -> TargetResult<(), Self> {
        *regs = read_regs_32(self.emu);
        Ok(())
    }

    fn write_registers(&mut self, regs: &X86CoreRegs) -> TargetResult<(), Self> {
        write_regs_32(self.emu, regs);
        Ok(())
    }

    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<usize, Self> {
        let start_addr = start_addr as u64;
        let mut bytes_read = 0;
        for (i, byte) in data.iter_mut().enumerate() {
            match self.emu.maps.read_byte(start_addr + i as u64) {
                Some(b) => {
                    *byte = b;
                    bytes_read += 1;
                }
                None => break,
            }
        }
        Ok(bytes_read)
    }

    fn write_addrs(&mut self, start_addr: u32, data: &[u8]) -> TargetResult<(), Self> {
        let start_addr = start_addr as u64;
        for (i, &byte) in data.iter().enumerate() {
            self.emu.maps.write_byte(start_addr + i as u64, byte);
        }
        Ok(())
    }

    #[inline(always)]
    fn support_resume(&mut self) -> Option<SingleThreadResumeOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_single_register_access(&mut self) -> Option<SingleRegisterAccessOps<'_, (), Self>> {
        Some(self)
    }
}

impl SingleThreadResume for MwemuTarget32<'_> {
    fn resume(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        self.single_step = false;
        Ok(())
    }

    #[inline(always)]
    fn support_single_step(&mut self) -> Option<SingleThreadSingleStepOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadSingleStep for MwemuTarget32<'_> {
    fn step(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        self.single_step = true;
        self.emu.step();
        Ok(())
    }
}

impl SingleRegisterAccess<()> for MwemuTarget32<'_> {
    fn read_register(
        &mut self,
        _tid: (),
        reg_id: X86CoreRegId,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let flags = self.emu.flags_snapshot();
        let regs = self.emu.regs();

        let (val, size): (u32, usize) = match reg_id {
            X86CoreRegId::Eax => (regs.rax as u32, 4),
            X86CoreRegId::Ecx => (regs.rcx as u32, 4),
            X86CoreRegId::Edx => (regs.rdx as u32, 4),
            X86CoreRegId::Ebx => (regs.rbx as u32, 4),
            X86CoreRegId::Esp => (regs.rsp as u32, 4),
            X86CoreRegId::Ebp => (regs.rbp as u32, 4),
            X86CoreRegId::Esi => (regs.rsi as u32, 4),
            X86CoreRegId::Edi => (regs.rdi as u32, 4),
            X86CoreRegId::Eip => (regs.rip as u32, 4),
            X86CoreRegId::Eflags => (flags.dump(), 4),
            X86CoreRegId::Segment(seg) => {
                let val = match seg {
                    X86SegmentRegId::CS => 0x1b,
                    X86SegmentRegId::SS => 0x23,
                    X86SegmentRegId::DS => 0x23,
                    X86SegmentRegId::ES => 0x23,
                    X86SegmentRegId::FS => 0x3b,
                    X86SegmentRegId::GS => 0x00,
                };
                (val, 4)
            }
            _ => return Err(TargetError::NonFatal),
        };

        let bytes = val.to_le_bytes();
        buf[..size].copy_from_slice(&bytes[..size]);
        Ok(size)
    }

    fn write_register(
        &mut self,
        _tid: (),
        reg_id: X86CoreRegId,
        val: &[u8],
    ) -> TargetResult<(), Self> {
        let value = u32::from_le_bytes(val[..4].try_into().map_err(|_| TargetError::NonFatal)?);
        let regs = self.emu.regs_mut();

        match reg_id {
            X86CoreRegId::Eax => regs.rax = value as u64,
            X86CoreRegId::Ecx => regs.rcx = value as u64,
            X86CoreRegId::Edx => regs.rdx = value as u64,
            X86CoreRegId::Ebx => regs.rbx = value as u64,
            X86CoreRegId::Esp => regs.rsp = value as u64,
            X86CoreRegId::Ebp => regs.rbp = value as u64,
            X86CoreRegId::Esi => regs.rsi = value as u64,
            X86CoreRegId::Edi => regs.rdi = value as u64,
            X86CoreRegId::Eip => regs.rip = value as u64,
            X86CoreRegId::Eflags => self.emu.flags_mut().load(value),
            _ => return Err(TargetError::NonFatal),
        }

        Ok(())
    }
}

impl TargetDescriptionXmlOverride for MwemuTarget32<'_> {
    fn target_description_xml(
        &self,
        annex: &[u8],
        offset: u64,
        length: usize,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let annex_str = std::str::from_utf8(annex).map_err(|_| TargetError::NonFatal)?;
        let xml = target_xml::x86::get_xml(annex_str).ok_or(TargetError::NonFatal)?;
        Ok(copy_range(xml.as_bytes(), offset, length, buf))
    }
}

impl Libraries for MwemuTarget32<'_> {
    fn get_libraries(
        &self,
        offset: u64,
        length: usize,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let xml = generate_library_list_xml(&self.emu.maps);
        Ok(copy_range(xml.as_bytes(), offset, length, buf))
    }
}

impl ExecFile for MwemuTarget32<'_> {
    fn get_exec_file(
        &self,
        _pid: Option<gdbstub::common::Pid>,
        offset: u64,
        length: usize,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        Ok(copy_range(
            self.emu.filename.as_bytes(),
            offset,
            length,
            buf,
        ))
    }
}
