use crate::utils::helpers::{color_enabled, disable_color, enable_color, likely, unlikely};

#[test]
fn test_color_enabled_by_default() {
    // Color should be enabled by default
    assert!(color_enabled(), "Color should be enabled by default");
}

#[test]
fn test_disable_color() {
    enable_color(); // Ensure it's enabled first
    disable_color();

    assert!(
        !color_enabled(),
        "Color should be disabled after disable_color()"
    );

    // Reset for other tests
    enable_color();
}

#[test]
fn test_enable_color() {
    disable_color(); // Ensure it's disabled first
    enable_color();

    assert!(
        color_enabled(),
        "Color should be enabled after enable_color()"
    );
}

#[test]
fn test_color_toggle() {
    // Test multiple toggles
    enable_color();
    assert!(color_enabled());

    disable_color();
    assert!(!color_enabled());

    enable_color();
    assert!(color_enabled());

    disable_color();
    assert!(!color_enabled());

    // Reset for other tests
    enable_color();
}

#[test]
fn test_likely_true() {
    let result = likely(true);
    assert!(result, "likely(true) should return true");
}

#[test]
fn test_likely_false() {
    let result = likely(false);
    assert!(!result, "likely(false) should return false");
}

#[test]
fn test_unlikely_true() {
    let result = unlikely(true);
    assert!(result, "unlikely(true) should return true");
}

#[test]
fn test_unlikely_false() {
    let result = unlikely(false);
    assert!(!result, "unlikely(false) should return false");
}

#[test]
fn test_likely_preserves_value() {
    // likely() should preserve the boolean value even though it's a hint
    for val in [true, false] {
        assert_eq!(likely(val), val, "likely() should preserve boolean value");
    }
}

#[test]
fn test_unlikely_preserves_value() {
    // unlikely() should preserve the boolean value even though it's a hint
    for val in [true, false] {
        assert_eq!(
            unlikely(val),
            val,
            "unlikely() should preserve boolean value"
        );
    }
}

#[test]
fn test_color_state_persistence() {
    // Test that color state persists
    enable_color();
    assert!(color_enabled());

    disable_color();
    assert!(!color_enabled());

    // Reset for other tests
    enable_color();
}

#[test]
fn test_color_idempotent_enable() {
    enable_color();
    enable_color();
    enable_color();

    assert!(color_enabled(), "Multiple enable calls should work");
}

#[test]
fn test_color_idempotent_disable() {
    disable_color();
    disable_color();
    disable_color();

    assert!(!color_enabled(), "Multiple disable calls should work");

    // Reset for other tests
    enable_color();
}
