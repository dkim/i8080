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

// SBI (Subtract immediate from A with borrow)
#[test]
fn sbi() {
    let mut i8080 = Intel8080::default();

    // Intel 8080 Assembly Language Programming, p. 28.
    i8080.cpu.a = 0x00;
    i8080.cpu.condition_flags.remove(ConditionFlags::CARRY);
    i8080.cpu.execute_instruction([0xDE, 0x01, 0], &mut i8080.memory); // SBI 1
    assert_eq!(i8080.cpu.a, 0xFF);
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::PARITY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::SIGN));

    // Intel 8080 Assembly Language Programming, p. 28.
    i8080.cpu.a = 0x00;
    i8080.cpu.condition_flags.insert(ConditionFlags::CARRY);
    i8080.cpu.execute_instruction([0xDE, 0x01, 0], &mut i8080.memory); // SBI 1
    assert_eq!(i8080.cpu.a, 0xFE);
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::PARITY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::SIGN));
}

// SUB r (Subtract register from A)
#[test]
fn sub_r() {
    let mut i8080 = Intel8080::default();

    // Intel 8080 Assembly Language Programming, p. 18.
    i8080.cpu.a = 0x3E;
    i8080.cpu.execute_instruction([0x97, 0, 0], &mut i8080.memory); // SUB A
    assert_eq!(i8080.cpu.a, 0);
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::PARITY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::SIGN));
}

// SUI (Subtract immediate from A)
#[test]
fn sui() {
    let mut i8080 = Intel8080::default();

    // Intel 8080 Assembly Language Programming, p. 28.
    i8080.cpu.a = 0x00;
    i8080.cpu.execute_instruction([0xD6, 0x01, 0], &mut i8080.memory); // SUI 1
    assert_eq!(i8080.cpu.a, 0xFF);
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::PARITY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::SIGN));

    // Intel 8080/8085 Assembly Language Programming Manual, p. 3-65.
    i8080.cpu.a = 0x09;
    i8080.cpu.execute_instruction([0xD6, 0x01, 0], &mut i8080.memory); // SUI 1
    assert_eq!(i8080.cpu.a, 0x08);
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::PARITY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::SIGN));
}
