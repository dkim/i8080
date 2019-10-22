#![warn(rust_2018_idioms)]

use super::*;

use crate::Intel8080;

// CMP r (Compare register with A)
#[test]
fn cmp_r() {
    let mut i8080 = Intel8080::default();

    // Intel 8080 Assembly Language Programming, p. 20.
    i8080.cpu.a = 0x0A;
    i8080.cpu.e = 0x05;
    i8080.cpu.execute_instruction([0xBB, 0, 0], &mut i8080.memory); // CMP E
    assert_eq!(i8080.cpu.a, 0x0A);
    assert_eq!(i8080.cpu.e, 0x05);
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));

    // Intel 8080 Assembly Language Programming, p. 21.
    i8080.cpu.a = 0x02;
    i8080.cpu.e = 0x05;
    i8080.cpu.execute_instruction([0xBB, 0, 0], &mut i8080.memory); // CMP E
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));

    // Intel 8080 Assembly Language Programming, p. 21.
    i8080.cpu.a = !0x1B + 1;
    i8080.cpu.e = 0x05;
    i8080.cpu.execute_instruction([0xBB, 0, 0], &mut i8080.memory); // CMP E
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
}

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

// DAA (Decimal adjust A)
#[test]
fn daa() {
    let mut i8080 = Intel8080::default();

    // Intel 8080 Assembly Language Programming, p. 16.
    i8080.cpu.a = 0x9B;
    i8080.cpu.condition_flags.remove(ConditionFlags::CARRY);
    i8080.cpu.condition_flags.remove(ConditionFlags::AUX_CARRY);
    i8080.cpu.execute_instruction([0x27, 0, 0], &mut i8080.memory); // DAA
    assert_eq!(i8080.cpu.a, 0x01);
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));

    // Intel 8080 Assembly Language Programming, p. 56.
    i8080.cpu.a = 0xBB;
    i8080.cpu.condition_flags.remove(ConditionFlags::CARRY);
    i8080.cpu.condition_flags.remove(ConditionFlags::AUX_CARRY);
    i8080.cpu.execute_instruction([0x27, 0, 0], &mut i8080.memory); // DAA
    assert_eq!(i8080.cpu.a, 0x21);
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));

    // Intel 8080 Assembly Language Programming, p. 56.
    i8080.cpu.a = 0x73;
    i8080.cpu.condition_flags.remove(ConditionFlags::CARRY);
    i8080.cpu.condition_flags.insert(ConditionFlags::AUX_CARRY);
    i8080.cpu.execute_instruction([0x27, 0, 0], &mut i8080.memory); // DAA
    assert_eq!(i8080.cpu.a, 0x79);
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
}

// SBB r (Subtract register from A with borrow)
#[test]
fn sbb_r() {
    let mut i8080 = Intel8080::default();

    // Intel 8080 Assembly Language Programming, p. 19.
    i8080.cpu.l = 0x02;
    i8080.cpu.a = 0x04;
    i8080.cpu.condition_flags.insert(ConditionFlags::CARRY);
    i8080.cpu.execute_instruction([0x9D, 0, 0], &mut i8080.memory); // SBB L
    assert_eq!(i8080.cpu.a, 0x01);
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::PARITY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::SIGN));

    // Intel 8080/8085 Assembly Language Programming Manual, p. 3-57.
    i8080.cpu.b = 0x02;
    i8080.cpu.a = 0x04;
    i8080.cpu.condition_flags.insert(ConditionFlags::CARRY);
    i8080.cpu.execute_instruction([0x98, 0, 0], &mut i8080.memory); // SBB B
    assert_eq!(i8080.cpu.a, 0x01);
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::PARITY));
    assert!(i8080.cpu.condition_flags.contains(ConditionFlags::AUX_CARRY));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::ZERO));
    assert!(!i8080.cpu.condition_flags.contains(ConditionFlags::SIGN));
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
