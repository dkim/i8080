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

            // MVI A (Move immediate to A)
            ([0x3E, _, 0], u32::MAX) => break,

            otherwise => unimplemented!("{:?}", otherwise),
        }
    }
}
