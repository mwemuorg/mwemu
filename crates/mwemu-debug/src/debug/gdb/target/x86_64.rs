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
use gdbstub_arch::x86::reg::X86_64CoreRegs;
use gdbstub_arch::x86::reg::id::{X86_64CoreRegId, X86SegmentRegId};

use crate::debug::gdb::registers::{read_regs_64, write_regs_64};
use crate::emu::Emu;

use crate::debug::gdb::target_xml;

use super::MwemuGdbError;
use super::shared::{copy_range, generate_library_list_xml};

/// GDB target wrapper for 64-bit emulation
pub struct MwemuTarget64<'a> {
    pub emu: &'a mut Emu,
    pub single_step: bool,
}

impl<'a> MwemuTarget64<'a> {
    pub fn new(emu: &'a mut Emu) -> Self {
        Self {
            emu,
            single_step: false,
        }
    }
}

impl Target for MwemuTarget64<'_> {
    type Arch = gdbstub_arch::x86::X86_64_SSE;
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

impl SingleThreadBase for MwemuTarget64<'_> {
    fn read_registers(&mut self, regs: &mut X86_64CoreRegs) -> TargetResult<(), Self> {
        *regs = read_regs_64(self.emu);
        Ok(())
    }

    fn write_registers(&mut self, regs: &X86_64CoreRegs) -> TargetResult<(), Self> {
        write_regs_64(self.emu, regs);
        Ok(())
    }

    fn read_addrs(&mut self, start_addr: u64, data: &mut [u8]) -> TargetResult<usize, Self> {
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

    fn write_addrs(&mut self, start_addr: u64, data: &[u8]) -> TargetResult<(), Self> {
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

impl SingleThreadResume for MwemuTarget64<'_> {
    fn resume(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        self.single_step = false;
        Ok(())
    }

    #[inline(always)]
    fn support_single_step(&mut self) -> Option<SingleThreadSingleStepOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadSingleStep for MwemuTarget64<'_> {
    fn step(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        self.single_step = true;
        self.emu.step();
        Ok(())
    }
}

impl SingleRegisterAccess<()> for MwemuTarget64<'_> {
    fn read_register(
        &mut self,
        _tid: (),
        reg_id: X86_64CoreRegId,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let flags = self.emu.flags_snapshot();
        let regs = self.emu.regs();

        let (val, size): (u64, usize) = match reg_id {
            X86_64CoreRegId::Gpr(n) => {
                let val = match n {
                    0 => regs.rax,
                    1 => regs.rbx,
                    2 => regs.rcx,
                    3 => regs.rdx,
                    4 => regs.rsi,
                    5 => regs.rdi,
                    6 => regs.rbp,
                    7 => regs.rsp,
                    8 => regs.r8,
                    9 => regs.r9,
                    10 => regs.r10,
                    11 => regs.r11,
                    12 => regs.r12,
                    13 => regs.r13,
                    14 => regs.r14,
                    15 => regs.r15,
                    _ => return Err(TargetError::NonFatal),
                };
                (val, 8)
            }
            X86_64CoreRegId::Rip => (regs.rip, 8),
            X86_64CoreRegId::Eflags => (flags.dump() as u64, 4),
            X86_64CoreRegId::Segment(seg) => {
                let val = match seg {
                    X86SegmentRegId::CS => 0x33,
                    X86SegmentRegId::SS => 0x2b,
                    X86SegmentRegId::DS => 0x2b,
                    X86SegmentRegId::ES => 0x2b,
                    X86SegmentRegId::FS => 0x53,
                    X86SegmentRegId::GS => 0x2b,
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
        reg_id: X86_64CoreRegId,
        val: &[u8],
    ) -> TargetResult<(), Self> {
        let regs = self.emu.regs_mut();

        match reg_id {
            X86_64CoreRegId::Gpr(n) => {
                let value =
                    u64::from_le_bytes(val[..8].try_into().map_err(|_| TargetError::NonFatal)?);
                match n {
                    0 => regs.rax = value,
                    1 => regs.rbx = value,
                    2 => regs.rcx = value,
                    3 => regs.rdx = value,
                    4 => regs.rsi = value,
                    5 => regs.rdi = value,
                    6 => regs.rbp = value,
                    7 => regs.rsp = value,
                    8 => regs.r8 = value,
                    9 => regs.r9 = value,
                    10 => regs.r10 = value,
                    11 => regs.r11 = value,
                    12 => regs.r12 = value,
                    13 => regs.r13 = value,
                    14 => regs.r14 = value,
                    15 => regs.r15 = value,
                    _ => return Err(TargetError::NonFatal),
                }
            }
            X86_64CoreRegId::Rip => {
                let value =
                    u64::from_le_bytes(val[..8].try_into().map_err(|_| TargetError::NonFatal)?);
                regs.rip = value;
            }
            X86_64CoreRegId::Eflags => {
                let value =
                    u32::from_le_bytes(val[..4].try_into().map_err(|_| TargetError::NonFatal)?);
                self.emu.flags_mut().load(value);
            }
            _ => return Err(TargetError::NonFatal),
        }

        Ok(())
    }
}

impl TargetDescriptionXmlOverride for MwemuTarget64<'_> {
    fn target_description_xml(
        &self,
        annex: &[u8],
        offset: u64,
        length: usize,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let annex_str = std::str::from_utf8(annex).map_err(|_| TargetError::NonFatal)?;
        let xml = target_xml::x64::get_xml(annex_str).ok_or(TargetError::NonFatal)?;
        Ok(copy_range(xml.as_bytes(), offset, length, buf))
    }
}

impl Libraries for MwemuTarget64<'_> {
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

impl ExecFile for MwemuTarget64<'_> {
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
