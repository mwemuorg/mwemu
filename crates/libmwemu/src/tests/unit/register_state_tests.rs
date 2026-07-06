use crate::regs64::Regs64;

#[test]
fn test_set_and_get_pre_op_regs() {
    let mut emu = crate::emu64();

    // Create a register state
    let mut regs = Regs64::new();
    regs.rax = 0x1111111111111111;
    regs.rbx = 0x2222222222222222;
    regs.rcx = 0x3333333333333333;

    // Set pre-op registers
    emu.set_pre_op_regs(regs.clone());

    // Get back and verify
    let retrieved = emu.pre_op_regs();
    assert_eq!(retrieved.rax, 0x1111111111111111, "Pre-op RAX should match");
    assert_eq!(retrieved.rbx, 0x2222222222222222, "Pre-op RBX should match");
    assert_eq!(retrieved.rcx, 0x3333333333333333, "Pre-op RCX should match");
}

#[test]
fn test_set_and_get_post_op_regs() {
    let mut emu = crate::emu64();

    // Create a register state
    let mut regs = Regs64::new();
    regs.rax = 0xAAAAAAAAAAAAAAAA;
    regs.rbx = 0xBBBBBBBBBBBBBBBB;
    regs.rcx = 0xCCCCCCCCCCCCCCCC;

    // Set post-op registers
    emu.set_post_op_regs(regs.clone());

    // Get back and verify
    let retrieved = emu.post_op_regs();
    assert_eq!(
        retrieved.rax, 0xAAAAAAAAAAAAAAAA,
        "Post-op RAX should match"
    );
    assert_eq!(
        retrieved.rbx, 0xBBBBBBBBBBBBBBBB,
        "Post-op RBX should match"
    );
    assert_eq!(
        retrieved.rcx, 0xCCCCCCCCCCCCCCCC,
        "Post-op RCX should match"
    );
}

#[test]
fn test_pre_op_regs_mut() {
    let mut emu = crate::emu64();

    // Modify pre-op registers directly
    emu.pre_op_regs_mut().rax = 0x1234;
    emu.pre_op_regs_mut().rbx = 0x5678;

    // Verify changes
    assert_eq!(
        emu.pre_op_regs().rax,
        0x1234,
        "Pre-op RAX should be modified"
    );
    assert_eq!(
        emu.pre_op_regs().rbx,
        0x5678,
        "Pre-op RBX should be modified"
    );
}

#[test]
fn test_post_op_regs_mut() {
    let mut emu = crate::emu64();

    // Modify post-op registers directly
    emu.post_op_regs_mut().rax = 0xABCD;
    emu.post_op_regs_mut().rbx = 0xEF01;

    // Verify changes
    assert_eq!(
        emu.post_op_regs().rax,
        0xABCD,
        "Post-op RAX should be modified"
    );
    assert_eq!(
        emu.post_op_regs().rbx,
        0xEF01,
        "Post-op RBX should be modified"
    );
}

#[test]
fn test_pre_post_op_isolation() {
    let mut emu = crate::emu64();

    // Set different values in pre-op and post-op
    emu.pre_op_regs_mut().rax = 0x1111;
    emu.post_op_regs_mut().rax = 0x2222;

    // Verify they are isolated
    assert_eq!(emu.pre_op_regs().rax, 0x1111, "Pre-op RAX should be 0x1111");
    assert_eq!(
        emu.post_op_regs().rax,
        0x2222,
        "Post-op RAX should be 0x2222"
    );
}

#[test]
fn test_regs_vs_pre_op_isolation() {
    let mut emu = crate::emu64();

    // Set different values in current regs vs pre-op
    emu.regs_mut().rax = 0xAAAA;
    emu.pre_op_regs_mut().rax = 0xBBBB;

    // Verify they are isolated
    assert_eq!(emu.regs().rax, 0xAAAA, "Current RAX should be 0xAAAA");
    assert_eq!(emu.pre_op_regs().rax, 0xBBBB, "Pre-op RAX should be 0xBBBB");
}

#[test]
fn test_regs_vs_post_op_isolation() {
    let mut emu = crate::emu64();

    // Set different values in current regs vs post-op
    emu.regs_mut().rax = 0xCCCC;
    emu.post_op_regs_mut().rax = 0xDDDD;

    // Verify they are isolated
    assert_eq!(emu.regs().rax, 0xCCCC, "Current RAX should be 0xCCCC");
    assert_eq!(
        emu.post_op_regs().rax,
        0xDDDD,
        "Post-op RAX should be 0xDDDD"
    );
}

#[test]
fn test_all_register_fields_pre_op() {
    let mut emu = crate::emu64();

    // Set all general-purpose registers in pre-op
    emu.pre_op_regs_mut().rax = 0x1;
    emu.pre_op_regs_mut().rbx = 0x2;
    emu.pre_op_regs_mut().rcx = 0x3;
    emu.pre_op_regs_mut().rdx = 0x4;
    emu.pre_op_regs_mut().rsi = 0x5;
    emu.pre_op_regs_mut().rdi = 0x6;
    emu.pre_op_regs_mut().rbp = 0x7;
    emu.pre_op_regs_mut().rsp = 0x8;

    // Verify all
    let pre = emu.pre_op_regs();
    assert_eq!(pre.rax, 0x1);
    assert_eq!(pre.rbx, 0x2);
    assert_eq!(pre.rcx, 0x3);
    assert_eq!(pre.rdx, 0x4);
    assert_eq!(pre.rsi, 0x5);
    assert_eq!(pre.rdi, 0x6);
    assert_eq!(pre.rbp, 0x7);
    assert_eq!(pre.rsp, 0x8);
}

#[test]
fn test_all_register_fields_post_op() {
    let mut emu = crate::emu64();

    // Set all general-purpose registers in post-op
    emu.post_op_regs_mut().rax = 0xA;
    emu.post_op_regs_mut().rbx = 0xB;
    emu.post_op_regs_mut().rcx = 0xC;
    emu.post_op_regs_mut().rdx = 0xD;
    emu.post_op_regs_mut().rsi = 0xE;
    emu.post_op_regs_mut().rdi = 0xF;
    emu.post_op_regs_mut().rbp = 0x10;
    emu.post_op_regs_mut().rsp = 0x11;

    // Verify all
    let post = emu.post_op_regs();
    assert_eq!(post.rax, 0xA);
    assert_eq!(post.rbx, 0xB);
    assert_eq!(post.rcx, 0xC);
    assert_eq!(post.rdx, 0xD);
    assert_eq!(post.rsi, 0xE);
    assert_eq!(post.rdi, 0xF);
    assert_eq!(post.rbp, 0x10);
    assert_eq!(post.rsp, 0x11);
}

#[test]
fn test_register_state_32bit() {
    let mut emu = crate::emu32();

    // Set 32-bit register values
    emu.regs_mut().set_eax(0x12345678);
    emu.pre_op_regs_mut().set_eax(0xAAAAAAAA);
    emu.post_op_regs_mut().set_eax(0xBBBBBBBB);

    // Verify all three are different
    assert_eq!(emu.regs().get_eax(), 0x12345678);
    assert_eq!(emu.pre_op_regs().get_eax(), 0xAAAAAAAA);
    assert_eq!(emu.post_op_regs().get_eax(), 0xBBBBBBBB);
}
