#![warn(rust_2018_idioms)]

use super::*;

use crate::Intel8080;

// CPI (Compare immediate with A)
#[test]
fn cpi() {
    let mut i8080 = Intel8080::default();

    // Intel 8080 Assembly Language Programming, p. 29.
    i8080.cpu.a = 0x4A;
    i8080.cpu.execute_instruction([0xFE, 0x40, 0], &mut i8080.memory); // CPI 40H
    assert_eq!(i8080.cpu.a, 0x4A);
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
}
