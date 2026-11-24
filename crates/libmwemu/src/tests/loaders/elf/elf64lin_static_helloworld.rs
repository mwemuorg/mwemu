use crate::emu64;
use crate::err::MwemuError;
use crate::tests::helpers;

const TLS_LOAD: u64 = 0x44ab87;
const THREAD_LIST_HEAD: u64 = 0x4cc2d0;
const INSTRUCTION_LIMIT: u64 = 100_000;

#[test]
fn elf64lin_static_helloworld() -> Result<(), MwemuError> {
    helpers::setup();
    let mut emu = emu64();
    emu.cfg.max_instructions = Some(INSTRUCTION_LIMIT);
    emu.load_code(&sample!("elf64lin_static_helloworld.bin"));
    assert_eq!(emu.run(Some(TLS_LOAD))?, TLS_LOAD);
    assert_eq!(emu.regs().rcx, THREAD_LIST_HEAD);

    emu.run(None)?;
    assert_eq!(emu.emulated_stdout, b"hello world\n");
    assert_eq!(emu.processes.exit_status, Some(0));
    Ok(())
}
