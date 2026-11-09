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
use gdbstub::target::{Target, TargetError, TargetResult};
use gdbstub_arch::aarch64::reg::AArch64CoreRegs;
use gdbstub_arch::aarch64::reg::id::AArch64RegId;

use crate::debug::gdb::registers::{read_regs_aarch64, write_regs_aarch64};
use crate::emu::Emu;

use super::MwemuGdbError;
use super::shared::{copy_range, generate_library_list_xml};

/// GDB target wrapper for AArch64 emulation
pub struct MwemuTargetAarch64<'a> {
    pub emu: &'a mut Emu,
    pub single_step: bool,
}

impl<'a> MwemuTargetAarch64<'a> {
    pub fn new(emu: &'a mut Emu) -> Self {
        Self {
            emu,
            single_step: false,
        }
    }
}

impl Target for MwemuTargetAarch64<'_> {
    type Arch = gdbstub_arch::aarch64::AArch64;
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
    fn support_libraries(&mut self) -> Option<LibrariesOps<'_, Self>> {
        Some(self)
    }

    #[inline(always)]
    fn support_exec_file(&mut self) -> Option<ExecFileOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadBase for MwemuTargetAarch64<'_> {
    fn read_registers(&mut self, regs: &mut AArch64CoreRegs) -> TargetResult<(), Self> {
        *regs = read_regs_aarch64(self.emu);
        Ok(())
    }

    fn write_registers(&mut self, regs: &AArch64CoreRegs) -> TargetResult<(), Self> {
        write_regs_aarch64(self.emu, regs);
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

impl SingleThreadResume for MwemuTargetAarch64<'_> {
    fn resume(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        self.single_step = false;
        Ok(())
    }

    #[inline(always)]
    fn support_single_step(&mut self) -> Option<SingleThreadSingleStepOps<'_, Self>> {
        Some(self)
    }
}

impl SingleThreadSingleStep for MwemuTargetAarch64<'_> {
    fn step(&mut self, _signal: Option<gdbstub::common::Signal>) -> Result<(), Self::Error> {
        self.single_step = true;
        self.emu.step();
        Ok(())
    }
}

impl SingleRegisterAccess<()> for MwemuTargetAarch64<'_> {
    fn read_register(
        &mut self,
        _tid: (),
        reg_id: AArch64RegId,
        buf: &mut [u8],
    ) -> TargetResult<usize, Self> {
        let regs = self.emu.regs_aarch64();

        match reg_id {
            AArch64RegId::X(n) => {
                if (n as usize) >= 31 {
                    return Err(TargetError::NonFatal);
                }
                let bytes = regs.x[n as usize].to_le_bytes();
                buf[..8].copy_from_slice(&bytes);
                Ok(8)
            }
            AArch64RegId::Sp => {
                buf[..8].copy_from_slice(&regs.sp.to_le_bytes());
                Ok(8)
            }
            AArch64RegId::Pc => {
                buf[..8].copy_from_slice(&regs.pc.to_le_bytes());
                Ok(8)
            }
            AArch64RegId::Pstate => {
                let val = regs.nzcv.as_u64() as u32;
                buf[..4].copy_from_slice(&val.to_le_bytes());
                Ok(4)
            }
            AArch64RegId::V(n) => {
                if (n as usize) >= 32 {
                    return Err(TargetError::NonFatal);
                }
                let bytes = regs.v[n as usize].to_le_bytes();
                buf[..16].copy_from_slice(&bytes);
                Ok(16)
            }
            _ => Err(TargetError::NonFatal),
        }
    }

    fn write_register(
        &mut self,
        _tid: (),
        reg_id: AArch64RegId,
        val: &[u8],
    ) -> TargetResult<(), Self> {
        let regs = self.emu.regs_aarch64_mut();

        match reg_id {
            AArch64RegId::X(n) => {
                if (n as usize) >= 31 {
                    return Err(TargetError::NonFatal);
                }
                let value =
                    u64::from_le_bytes(val[..8].try_into().map_err(|_| TargetError::NonFatal)?);
                regs.x[n as usize] = value;
            }
            AArch64RegId::Sp => {
                regs.sp =
                    u64::from_le_bytes(val[..8].try_into().map_err(|_| TargetError::NonFatal)?);
            }
            AArch64RegId::Pc => {
                regs.pc =
                    u64::from_le_bytes(val[..8].try_into().map_err(|_| TargetError::NonFatal)?);
            }
            AArch64RegId::Pstate => {
                let value =
                    u32::from_le_bytes(val[..4].try_into().map_err(|_| TargetError::NonFatal)?);
                regs.nzcv.from_u64(value as u64);
            }
            AArch64RegId::V(n) => {
                if (n as usize) >= 32 {
                    return Err(TargetError::NonFatal);
                }
                regs.v[n as usize] =
                    u128::from_le_bytes(val[..16].try_into().map_err(|_| TargetError::NonFatal)?);
            }
            _ => return Err(TargetError::NonFatal),
        }
        Ok(())
    }
}

impl Libraries for MwemuTargetAarch64<'_> {
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

impl ExecFile for MwemuTargetAarch64<'_> {
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
