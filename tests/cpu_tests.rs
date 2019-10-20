#![warn(rust_2018_idioms)]

use std::{path::Path, u32};

use i8080::Intel8080;

#[test]
fn cpu_tests_8080pre() {
    cpu_tests("tests/cpu_tests/8080PRE.COM", |output| {
        println!("{}", String::from_utf8_lossy(output));
        assert_eq!(output, b"8080 Preliminary tests complete");
    });
}

#[test]
fn cpu_tests_tst8080() {
    cpu_tests("tests/cpu_tests/TST8080.COM", |output| {
        println!("{}", String::from_utf8_lossy(&output));
        assert!(output.ends_with(b" CPU IS OPERATIONAL"));
    });
}

fn cpu_tests<P: AsRef<Path>, F: FnOnce(&[u8])>(program: P, check: F) {
    let mut i8080 = Intel8080::new(&[program], 0x100).unwrap();
    // Location 0x0005 (CP/M BOOT + 0x0005) is the principal entry to the CP/M FDOS (BIOS + BDOS)
    // functions.
    i8080.memory[0x0005] = 0xC9; // RET (Return)
    let mut output = Vec::new();
    loop {
        match i8080.cpu.pc {
            // The machine code found at location 0x0000 (CP/M BOOT) performs a system warm start,
            // which returns control to the Console Command Processor (CCP).
            0x0000 => {
                check(&output);
                break;
            }
            // Location 0x0005 (CP/M BOOT + 0x0005) is the principal entry to the CP/M FDOS (BIOS +
            // BDOS) functions. The function number is passed in register C.
            0x0005 => match i8080.cpu.c {
                // FDOS function 2 - console output (E = ASCII character).
                0x02 => output.push(i8080.cpu.e),
                // FDOS function 9 - print string (DE = string address).
                0x09 => {
                    let address = u16::from_le_bytes([i8080.cpu.e, i8080.cpu.d]);
                    let string = i8080.memory[address..].iter().take_while(|byte| **byte != b'$');
                    output.extend(string);
                }
                _ => unimplemented!("Call 5 (C = {:#04X})", i8080.cpu.c),
            },
            _ => (),
        }
        match i8080.fetch_execute_instruction() {
            // 8080PRE.COM

            // ANI (And immediate with A)
            ([0xE6, _, 0], 7) => (),

            // CALL (Call unconditional)
            ([0xCD, _, _], 17) => (),

            // CNZ (Call on no zero)
            ([0xC4, _, _], 11) | ([0xC4, _, _], 17) => (),
            // CZ (Call on zero)
            ([0xCC, _, _], 11) | ([0xCC, _, _], 17) => (),
            // CNC (Call on no carry)
            ([0xD4, _, _], 11) | ([0xD4, _, _], 17) => (),
            // CC (Call on carry)
            ([0xDC, _, _], 11) | ([0xDC, _, _], 17) => (),
            // CPO (Call on parity odd)
            ([0xE4, _, _], 11) | ([0xE4, _, _], 17) => (),
            // CPE (Call on parity even)
            ([0xEC, _, _], 11) | ([0xEC, _, _], 17) => (),
            // CP (Call on positive)
            ([0xF4, _, _], 11) | ([0xF4, _, _], 17) => (),
            // CM (Call on minus)
            ([0xFC, _, _], 11) | ([0xFC, _, _], 17) => (),

            // CPI (Compare immediate with A)
            ([0xFE, _, 0], 7) => (),

            // DCR B (Decrement B)
            ([0x05, 0, 0], 5) => (),
            // DCR C (Decrement C)
            ([0x0D, 0, 0], 5) => (),
            // DCR D (Decrement B)
            ([0x15, 0, 0], 5) => (),
            // DCR E (Decrement C)
            ([0x1D, 0, 0], 5) => (),
            // DCR H (Decrement B)
            ([0x25, 0, 0], 5) => (),
            // DCR L (Decrement C)
            ([0x2D, 0, 0], 5) => (),
            // DCR A (Decrement A)
            ([0x3D, 0, 0], 5) => (),

            // INR B (Increment B)
            ([0x04, 0, 0], 5) => (),
            // INR C (Increment C)
            ([0x0C, 0, 0], 5) => (),
            // INR D (Increment D)
            ([0x14, 0, 0], 5) => (),
            // INR E (Increment E)
            ([0x1C, 0, 0], 5) => (),
            // INR H (Increment H)
            ([0x24, 0, 0], 5) => (),
            // INR L (Increment L)
            ([0x2C, 0, 0], 5) => (),
            // INR A (Increment A)
            ([0x3C, 0, 0], 5) => (),

            // INX B (Increment BC by one)
            ([0x03, 0, 0], 5) => (),
            // INX D (Increment DE by one)
            ([0x13, 0, 0], 5) => (),
            // INX H (Increment HL by one)
            ([0x23, 0, 0], 5) => (),
            // INX SP (Increment SP by one)
            ([0x33, 0, 0], 5) => (),

            // JMP (Jump unconditional)
            ([0xC3, _, _], 10) => (),

            // JNZ (Jump on no zero)
            ([0xC2, _, _], 10) => (),
            // JZ (Jump on zero)
            ([0xCA, _, _], 10) => (),
            // JNC (Jump on no carry)
            ([0xD2, _, _], 10) => (),
            // JC (Jump on carry)
            ([0xDA, _, _], 10) => (),
            // JPO (Jump on parity odd)
            ([0xE2, _, _], 10) => (),
            // JPE (Jump on parity even)
            ([0xEA, _, _], 10) => (),
            // JP (Jump on positive)
            ([0xF2, _, _], 10) => (),
            // JM (Jump on minus)
            ([0xFA, _, _], 10) => (),

            // LDA (Load A direct)
            ([0x3A, _, _], 13) => (),

            // LXI SP (Load immediate stack pointer)
            ([0x31, _, _], 10) => (),

            // LXI B (Load immediate register pair B & C)
            ([0x01, _, _], 10) => (),
            // LXI D (Load immediate register pair D & E)
            ([0x11, _, _], 10) => (),
            // LXI H (Load immediate register pair H & L)
            ([0x21, _, _], 10) => (),

            // MOV B,M (Move memory to B)
            ([0x46, 0, 0], 7) => (),
            // MOV C,M (Move memory to C)
            ([0x4E, 0, 0], 7) => (),
            // MOV D,M (Move memory to D)
            ([0x56, 0, 0], 7) => (),
            // MOV E,M (Move memory to E)
            ([0x5E, 0, 0], 7) => (),
            // MOV H,M (Move memory to H)
            ([0x66, 0, 0], 7) => (),
            // MOV L,M (Move memory to L)
            ([0x6E, 0, 0], 7) => (),
            // MOV A,M (Move memory to A)
            ([0x7E, 0, 0], 7) => (),

            // MOV B,B (Move B to B)
            ([0x40, 0, 0], 5) => (),
            // MOV B,C (Move C to B)
            ([0x41, 0, 0], 5) => (),
            // MOV B,D (Move D to B)
            ([0x42, 0, 0], 5) => (),
            // MOV B,E (Move E to B)
            ([0x43, 0, 0], 5) => (),
            // MOV B,H (Move H to B)
            ([0x44, 0, 0], 5) => (),
            // MOV B,L (Move L to B)
            ([0x45, 0, 0], 5) => (),
            // MOV B,A (Move A to B)
            ([0x47, 0, 0], 5) => (),
            // MOV C,B (Move B to C)
            ([0x48, 0, 0], 5) => (),
            // MOV C,C (Move C to C)
            ([0x49, 0, 0], 5) => (),
            // MOV C,D (Move D to C)
            ([0x4A, 0, 0], 5) => (),
            // MOV C,E (Move E to C)
            ([0x4B, 0, 0], 5) => (),
            // MOV C,H (Move H to C)
            ([0x4C, 0, 0], 5) => (),
            // MOV C,L (Move L to C)
            ([0x4D, 0, 0], 5) => (),
            // MOV C,A (Move A to C)
            ([0x4F, 0, 0], 5) => (),
            // MOV D,B (Move B to D)
            ([0x50, 0, 0], 5) => (),
            // MOV D,C (Move C to D)
            ([0x51, 0, 0], 5) => (),
            // MOV D,D (Move D to D)
            ([0x52, 0, 0], 5) => (),
            // MOV D,E (Move E to D)
            ([0x53, 0, 0], 5) => (),
            // MOV D,H (Move H to D)
            ([0x54, 0, 0], 5) => (),
            // MOV D,L (Move L to D)
            ([0x55, 0, 0], 5) => (),
            // MOV D,A (Move A to D)
            ([0x57, 0, 0], 5) => (),
            // MOV E,B (Move B to E)
            ([0x58, 0, 0], 5) => (),
            // MOV E,C (Move C to E)
            ([0x59, 0, 0], 5) => (),
            // MOV E,D (Move D to E)
            ([0x5A, 0, 0], 5) => (),
            // MOV E,E (Move E to E)
            ([0x5B, 0, 0], 5) => (),
            // MOV E,H (Move H to E)
            ([0x5C, 0, 0], 5) => (),
            // MOV E,L (Move L to E)
            ([0x5D, 0, 0], 5) => (),
            // MOV E,A (Move A to E)
            ([0x5F, 0, 0], 5) => (),
            // MOV H,B (Move B to H)
            ([0x60, 0, 0], 5) => (),
            // MOV H,C (Move C to H)
            ([0x61, 0, 0], 5) => (),
            // MOV H,D (Move D to H)
            ([0x62, 0, 0], 5) => (),
            // MOV H,E (Move E to H)
            ([0x63, 0, 0], 5) => (),
            // MOV H,H (Move H to H)
            ([0x64, 0, 0], 5) => (),
            // MOV H,L (Move L to H)
            ([0x65, 0, 0], 5) => (),
            // MOV H,A (Move A to H)
            ([0x67, 0, 0], 5) => (),
            // MOV L,B (Move B to L)
            ([0x68, 0, 0], 5) => (),
            // MOV L,C (Move C to L)
            ([0x69, 0, 0], 5) => (),
            // MOV L,D (Move D to L)
            ([0x6A, 0, 0], 5) => (),
            // MOV L,E (Move E to L)
            ([0x6B, 0, 0], 5) => (),
            // MOV L,H (Move H to L)
            ([0x6C, 0, 0], 5) => (),
            // MOV L,L (Move L to L)
            ([0x6D, 0, 0], 5) => (),
            // MOV L,A (Move A to L)
            ([0x6F, 0, 0], 5) => (),
            // MOV A,B (Move B to A)
            ([0x78, 0, 0], 5) => (),
            // MOV A,C (Move C to A)
            ([0x79, 0, 0], 5) => (),
            // MOV A,D (Move D to A)
            ([0x7A, 0, 0], 5) => (),
            // MOV A,E (Move E to A)
            ([0x7B, 0, 0], 5) => (),
            // MOV A,H (Move H to A)
            ([0x7C, 0, 0], 5) => (),
            // MOV A,L (Move L to A)
            ([0x7D, 0, 0], 5) => (),
            // MOV A,A (Move A to A)
            ([0x7F, 0, 0], 5) => (),

            // MVI B (Move immediate to B)
            ([0x06, _, 0], 7) => (),
            // MVI C (Move immediate to C)
            ([0x0E, _, 0], 7) => (),
            // MVI D (Move immediate to D)
            ([0x16, _, 0], 7) => (),
            // MVI E (Move immediate to E)
            ([0x1E, _, 0], 7) => (),
            // MVI H (Move immediate to H)
            ([0x26, _, 0], 7) => (),
            // MVI L (Move immediate to L)
            ([0x2E, _, 0], 7) => (),
            // MVI A (Move immediate to A)
            ([0x3E, _, 0], 7) => (),

            // PCHL (H & L to program counter)
            ([0xE9, 0, 0], 5) => (),

            // POP PSW (Pop A and Flags off stack)
            ([0xF1, 0, 0], 10) => (),

            // POP B (Pop register pair B & C off stack)
            ([0xC1, 0, 0], 10) => (),
            // POP D (Pop register pair D & E off stack)
            ([0xD1, 0, 0], 10) => (),
            // POP H (Pop register pair H & L off stack)
            ([0xE1, 0, 0], 10) => (),

            // PUSH PSW (Push A and Flags on stack)
            ([0xF5, 0, 0], 11) => (),

            // PUSH B (Push register pair B & C on stack)
            ([0xC5, 0, 0], 11) => (),
            // PUSH D (Push register pair D & E on stack)
            ([0xD5, 0, 0], 11) => (),
            // PUSH H (Push register pair H & L on stack)
            ([0xE5, 0, 0], 11) => (),

            // RET (Return)
            ([0xC9, 0, 0], 10) => (),

            // RNZ (Return on no zero)
            ([0xC0, 0, 0], 5) | ([0xC0, 0, 0], 11) => (),
            // RZ (Return on zero)
            ([0xC8, 0, 0], 5) | ([0xC8, 0, 0], 11) => (),
            // RNC (Return on no carry)
            ([0xD0, 0, 0], 5) | ([0xD0, 0, 0], 11) => (),
            // RC (Return on carry)
            ([0xD8, 0, 0], 5) | ([0xD8, 0, 0], 11) => (),
            // RPO (Return on parity odd)
            ([0xE0, 0, 0], 5) | ([0xE0, 0, 0], 11) => (),
            // RPE (Return on parity even)
            ([0xE8, 0, 0], 5) | ([0xE8, 0, 0], 11) => (),
            // RP (Return on positive)
            ([0xF0, 0, 0], 5) | ([0xF0, 0, 0], 11) => (),
            // RM (Return on minus)
            ([0xF8, 0, 0], 5) | ([0xF8, 0, 0], 11) => (),

            // RRC (Rotate A right)
            ([0x0F, 0, 0], 4) => (),

            // TST8080.COM

            // ACI (Add immediate to A with carry)
            ([0xCE, _, 0], 7) => (),

            // ADC M (Add memory to A with carry)
            ([0x8E, 0, 0], 7) => (),

            // ADC B (Add B to A with carry)
            ([0x88, 0, 0], 4) => (),
            // ADC C (Add C to A with carry)
            ([0x89, 0, 0], 4) => (),
            // ADC D (Add D to A with carry)
            ([0x8A, 0, 0], 4) => (),
            // ADC E (Add E to A with carry)
            ([0x8B, 0, 0], 4) => (),
            // ADC H (Add H to A with carry)
            ([0x8C, 0, 0], 4) => (),
            // ADC L (Add L to A with carry)
            ([0x8D, 0, 0], 4) => (),
            // ADC A (Add A to A with carry)
            ([0x8F, 0, 0], 4) => (),

            // ADD M (Add memory to A)
            ([0x86, 0, 0], 7) => (),

            // ADD B (Add B to A)
            ([0x80, 0, 0], 4) => (),
            // ADD C (Add C to A)
            ([0x81, 0, 0], 4) => (),
            // ADD D (Add D to A)
            ([0x82, 0, 0], 4) => (),
            // ADD E (Add E to A)
            ([0x83, 0, 0], 4) => (),
            // ADD H (Add H to A)
            ([0x84, 0, 0], 4) => (),
            // ADD L (Add L to A)
            ([0x85, 0, 0], 4) => (),
            // ADD A (Add A to A)
            ([0x87, 0, 0], 4) => (),

            // ADI (Add immediate to A)
            ([0xC6, _, 0], 7) => (),

            // ANA M (And memory with A)
            ([0xA6, 0, 0], 7) => (),

            // ANA B (And B with A)
            ([0xA0, 0, 0], 4) => (),
            // ANA C (And C with A)
            ([0xA1, 0, 0], 4) => (),
            // ANA D (And D with A)
            ([0xA2, 0, 0], 4) => (),
            // ANA E (And E with A)
            ([0xA3, 0, 0], 4) => (),
            // ANA H (And H with A)
            ([0xA4, 0, 0], 4) => (),
            // ANA L (And L with A)
            ([0xA5, 0, 0], 4) => (),
            // ANA A (And A with A)
            ([0xA7, 0, 0], 4) => (),

            // CMP M (Compare memory with A)
            ([0xBE, 0, 0], 7) => (),

            // CMP B (Compare B with A)
            ([0xB8, 0, 0], 4) => (),
            // CMP C (Compare C with A)
            ([0xB9, 0, 0], 4) => (),
            // CMP D (Compare D with A)
            ([0xBA, 0, 0], 4) => (),
            // CMP E (Compare E with A)
            ([0xBB, 0, 0], 4) => (),
            // CMP H (Compare H with A)
            ([0xBC, 0, 0], 4) => (),
            // CMP L (Compare L with A)
            ([0xBD, 0, 0], 4) => (),
            // CMP A (Compare A with A)
            ([0xBF, 0, 0], 4) => (),

            // DCR M (Decrement memory)
            ([0x35, 0, 0], 10) => (),

            // DCX B (Decrement BC by one)
            ([0x0B, 0, 0], 5) => (),
            // DCX D (Decrement DE by one)
            ([0x1B, 0, 0], 5) => (),
            // DCX H (Decrement HL by one)
            ([0x2B, 0, 0], 5) => (),
            // DCX SP (Decrement SP by one)
            ([0x3B, 0, 0], 5) => (),

            // INR M (Increment memory)
            ([0x34, 0, 0], 10) => (),

            // LDAX B (Load A from address in B & C)
            ([0x0A, 0, 0], 7) => (),
            // LDAX D (Load A from address in D & E)
            ([0x1A, 0, 0], 7) => (),

            // LHLD (Load H & L direct)
            ([0x2A, _, _], 16) => (),

            // MOV M,B (Move B to memory)
            ([0x70, 0, 0], 7) => (),
            // MOV M,C (Move C to memory)
            ([0x71, 0, 0], 7) => (),
            // MOV M2D (Move D to memory)
            ([0x72, 0, 0], 7) => (),
            // MOV M,E (Move E to memory)
            ([0x73, 0, 0], 7) => (),
            // MOV M,H (Move H to memory)
            ([0x74, 0, 0], 7) => (),
            // MOV M,L (Move L to memory)
            ([0x75, 0, 0], 7) => (),
            // MOV M,A (Move A to memory)
            ([0x77, 0, 0], 7) => (),

            // MVI M (Move immediate to memory)
            ([0x36, _, 0], 10) => (),

            // ORA M (Or memory with A)
            ([0xB6, 0, 0], 7) => (),

            // ORA B (Or B with A)
            ([0xB0, 0, 0], 4) => (),
            // ORA C (Or C with A)
            ([0xB1, 0, 0], 4) => (),
            // ORA D (Or D with A)
            ([0xB2, 0, 0], 4) => (),
            // ORA E (Or E with A)
            ([0xB3, 0, 0], 4) => (),
            // ORA H (Or H with A)
            ([0xB4, 0, 0], 4) => (),
            // ORA L (Or L with A)
            ([0xB5, 0, 0], 4) => (),
            // ORA A (Or A with A)
            ([0xB7, 0, 0], 4) => (),

            // ORI (Or immediate with A)
            ([0xF6, _, 0], 7) => (),

            // SBB M (Subtract memory from A with borrow)
            ([0x9E, 0, 0], 7) => (),

            // SBB B (Subtract B from A with borrow)
            ([0x98, 0, 0], 4) => (),
            // SBB C (Subtract C from A with borrow)
            ([0x99, 0, 0], 4) => (),
            // SBB D (Subtract D from A with borrow)
            ([0x9A, 0, 0], 4) => (),
            // SBB E (Subtract E from A with borrow)
            ([0x9B, 0, 0], 4) => (),
            // SBB H (Subtract H from A with borrow)
            ([0x9C, 0, 0], 4) => (),
            // SBB L (Subtract L from A with borrow)
            ([0x9D, 0, 0], 4) => (),
            // SBB A (Subtract A from A with borrow)
            ([0x9F, 0, 0], 4) => (),

            // SBI (Subtract immediate from A with borrow)
            ([0xDE, _, 0], 7) => (),

            // SHLD (Store H & L direct)
            ([0x22, _, _], 16) => (),

            // STA (Store A direct)
            ([0x32, _, _], 13) => (),

            // STAX B (Store A in address in B & C)
            ([0x02, 0, 0], 7) => (),
            // STAX D (Store A in address in D & E)
            ([0x12, 0, 0], 7) => (),

            // SUB M (Subtract memory from A)
            ([0x96, 0, 0], 7) => (),

            // SUB B (Subtract B from A)
            ([0x90, 0, 0], 4) => (),
            // SUB C (Subtract C from A)
            ([0x91, 0, 0], 4) => (),
            // SUB D (Subtract D from A)
            ([0x92, 0, 0], 4) => (),
            // SUB E (Subtract E from A)
            ([0x93, 0, 0], 4) => (),
            // SUB H (Subtract H from A)
            ([0x94, 0, 0], 4) => (),
            // SUB L (Subtract L from A)
            ([0x95, 0, 0], 4) => (),
            // SUB A (Subtract A from A)
            ([0x97, 0, 0], 4) => (),

            // SUI (Subtract immediate from A)
            ([0xD6, _, 0], 7) => (),

            // XCHG (Exchange D & E, H & L registers)
            ([0xEB, 0, 0], 4) => (),

            // XRA M (Exclusive Or memory with A)
            ([0xAE, 0, 0], 7) => (),

            // XRA B (Exclusive Or B with A)
            ([0xA8, 0, 0], 4) => (),
            // XRA C (Exclusive Or C with A)
            ([0xA9, 0, 0], 4) => (),
            // XRA D (Exclusive Or D with A)
            ([0xAA, 0, 0], 4) => (),
            // XRA E (Exclusive Or E with A)
            ([0xAB, 0, 0], 4) => (),
            // XRA H (Exclusive Or H with A)
            ([0xAC, 0, 0], 4) => (),
            // XRA L (Exclusive Or L with A)
            ([0xAD, 0, 0], 4) => (),
            // XRA A (Exclusive Or A with A)
            ([0xAF, 0, 0], 4) => (),

            // XRI (Exclusive Or immediate with A)
            ([0xEE, _, 0], 7) => (),

            // DAD H (Add contents of H & L to H & L)
            ([0x29, 0, 0], u32::MAX) => break,

            otherwise => unimplemented!("{:?}", otherwise),
        }
    }
}
