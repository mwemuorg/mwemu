use crate::{fpu::FPU, tests::helpers};

#[test]
// test FPU stack operations beyond basic F80 tests
pub fn fpu_stack_operations() {
    helpers::setup();

    let mut fpu = FPU::new();

    // Test initial stack state
    assert_eq!(fpu.get_top(), 0);
    assert_eq!(fpu.get_depth(), 0);

    // Test stack push operations
    fpu.push_f64(1.0);
    assert_eq!(fpu.get_depth(), 1);
    assert_eq!(fpu.peek_st_logical_f64(0), 1.0);

    fpu.push_f64(2.0);
    assert_eq!(fpu.get_depth(), 2);
    assert_eq!(fpu.peek_st_logical_f64(0), 2.0);
    assert_eq!(fpu.peek_st_logical_f64(1), 1.0);

    fpu.push_f64(3.0);
    assert_eq!(fpu.get_depth(), 3);
    assert_eq!(fpu.peek_st_logical_f64(0), 3.0);
    assert_eq!(fpu.peek_st_logical_f64(1), 2.0);
    assert_eq!(fpu.peek_st_logical_f64(2), 1.0);

    // Test stack pop operations
    let val = fpu.pop_f64();
    assert_eq!(val, 3.0);
    assert_eq!(fpu.get_depth(), 2);
    assert_eq!(fpu.peek_st_logical_f64(0), 2.0);

    // Test stack overflow protection (push 5 more values to reach 8 total)
    for i in 3..9 {
        fpu.push_f64(i as f64);
    }

    // Stack should be full now, test behavior
    assert_eq!(fpu.get_depth(), 8);

    // Test clearing stack
    fpu.clear();
    assert_eq!(fpu.get_depth(), 0);
    assert_eq!(fpu.get_top(), 0);

    // Test mixed operations
    fpu.push_f64(10.5);
    fpu.push_f64(20.25);
    fpu.push_f64(30.125);
    fpu.st.print();
    assert_eq!(fpu.peek_st_logical_f64(0), 30.125);
    assert_eq!(fpu.peek_st_logical_f64(1), 20.25);
    assert_eq!(fpu.peek_st_logical_f64(2), 10.5);
}
