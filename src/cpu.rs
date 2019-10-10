#![warn(rust_2018_idioms)]

use std::u32;

use bitflags::bitflags;

use crate::memory::Memory;

/// An Intel 8080 CPU.
#[derive(Default)]
pub struct Cpu {
    /// Program counter.
    pub pc: u16,

    /// Register B.
    pub b: u8,
    /// Register C.
    pub c: u8,
    /// Register D.
    pub d: u8,
    /// Register E.
    pub e: u8,
    /// Register H.
    pub h: u8,
    /// Register L.
    pub l: u8,

    /// Accumulator.
    pub a: u8,
    /// Condition flags.
    pub condition_flags: ConditionFlags,
}

impl Cpu {
    /// Fetches and executes an instruction, returning it with the number of states taken.
    pub fn fetch_execute_instruction(&mut self, memory: &mut Memory) -> (Instruction, u32) {
        let instruction = self.fetch_instruction(memory);
        let states = self.execute_instruction(instruction, memory);
        (instruction, states)
    }

    fn fetch_instruction(&mut self, memory: &Memory) -> Instruction {
        match memory[self.pc] {
            | 0x00 /* NOP */ | 0x02 /* STAX B */ | 0x03 /* INX B */ | 0x04 /* INR B */
            | 0x05 /* DCR B */ | 0x07 /* RLC */ | 0x09 /* DAD */ | 0x0A /* LDAX B */
            | 0x0B /* DCX B */ | 0x0C /* INR C */ | 0x0D /* DCR C */ | 0x0F /* RRC */
            | 0x12 /* STAX D */ | 0x13 /* INX D */ | 0x14 /* INR D */ | 0x15 /* DCR D */
            | 0x17 /* RAL */ | 0x19 /* DAD D */ | 0x1A /* LDAX D */ | 0x1B /* DCX D */
            | 0x1C /* INR E */ | 0x1D /* DCR E */ | 0x1F /* RAR */
            | 0x23 /* INX H */ | 0x24 /* INR H */ | 0x25 /* DCR H */ | 0x27 /* DAA */
            | 0x29 /* DAD H */ | 0x2B /* DCX H */ | 0x2C /* INR L */ | 0x2D /* DCR L */
            | 0x2F /* CMA */
            | 0x33 /* INX SP */ | 0x34 /* INR M */ | 0x35 /* DCR M */ | 0x37 /* STC */
            | 0x39 /* DAD SP */ | 0x3B /* DCX SP */ | 0x3C /* INR A */ | 0x3D /* DCR A */
            | 0x3F /* CMC */
            | 0x40 /* MOV B,B */ | 0x41 /* MOV B,C */ | 0x42 /* MOV B,D */ | 0x43 /* MOV B,E */
            | 0x44 /* MOV B,H */ | 0x45 /* MOV B,L */ | 0x46 /* MOV B,M */ | 0x47 /* MOV B,A */
            | 0x48 /* MOV C,B */ | 0x49 /* MOV C,C */ | 0x4A /* MOV C,D */ | 0x4B /* MOV C,E */
            | 0x4C /* MOV C,H */ | 0x4D /* MOV C,L */ | 0x4E /* MOV C,M */ | 0x4F /* MOV C,A */
            | 0x50 /* MOV D,B */ | 0x51 /* MOV D,C */ | 0x52 /* MOV D,D */ | 0x53 /* MOV D,E */
            | 0x54 /* MOV D,H */ | 0x55 /* MOV D,L */ | 0x56 /* MOV D,M */ | 0x57 /* MOV D,A */
            | 0x58 /* MOV E,B */ | 0x59 /* MOV E,C */ | 0x5A /* MOV E,D */ | 0x5B /* MOV E,E */
            | 0x5C /* MOV E,H */ | 0x5D /* MOV E,L */ | 0x5E /* MOV E,M */ | 0x5F /* MOV E,A */
            | 0x60 /* MOV H,B */ | 0x61 /* MOV H,C */ | 0x62 /* MOV H,D */ | 0x63 /* MOV H,E */
            | 0x64 /* MOV H,H */ | 0x65 /* MOV H,L */ | 0x66 /* MOV H,M */ | 0x67 /* MOV H,A */
            | 0x68 /* MOV L,B */ | 0x69 /* MOV L,C */ | 0x6A /* MOV L,D */ | 0x6B /* MOV L,E */
            | 0x6C /* MOV L,H */ | 0x6D /* MOV L,L */ | 0x6E /* MOV L,M */ | 0x6F /* MOV L,A */
            | 0x70 /* MOV M,B */ | 0x71 /* MOV M,C */ | 0x72 /* MOV M,D */ | 0x73 /* MOV M,E */
            | 0x74 /* MOV M,H */ | 0x75 /* MOV M,L */ | 0x76 /* HLT */ | 0x77 /* MOV M,A */
            | 0x78 /* MOV A,B */ | 0x79 /* MOV A,C */ | 0x7A /* MOV A,D */ | 0x7B /* MOV A,E */
            | 0x7C /* MOV A,H */ | 0x7D /* MOV A,L */ | 0x7E /* MOV A,M */ | 0x7F /* MOV A,A */
            | 0x80 /* ADD B */ | 0x81 /* ADD C */ | 0x82 /* ADD D */ | 0x83 /* ADD E */
            | 0x84 /* ADD H */ | 0x85 /* ADD L */ | 0x86 /* ADD M */ | 0x87 /* ADD A */
            | 0x88 /* ADC B */ | 0x89 /* ADC C */ | 0x8A /* ADC D */ | 0x8B /* ADC E */
            | 0x8C /* ADC H */ | 0x8D /* ADC L */ | 0x8E /* ADC M */ | 0x8F /* ADC A */
            | 0x90 /* SUB B */ | 0x91 /* SUB C */ | 0x92 /* SUB D */ | 0x93 /* SUB E */
            | 0x94 /* SUB H */ | 0x95 /* SUB L */ | 0x96 /* SUB M */ | 0x97 /* SUB A */
            | 0x98 /* SBB B */ | 0x99 /* SBB C */ | 0x9A /* SBB D */ | 0x9B /* SBB E */
            | 0x9C /* SBB H */ | 0x9D /* SBB L */ | 0x9E /* SBB M */ | 0x9F /* SBB A */
            | 0xA0 /* ANA B */ | 0xA1 /* ANA C */ | 0xA2 /* ANA D */ | 0xA3 /* ANA E */
            | 0xA4 /* ANA H */ | 0xA5 /* ANA L */ | 0xA6 /* ANA M */ | 0xA7 /* ANA A */
            | 0xA8 /* XRA B */ | 0xA9 /* XRA C */ | 0xAA /* XRA D */ | 0xAB /* XRA E */
            | 0xAC /* XRA H */ | 0xAD /* XRA L */ | 0xAE /* XRA M */ | 0xAF /* XRA A */
            | 0xB0 /* ORA B */ | 0xB1 /* ORA C */ | 0xB2 /* ORA D */ | 0xB3 /* ORA E */
            | 0xB4 /* ORA H */ | 0xB5 /* ORA L */ | 0xB6 /* ORA M */ | 0xB7 /* ORA A */
            | 0xB8 /* CMP B */ | 0xB9 /* CMP C */ | 0xBA /* CMP D */ | 0xBB /* CMP E */
            | 0xBC /* CMP H */ | 0xBD /* CMP L */ | 0xBE /* CMP M */ | 0xBF /* CMP A */
            | 0xC0 /* RNZ */ | 0xC1 /* POP B */ | 0xC5 /* PUSH B */ | 0xC7 /* RST 0 */
            | 0xC8 /* RZ */ | 0xC9 /* RET */ | 0xCF /* RST 1 */
            | 0xD0 /* RNC */ | 0xD1 /* POP D */ | 0xD5 /* PUSH D */ | 0xD7 /* RST 2 */
            | 0xD8 /* RC */ | 0xDF /* RST 3 */
            | 0xE0 /* RPO */ | 0xE1 /* POP H */ | 0xE3 /* XTHL */ | 0xE5 /* PUSH H */
            | 0xE7 /* RST 4 */ | 0xE8 /* RPE */ | 0xE9 /* PCHL */ | 0xEB /* XCHG */
            | 0xEF /* RST 5 */
            | 0xF0 /* RP */ | 0xF1 /* POP PSW */ | 0xF3 /* DI */ | 0xF5 /* PUSH PSW */
            | 0xF7 /* RST 6 */ | 0xF8 /* RM */ | 0xF9 /* SPHL */ | 0xFB /* EI */
            | 0xFF /* RST 7 */ => {
                let instruction = [memory[self.pc], 0, 0];
                self.pc += 1;
                instruction
            }
            | 0x06 /* MVI B */ | 0x0E /* MVI C */
            | 0x16 /* MVI D */ | 0x1E /* MVI E */
            | 0x26 /* MVI H */ | 0x2E /* MVI L */
            | 0x36 /* MVI M */ | 0x3E /* MVI A */
            | 0xC6 /* ADI */ | 0xCE /* ACI */
            | 0xD3 /* OUT */ | 0xD6 /* SUI */ | 0xDB /* IN */ | 0xDE /* SBI */
            | 0xE6 /* ANI */ | 0xEE /* XRI */
            | 0xF6 /* ORI */ | 0xFE /* CPI */ => {
                let instruction = [memory[self.pc], memory[self.pc + 1], 0];
                self.pc += 2;
                instruction
            }
            | 0x01 /* LXI B */
            | 0x11 /* LXI D */
            | 0x21 /* LXI H */ | 0x22 /* SHLD */ | 0x2A /* LHLD */
            | 0x31 /* LXI SP */ | 0x32 /* STA */ | 0x3A /* LDA */
            | 0xC2 /* JNZ */ | 0xC3 /* JMP */ | 0xC4 /* CNZ */ | 0xCA /* JZ */
            | 0xCC /* CZ */ | 0xCD /* CALL */
            | 0xD2 /* JNC */ | 0xD4 /* CNC */ | 0xDA /* JC */ | 0xDC /* CC */
            | 0xE2 /* JPO */ | 0xE4 /* CPO */ | 0xEA /* JPE */ | 0xEC /* CPE */
            | 0xF2 /* JP */ | 0xF4 /* CP */ | 0xFA /* JM */ | 0xFC /* CM */ => {
                let instruction =
                    [memory[self.pc], memory[self.pc + 1], memory[self.pc + 2]];
                self.pc += 3;
                instruction
            }
            instruction => unimplemented!("{:#04X?} (undocumented)", instruction),
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction, _memory: &mut Memory) -> u32 {
        match instruction[0] {
            // CPI (Compare immediate with A)
            0xFE => {
                let (_, borrow_out) = self.subtract(self.a, instruction[1], false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                7
            }

            // JMP (Jump unconditional)
            0xC3 => {
                self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                10
            }

            // JNZ (Jump on no zero)
            0xC2 => {
                if !self.condition_flags.contains(ConditionFlags::ZERO) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }
            // JZ (Jump on zero)
            0xCA => {
                if self.condition_flags.contains(ConditionFlags::ZERO) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }
            // JNC (Jump on no carry)
            0xD2 => {
                if !self.condition_flags.contains(ConditionFlags::CARRY) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }
            // JC (Jump on carry)
            0xDA => {
                if self.condition_flags.contains(ConditionFlags::CARRY) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }
            // JPO (Jump on parity odd)
            0xE2 => {
                if !self.condition_flags.contains(ConditionFlags::PARITY) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }
            // JPE (Jump on parity even)
            0xEA => {
                if self.condition_flags.contains(ConditionFlags::PARITY) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }
            // JP (Jump on positive)
            0xF2 => {
                if !self.condition_flags.contains(ConditionFlags::SIGN) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }
            // JM (Jump on minus)
            0xFA => {
                if self.condition_flags.contains(ConditionFlags::SIGN) {
                    self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                }
                10
            }

            // MVI B (Move immediate to B)
            0x06 => {
                self.b = instruction[1];
                7
            }
            // MVI C (Move immediate to C)
            0x0E => {
                self.c = instruction[1];
                7
            }
            // MVI D (Move immediate to D)
            0x16 => {
                self.d = instruction[1];
                7
            }
            // MVI E (Move immediate to E)
            0x1E => {
                self.e = instruction[1];
                7
            }
            // MVI H (Move immediate to D)
            0x26 => {
                self.h = instruction[1];
                7
            }
            // MVI L (Move immediate to E)
            0x2E => {
                self.l = instruction[1];
                7
            }
            // MVI A (Move immediate to A)
            0x3E => {
                self.a = instruction[1];
                7
            }

            _ => u32::MAX,
        }
    }

    fn add(&mut self, x: u8, y: u8, carry_in: bool) -> (u8, bool) {
        self.condition_flags.set(
            ConditionFlags::AUX_CARRY,
            if carry_in { (x & 0x0F) + (y & 0x0F) >= 0x0F } else { (x & 0x0F) + (y & 0x0F) > 0x0F },
        );
        let result = if carry_in { x.wrapping_add(y).wrapping_add(1) } else { x.wrapping_add(y) };
        self.update_parity_zero_sign_flags(result);
        (result, if carry_in { x >= 0xFF - y } else { x > 0xFF - y })
    }

    fn subtract(&mut self, x: u8, y: u8, borrow_in: bool) -> (u8, bool) {
        // Refer to https://retrocomputing.stackexchange.com/q/12558 for details on the behavior of
        // the auxiliary carry flag in subtraction.
        let (result, carry_out) = self.add(x, !y, !borrow_in);
        let borrow_out = !carry_out;
        (result, borrow_out)
    }

    fn update_parity_zero_sign_flags(&mut self, result: u8) {
        self.condition_flags.set(ConditionFlags::PARITY, result.count_ones() % 2 == 0);
        self.condition_flags.set(ConditionFlags::ZERO, result == 0);
        self.condition_flags.set(ConditionFlags::SIGN, result & 0x80 > 0);
    }
}

/// A type alias for `[u8; 3]` that represents an instruction. If the instruction is shorter than 3
/// bytes, it is padded with null bytes at the end.
pub type Instruction = [u8; 3];

bitflags! {
    /// A byte that holds the settings of the condition flags:
    ///
    /// <table>
    /// <tr> <th>Bit</th> <th>Condition Flag</th> </tr>
    /// <tr> <td>7</td>   <td>Sign</td> </tr>
    /// <tr> <td>6</td>   <td>Zero</td> </tr>
    /// <tr> <td>5</td>   <td>0</td> </tr>
    /// <tr> <td>4</td>   <td>Auxiliary Carry</td> </tr>
    /// <tr> <td>3</td>   <td>0</td> </tr>
    /// <tr> <td>2</td>   <td>Parity</td> </tr>
    /// <tr> <td>1</td>   <td>1</td> </tr>
    /// <tr> <td>0</td>   <td>Carry</td> </tr>
    /// </table>
    pub struct ConditionFlags: u8 {
        const CARRY = 0b0000_0001;
        const ALWAYS_ONE = 0b0000_0010;
        const PARITY = 0b0000_0100;
        const AUX_CARRY = 0b0001_0000; // auxiliary carry
        const ZERO = 0b0100_0000;
        const SIGN = 0b1000_0000;
    }
}

impl Default for ConditionFlags {
    fn default() -> Self {
        ConditionFlags::ALWAYS_ONE
    }
}

#[cfg(test)]
mod tests;
