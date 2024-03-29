use std::mem;

use bitflags::bitflags;

use crate::{memory::Memory, Error, Result};

/// An Intel 8080 CPU.
#[derive(Default)]
pub struct Cpu {
    /// Program counter.
    pub pc: u16,
    /// Stack pointer.
    pub sp: u16,

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

    interruptable: Interruptable,
    is_halted: bool,
}

impl Cpu {
    /// Fetches and executes an instruction, returning it with the number of states taken.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::Halted`] error if the CPU is in the halted state.
    ///
    /// [`Error::Halted`]: ../enum.Error.html#variant.Halted
    pub fn fetch_execute_instruction(&mut self, memory: &mut Memory) -> Result<(Instruction, u32)> {
        if self.is_halted {
            return Err(Error::Halted);
        }
        let instruction = self.fetch_instruction(memory);
        let interruptable = self.interruptable;
        let states = self.execute_instruction(instruction, memory);
        // XXX: If two EI instructions occur consecutively, the interrupt system is enabled
        // immediately following the execution of the second EI instruction.
        if let (Interruptable::Enabling, Interruptable::Enabling) =
            (interruptable, self.interruptable)
        {
            self.interruptable = Interruptable::Enabled;
        }
        Ok((instruction, states))
    }

    /// Escapes from the halt state, if necessary, and executes `instruction` with further
    /// interrupts disabled.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InterruptNotEnabled`] error if the interrupt system
    /// is already disabled.
    ///
    /// [`Error::InterruptNotEnabled`]: ../enum.Error.html#variant.InterruptNotEnabled
    pub fn interrupt(&mut self, instruction: Instruction, memory: &mut Memory) -> Result<u32> {
        if let Interruptable::Enabled = self.interruptable {
            self.is_halted = false;
            self.interruptable = Interruptable::Disabled;
            Ok(self.execute_instruction(instruction, memory))
        } else {
            Err(Error::InterruptNotEnabled)
        }
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

    #[allow(clippy::cognitive_complexity)]
    fn execute_instruction(&mut self, instruction: Instruction, memory: &mut Memory) -> u32 {
        match instruction[0] {
            // ACI (Add immediate to A with carry)
            0xCE => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, instruction[1], carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                7
            }

            // ADC M (Add memory to A with carry)
            0x8E => {
                let address = u16::from_le_bytes([self.l, self.h]);
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, memory[address], carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                7
            }

            // ADC B (Add B to A with carry)
            0x88 => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, self.b, carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADC C (Add C to A with carry)
            0x89 => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, self.c, carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADC D (Add D to A with carry)
            0x8A => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, self.d, carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADC E (Add E to A with carry)
            0x8B => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, self.e, carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADC H (Add H to A with carry)
            0x8C => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, self.h, carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADC L (Add L to A with carry)
            0x8D => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, self.l, carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADC A (Add A to A with carry)
            0x8F => {
                let carry_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, carry_out) = self.add(self.a, self.a, carry_in);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }

            // ADD M (Add memory to A)
            0x86 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                let (result, carry_out) = self.add(self.a, memory[address], false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                7
            }

            // ADD B (Add B to A)
            0x80 => {
                let (result, carry_out) = self.add(self.a, self.b, false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADD C (Add C to A)
            0x81 => {
                let (result, carry_out) = self.add(self.a, self.c, false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADD D (Add D to A)
            0x82 => {
                let (result, carry_out) = self.add(self.a, self.d, false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADD E (Add E to A)
            0x83 => {
                let (result, carry_out) = self.add(self.a, self.e, false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADD H (Add H to A)
            0x84 => {
                let (result, carry_out) = self.add(self.a, self.h, false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADD L (Add L to A)
            0x85 => {
                let (result, carry_out) = self.add(self.a, self.l, false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }
            // ADD A (Add A to A)
            0x87 => {
                let (result, carry_out) = self.add(self.a, self.a, false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                4
            }

            // ADI (Add immediate to A)
            0xC6 => {
                let (result, carry_out) = self.add(self.a, instruction[1], false);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.a = result;
                7
            }

            // ANA M (And memory with A)
            0xA6 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.logical_and(memory[address]);
                7
            }

            // ANA B (And B with A)
            0xA0 => {
                self.logical_and(self.b);
                4
            }
            // ANA C (And C with A)
            0xA1 => {
                self.logical_and(self.c);
                4
            }
            // ANA D (And D with A)
            0xA2 => {
                self.logical_and(self.d);
                4
            }
            // ANA E (And E with A)
            0xA3 => {
                self.logical_and(self.e);
                4
            }
            // ANA H (And H with A)
            0xA4 => {
                self.logical_and(self.h);
                4
            }
            // ANA L (And L with A)
            0xA5 => {
                self.logical_and(self.l);
                4
            }
            // ANA A (And A with A)
            0xA7 => {
                self.logical_and(self.a);
                4
            }

            // ANI (And immediate with A)
            0xE6 => {
                self.logical_and(instruction[1]);
                7
            }

            // CALL (Call unconditional)
            0xCD => {
                self.call(instruction, memory);
                17
            }
            // CALL (Call unconditional, undocumented)
            0xDD | 0xED | 0xFD => {
                self.call(instruction, memory);
                17
            }

            // CMA (Complement A)
            0x2F => {
                self.a = !self.a;
                4
            }

            // CMC (Complement carry flag)
            0x3F => {
                self.condition_flags.toggle(ConditionFlags::CARRY);
                4
            }

            // CMP M (Compare memory with A)
            0xBE => {
                let address = u16::from_le_bytes([self.l, self.h]);
                let (_, borrow_out) = self.subtract(self.a, memory[address], false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                7
            }

            // CMP B (Compare B with A)
            0xB8 => {
                let (_, borrow_out) = self.subtract(self.a, self.b, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                4
            }
            // CMP C (Compare C with A)
            0xB9 => {
                let (_, borrow_out) = self.subtract(self.a, self.c, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                4
            }
            // CMP D (Compare D with A)
            0xBA => {
                let (_, borrow_out) = self.subtract(self.a, self.d, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                4
            }
            // CMP E (Compare E with A)
            0xBB => {
                let (_, borrow_out) = self.subtract(self.a, self.e, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                4
            }
            // CMP H (Compare H with A)
            0xBC => {
                let (_, borrow_out) = self.subtract(self.a, self.h, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                4
            }
            // CMP L (Compare L with A)
            0xBD => {
                let (_, borrow_out) = self.subtract(self.a, self.l, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                4
            }
            // CMP A (Compare A with A)
            0xBF => {
                let (_, borrow_out) = self.subtract(self.a, self.a, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                4
            }

            // CNZ (Call on no zero)
            0xC4 => {
                if !self.condition_flags.contains(ConditionFlags::ZERO) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }
            // CZ (Call on zero)
            0xCC => {
                if self.condition_flags.contains(ConditionFlags::ZERO) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }
            // CNC (Call on no carry)
            0xD4 => {
                if !self.condition_flags.contains(ConditionFlags::CARRY) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }
            // CC (Call on carry)
            0xDC => {
                if self.condition_flags.contains(ConditionFlags::CARRY) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }
            // CPO (Call on parity odd)
            0xE4 => {
                if !self.condition_flags.contains(ConditionFlags::PARITY) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }
            // CPE (Call on parity even)
            0xEC => {
                if self.condition_flags.contains(ConditionFlags::PARITY) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }
            // CP (Call on postive)
            0xF4 => {
                if !self.condition_flags.contains(ConditionFlags::SIGN) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }
            // CM (Call on minus)
            0xFC => {
                if self.condition_flags.contains(ConditionFlags::SIGN) {
                    self.call(instruction, memory);
                    17
                } else {
                    11
                }
            }

            // CPI (Compare immediate with A)
            0xFE => {
                let (_, borrow_out) = self.subtract(self.a, instruction[1], false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                7
            }

            // DAA (Decimal adjust A)
            0x27 => {
                if self.a & 0x0F > 0x09 || self.condition_flags.contains(ConditionFlags::AUX_CARRY)
                {
                    let (result, carry) = self.a.overflowing_add(0x06);
                    if carry {
                        self.condition_flags.insert(ConditionFlags::CARRY);
                    }
                    self.condition_flags
                        .set(ConditionFlags::AUX_CARRY, (self.a & 0x0F) + 0x06 > 0x0F);
                    self.a = result
                }
                if self.a & 0xF0 > 0x90 || self.condition_flags.contains(ConditionFlags::CARRY) {
                    self.a = self.a.wrapping_add(0x60);
                    self.condition_flags.insert(ConditionFlags::CARRY);
                }
                self.update_parity_zero_sign_flags(self.a);
                4
            }

            // DAD B (Add contents of B & C to H & L)
            0x09 => {
                let (result, carry_out) = u16::from_le_bytes([self.l, self.h])
                    .overflowing_add(u16::from_le_bytes([self.c, self.b]));
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.h = ((result & 0xFF00) >> 8) as u8;
                self.l = (result & 0x00FF) as u8;
                10
            }
            // DAD D (Add contents of D & E to H & L)
            0x19 => {
                let (result, carry_out) = u16::from_le_bytes([self.l, self.h])
                    .overflowing_add(u16::from_le_bytes([self.e, self.d]));
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.h = ((result & 0xFF00) >> 8) as u8;
                self.l = (result & 0x00FF) as u8;
                10
            }
            // DAD H (Add contents of H & L to H & L)
            0x29 => {
                let (result, carry_out) = u16::from_le_bytes([self.l, self.h])
                    .overflowing_add(u16::from_le_bytes([self.l, self.h]));
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.h = ((result & 0xFF00) >> 8) as u8;
                self.l = (result & 0x00FF) as u8;
                10
            }
            // DAD SP (Add contents of SP to H & L)
            0x39 => {
                let (result, carry_out) =
                    u16::from_le_bytes([self.l, self.h]).overflowing_add(self.sp);
                self.condition_flags.set(ConditionFlags::CARRY, carry_out);
                self.h = ((result & 0xFF00) >> 8) as u8;
                self.l = (result & 0x00FF) as u8;
                10
            }

            // DCR M (Decrement memory)
            0x35 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                let (result, _) = self.subtract(memory[address], 1, false);
                memory[address] = result;
                10
            }

            // DCR B (Decrement B)
            0x05 => {
                let (result, _) = self.subtract(self.b, 1, false);
                self.b = result;
                5
            }
            // DCR C (Decrement C)
            0x0D => {
                let (result, _) = self.subtract(self.c, 1, false);
                self.c = result;
                5
            }
            // DCR D (Decrement D)
            0x15 => {
                let (result, _) = self.subtract(self.d, 1, false);
                self.d = result;
                5
            }
            // DCR E (Decrement E)
            0x1D => {
                let (result, _) = self.subtract(self.e, 1, false);
                self.e = result;
                5
            }
            // DCR H (Decrement H)
            0x25 => {
                let (result, _) = self.subtract(self.h, 1, false);
                self.h = result;
                5
            }
            // DCR L (Decrement L)
            0x2D => {
                let (result, _) = self.subtract(self.l, 1, false);
                self.l = result;
                5
            }
            // DCR A (Decrement A)
            0x3D => {
                let (result, _) = self.subtract(self.a, 1, false);
                self.condition_flags.set(ConditionFlags::SIGN, result & 0x80 > 0);
                self.a = result;
                5
            }

            // DCX B (Decrement BC by one)
            0x0B => {
                let result = u16::from_le_bytes([self.c, self.b]).wrapping_sub(1);
                self.b = ((result & 0xFF00) >> 8) as u8;
                self.c = (result & 0x00FF) as u8;
                5
            }
            // DCX D (Decrement DE by one)
            0x1B => {
                let result = u16::from_le_bytes([self.e, self.d]).wrapping_sub(1);
                self.d = ((result & 0xFF00) >> 8) as u8;
                self.e = (result & 0x00FF) as u8;
                5
            }
            // DCX H (Decrement HL by one)
            0x2B => {
                let result = u16::from_le_bytes([self.l, self.h]).wrapping_sub(1);
                self.h = ((result & 0xFF00) >> 8) as u8;
                self.l = (result & 0x00FF) as u8;
                5
            }
            // DCX SP (Decrement SP by one)
            0x3B => {
                self.sp = self.sp.wrapping_sub(1);
                5
            }

            // DI (Disable interrupt system)
            0xF3 => {
                self.interruptable = Interruptable::Disabled;
                4
            }

            // EI (Enable interrupt system)
            0xFB => {
                if let Interruptable::Disabled = self.interruptable {
                    self.interruptable = Interruptable::Enabling;
                }
                4
            }

            // HLT (Halt)
            0x76 => {
                self.is_halted = true;
                7
            }

            // IN port (Initiate input operation)
            0xDB => 10,

            // INR M (Increment memory)
            0x34 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                let (result, _) = self.add(memory[address], 1, false);
                memory[address] = result;
                10
            }

            // INR B (Increment B)
            0x04 => {
                let (result, _) = self.add(self.b, 1, false);
                self.b = result;
                5
            }
            // INR C (Increment C)
            0x0C => {
                let (result, _) = self.add(self.c, 1, false);
                self.c = result;
                5
            }
            // INR D (Increment D)
            0x14 => {
                let (result, _) = self.add(self.d, 1, false);
                self.d = result;
                5
            }
            // INR E (Increment E)
            0x1C => {
                let (result, _) = self.add(self.e, 1, false);
                self.e = result;
                5
            }
            // INR H (Increment H)
            0x24 => {
                let (result, _) = self.add(self.h, 1, false);
                self.h = result;
                5
            }
            // INR L (Increment L)
            0x2C => {
                let (result, _) = self.add(self.l, 1, false);
                self.l = result;
                5
            }
            // INR A (Increment A)
            0x3C => {
                let (result, _) = self.add(self.a, 1, false);
                self.a = result;
                5
            }

            // INX B (Increment BC by one)
            0x03 => {
                let result = u16::from_le_bytes([self.c, self.b]).wrapping_add(1);
                self.b = ((result & 0xFF00) >> 8) as u8;
                self.c = (result & 0x00FF) as u8;
                5
            }
            // INX D (Increment DE by one)
            0x13 => {
                let result = u16::from_le_bytes([self.e, self.d]).wrapping_add(1);
                self.d = ((result & 0xFF00) >> 8) as u8;
                self.e = (result & 0x00FF) as u8;
                5
            }
            // INX H (Increment HL by one)
            0x23 => {
                let result = u16::from_le_bytes([self.l, self.h]).wrapping_add(1);
                self.h = ((result & 0xFF00) >> 8) as u8;
                self.l = (result & 0x00FF) as u8;
                5
            }
            // INX SP (Increment SP by one)
            0x33 => {
                self.sp = self.sp.wrapping_add(1);
                5
            }

            // JMP (Jump unconditional)
            0xC3 => {
                self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
                10
            }
            // JMP (Jump unconditional, undocumented)
            0xCB => {
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

            // LDA (Load A direct)
            0x3A => {
                let address = u16::from_le_bytes([instruction[1], instruction[2]]);
                self.a = memory[address];
                13
            }

            // LDAX B (Load A from address in B & C)
            0x0A => {
                let address = u16::from_le_bytes([self.c, self.b]);
                self.a = memory[address];
                7
            }
            // LDAX D (Load A from address in D & E)
            0x1A => {
                let address = u16::from_le_bytes([self.e, self.d]);
                self.a = memory[address];
                7
            }

            // LHLD (Load H & L direct)
            0x2A => {
                let address = u16::from_le_bytes([instruction[1], instruction[2]]);
                self.l = memory[address];
                self.h = memory[address.wrapping_add(1)];
                16
            }

            // LXI SP (Load immediate stack pointer)
            0x31 => {
                self.sp = u16::from_le_bytes([instruction[1], instruction[2]]);
                10
            }

            // LXI B (Load immediate register pair B & C)
            0x01 => {
                self.b = instruction[2];
                self.c = instruction[1];
                10
            }
            // LXI D (Load immediate register pair D & E)
            0x11 => {
                self.d = instruction[2];
                self.e = instruction[1];
                10
            }
            // LXI H (Load immediate register pair H & L)
            0x21 => {
                self.h = instruction[2];
                self.l = instruction[1];
                10
            }

            // MOV B,M (Move memory to B)
            0x46 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.b = memory[address];
                7
            }
            // MOV C,M (Move memory to C)
            0x4E => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.c = memory[address];
                7
            }
            // MOV D,M (Move memory to D)
            0x56 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.d = memory[address];
                7
            }
            // MOV E,M (Move memory to E)
            0x5E => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.e = memory[address];
                7
            }
            // MOV H,M (Move memory to H)
            0x66 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.h = memory[address];
                7
            }
            // MOV L,M (Move memory to L)
            0x6E => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.l = memory[address];
                7
            }
            // MOV A,M (Move memory to A)
            0x7E => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.a = memory[address];
                7
            }

            // MOV B,B (Move B to B)
            0x40 => 5,
            // MOV B,C (Move C to B)
            0x41 => {
                self.b = self.c;
                5
            }
            // MOV B,D (Move D to B)
            0x42 => {
                self.b = self.d;
                5
            }
            // MOV B,E (Move E to B)
            0x43 => {
                self.b = self.e;
                5
            }
            // MOV B,H (Move H to B)
            0x44 => {
                self.b = self.h;
                5
            }
            // MOV B,L (Move L to B)
            0x45 => {
                self.b = self.l;
                5
            }
            // MOV B,A (Move A to B)
            0x47 => {
                self.b = self.a;
                5
            }
            // MOV C,B (Move B to C)
            0x48 => {
                self.c = self.b;
                5
            }
            // MOV C,C (Move B to C)
            0x49 => 5,
            // MOV C,D (Move D to C)
            0x4A => {
                self.c = self.d;
                5
            }
            // MOV C,E (Move E to C)
            0x4B => {
                self.c = self.e;
                5
            }
            // MOV C,H (Move H to C)
            0x4C => {
                self.c = self.h;
                5
            }
            // MOV C,L (Move L to C)
            0x4D => {
                self.c = self.l;
                5
            }
            // MOV C,A (Move A to C)
            0x4F => {
                self.c = self.a;
                5
            }
            // MOV D,B (Move B to D)
            0x50 => {
                self.d = self.b;
                5
            }
            // MOV D,C (Move C to D)
            0x51 => {
                self.d = self.c;
                5
            }
            // MOV D,D (Move D to D)
            0x52 => 5,
            // MOV D,E (Move E to D)
            0x53 => {
                self.d = self.e;
                5
            }
            // MOV D,H (Move H to D)
            0x54 => {
                self.d = self.h;
                5
            }
            // MOV D,L (Move L to D)
            0x55 => {
                self.d = self.l;
                5
            }
            // MOV D,A (Move A to D)
            0x57 => {
                self.d = self.a;
                5
            }
            // MOV E,B (Move B to E)
            0x58 => {
                self.e = self.b;
                5
            }
            // MOV E,C (Move C to E)
            0x59 => {
                self.e = self.c;
                5
            }
            // MOV E,D (Move D to E)
            0x5A => {
                self.e = self.d;
                5
            }
            // MOV E,E (Move E to E)
            0x5B => 5,
            // MOV E,H (Move H to E)
            0x5C => {
                self.e = self.h;
                5
            }
            // MOV E,L (Move L to E)
            0x5D => {
                self.e = self.l;
                5
            }
            // MOV E,A (Move A to E)
            0x5F => {
                self.e = self.a;
                5
            }
            // MOV H,B (Move B to H)
            0x60 => {
                self.h = self.b;
                5
            }
            // MOV H,C (Move C to H)
            0x61 => {
                self.h = self.c;
                5
            }
            // MOV H,D (Move D to H)
            0x62 => {
                self.h = self.d;
                5
            }
            // MOV H,E (Move E to H)
            0x63 => {
                self.h = self.e;
                5
            }
            // MOV H,H (Move H to H)
            0x64 => 5,
            // MOV H,L (Move L to H)
            0x65 => {
                self.h = self.l;
                5
            }
            // MOV H,A (Move A to H)
            0x67 => {
                self.h = self.a;
                5
            }
            // MOV L,B (Move B to L)
            0x68 => {
                self.l = self.b;
                5
            }
            // MOV L,C (Move C to L)
            0x69 => {
                self.l = self.c;
                5
            }
            // MOV L,D (Move D to L)
            0x6A => {
                self.l = self.d;
                5
            }
            // MOV L,E (Move E to L)
            0x6B => {
                self.l = self.e;
                5
            }
            // MOV L,H (Move H to L)
            0x6C => {
                self.l = self.h;
                5
            }
            // MOV L,L (Move L to L)
            0x6D => 5,
            // MOV L,A (Move A to L)
            0x6F => {
                self.l = self.a;
                5
            }
            // MOV A,B (Move B to A)
            0x78 => {
                self.a = self.b;
                5
            }
            // MOV A,C (Move C to A)
            0x79 => {
                self.a = self.c;
                5
            }
            // MOV A,D (Move D to A)
            0x7A => {
                self.a = self.d;
                5
            }
            // MOV A,E (Move E to A)
            0x7B => {
                self.a = self.e;
                5
            }
            // MOV A,H (Move H to A)
            0x7C => {
                self.a = self.h;
                5
            }
            // MOV A,L (Move L to A)
            0x7D => {
                self.a = self.l;
                5
            }
            // MOV A,A (Move A to A)
            0x7F => 5,

            // MOV M,B (Move B to memory)
            0x70 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = self.b;
                7
            }
            // MOV M,C (Move C to memory)
            0x71 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = self.c;
                7
            }
            // MOV M,D (Move D to memory)
            0x72 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = self.d;
                7
            }
            // MOV M,E (Move E to memory)
            0x73 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = self.e;
                7
            }
            // MOV M,H (Move H to memory)
            0x74 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = self.h;
                7
            }
            // MOV M,L (Move L to memory)
            0x75 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = self.l;
                7
            }
            // MOV M,A (Move A to memory)
            0x77 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = self.a;
                7
            }

            // MVI M (Move immediate to memory)
            0x36 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                memory[address] = instruction[1];
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

            // NOP (No operation)
            0x00 => 4,
            // NOP (No operation, undocumented)
            0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => 4,

            // ORA M (Or memory with A)
            0xB6 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.logical_or(memory[address]);
                7
            }

            // ORA B (Or B with A)
            0xB0 => {
                self.logical_or(self.b);
                4
            }
            // ORA C (Or C with A)
            0xB1 => {
                self.logical_or(self.c);
                4
            }
            // ORA D (Or D with A)
            0xB2 => {
                self.logical_or(self.d);
                4
            }
            // ORA E (Or E with A)
            0xB3 => {
                self.logical_or(self.e);
                4
            }
            // ORA H (Or H with A)
            0xB4 => {
                self.logical_or(self.h);
                4
            }
            // ORA L (Or L with A)
            0xB5 => {
                self.logical_or(self.l);
                4
            }
            // ORA A (Or A with A)
            0xB7 => {
                self.logical_or(self.a);
                4
            }

            // ORI (Or immediate with A)
            0xF6 => {
                self.logical_or(instruction[1]);
                7
            }

            // OUT port (Initiate output operation)
            0xD3 => 10,

            // PCHL (H & L to program counter)
            0xE9 => {
                self.pc = u16::from_le_bytes([self.l, self.h]);
                5
            }

            // POP PSW (Pop A and Flags off stack)
            0xF1 => {
                self.condition_flags = ConditionFlags::from_bits_truncate(
                    memory[self.sp] | ConditionFlags::ALWAYS_ONE.bits(),
                );
                self.a = memory[self.sp.wrapping_add(1)];
                self.sp = self.sp.wrapping_add(2);
                10
            }

            // POP B (Pop register pair B & C off stack)
            0xC1 => {
                self.c = memory[self.sp];
                self.b = memory[self.sp.wrapping_add(1)];
                self.sp = self.sp.wrapping_add(2);
                10
            }
            // POP D (Pop register pair D & E off stack)
            0xD1 => {
                self.e = memory[self.sp];
                self.d = memory[self.sp.wrapping_add(1)];
                self.sp = self.sp.wrapping_add(2);
                10
            }
            // POP H (Pop register pair H & L off stack)
            0xE1 => {
                self.l = memory[self.sp];
                self.h = memory[self.sp.wrapping_add(1)];
                self.sp = self.sp.wrapping_add(2);
                10
            }

            // PUSH PSW (Push A and Flags on stack)
            0xF5 => {
                memory[self.sp.wrapping_sub(1)] = self.a;
                memory[self.sp.wrapping_sub(2)] = self.condition_flags.bits();
                self.sp = self.sp.wrapping_sub(2);
                11
            }

            // PUSH B (Push register pair B & C on stack)
            0xC5 => {
                memory[self.sp.wrapping_sub(1)] = self.b;
                memory[self.sp.wrapping_sub(2)] = self.c;
                self.sp = self.sp.wrapping_sub(2);
                11
            }
            // PUSH D (Push register pair D & E on stack)
            0xD5 => {
                memory[self.sp.wrapping_sub(1)] = self.d;
                memory[self.sp.wrapping_sub(2)] = self.e;
                self.sp = self.sp.wrapping_sub(2);
                11
            }
            // PUSH H (Push register pair H & L on stack)
            0xE5 => {
                memory[self.sp.wrapping_sub(1)] = self.h;
                memory[self.sp.wrapping_sub(2)] = self.l;
                self.sp = self.sp.wrapping_sub(2);
                11
            }

            // RET (Return)
            0xC9 => {
                self.ret(memory);
                10
            }
            // RET (Return, undocumented)
            0xD9 => {
                self.ret(memory);
                10
            }

            // RAL (Rotage left through carry)
            0x17 => {
                let carry = self.condition_flags.contains(ConditionFlags::CARRY);
                self.condition_flags.set(ConditionFlags::CARRY, self.a & 0x80 > 0);
                // The left shift operator `<<` fills blank spaces up by zeroes.
                self.a = if carry { (self.a << 1) | 0x01 } else { self.a << 1 };
                4
            }

            // RAR (Rotage right through carry)
            0x1F => {
                let carry = self.condition_flags.contains(ConditionFlags::CARRY);
                self.condition_flags.set(ConditionFlags::CARRY, self.a & 0x01 > 0);
                // The right shift operator `>>` performed on an unsigned type fills blank spaces
                // up by zeroes.
                self.a = if carry { (self.a >> 1) | 0x80 } else { self.a >> 1 };
                4
            }

            // RLC (Rotate A left)
            0x07 => {
                self.condition_flags.set(ConditionFlags::CARRY, self.a & 0x80 > 0);
                self.a = self.a.rotate_left(1);
                4
            }

            // RNZ (Return on no zero)
            0xC0 => {
                if !self.condition_flags.contains(ConditionFlags::ZERO) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }
            // RZ (Return on zero)
            0xC8 => {
                if self.condition_flags.contains(ConditionFlags::ZERO) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }
            // RNC (Return on no carry)
            0xD0 => {
                if !self.condition_flags.contains(ConditionFlags::CARRY) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }
            // RC (Return on carry)
            0xD8 => {
                if self.condition_flags.contains(ConditionFlags::CARRY) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }
            // RPO (Return on parity odd)
            0xE0 => {
                if !self.condition_flags.contains(ConditionFlags::PARITY) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }
            // RPE (Return on parity even)
            0xE8 => {
                if self.condition_flags.contains(ConditionFlags::PARITY) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }
            // RP (Return on positive)
            0xF0 => {
                if !self.condition_flags.contains(ConditionFlags::SIGN) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }
            // RM (Return on minus)
            0xF8 => {
                if self.condition_flags.contains(ConditionFlags::SIGN) {
                    self.ret(memory);
                    11
                } else {
                    5
                }
            }

            // RRC (Rotate A right)
            0x0F => {
                self.condition_flags.set(ConditionFlags::CARRY, self.a & 0x01 > 0);
                self.a = self.a.rotate_right(1);
                4
            }

            // RST n (Restart n)
            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => {
                self.restart(instruction[0], memory);
                11
            }

            // SBB M (Subtract memory from A with borrow)
            0x9E => {
                let address = u16::from_le_bytes([self.l, self.h]);
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, memory[address], borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                7
            }

            // SBB B (Subtract B from A with borrow)
            0x98 => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, self.b, borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SBB C (Subtract C from A with borrow)
            0x99 => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, self.c, borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SBB D (Subtract D from A with borrow)
            0x9A => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, self.d, borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SBB E (Subtract E from A with borrow)
            0x9B => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, self.e, borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SBB H (Subtract H from A with borrow)
            0x9C => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, self.h, borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SBB L (Subtract L from A with borrow)
            0x9D => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, self.l, borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SBB A (Subtract A from A with borrow)
            0x9F => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, self.a, borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }

            // SBI (Subtract immediate from A with borrow)
            0xDE => {
                let borrow_in = self.condition_flags.contains(ConditionFlags::CARRY);
                let (result, borrow_out) = self.subtract(self.a, instruction[1], borrow_in);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                7
            }

            // SHLD (Store H & L direct)
            0x22 => {
                let address = u16::from_le_bytes([instruction[1], instruction[2]]);
                memory[address] = self.l;
                memory[address.wrapping_add(1)] = self.h;
                16
            }

            // SPHL (Move contents of HL to SP)
            0xF9 => {
                self.sp = u16::from_le_bytes([self.l, self.h]);
                5
            }

            // STA (Store A direct)
            0x32 => {
                let address = u16::from_le_bytes([instruction[1], instruction[2]]);
                memory[address] = self.a;
                13
            }

            // STAX B (Store A in address in B & C)
            0x02 => {
                let address = u16::from_le_bytes([self.c, self.b]);
                memory[address] = self.a;
                7
            }
            // STAX D (Store A in address in D & E)
            0x12 => {
                let address = u16::from_le_bytes([self.e, self.d]);
                memory[address] = self.a;
                7
            }

            // STC (Set carry flag)
            0x37 => {
                self.condition_flags.insert(ConditionFlags::CARRY);
                4
            }

            // SUB M (Subtract memory from A)
            0x96 => {
                let address = u16::from_le_bytes([self.l, self.h]);
                let (result, borrow_out) = self.subtract(self.a, memory[address], false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                7
            }

            // SUB B (Subtract B from A)
            0x90 => {
                let (result, borrow_out) = self.subtract(self.a, self.b, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SUB C (Subtract C from A)
            0x91 => {
                let (result, borrow_out) = self.subtract(self.a, self.c, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SUB D (Subtract D from A)
            0x92 => {
                let (result, borrow_out) = self.subtract(self.a, self.d, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SUB E (Subtract E from A)
            0x93 => {
                let (result, borrow_out) = self.subtract(self.a, self.e, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SUB H (Subtract H from A)
            0x94 => {
                let (result, borrow_out) = self.subtract(self.a, self.h, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SUB L (Subtract L from A)
            0x95 => {
                let (result, borrow_out) = self.subtract(self.a, self.l, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }
            // SUB A (Subtract A from A)
            0x97 => {
                let (result, borrow_out) = self.subtract(self.a, self.a, false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                4
            }

            // SUI (Subtract immediate from A)
            0xD6 => {
                let (result, borrow_out) = self.subtract(self.a, instruction[1], false);
                self.condition_flags.set(ConditionFlags::CARRY, borrow_out);
                self.a = result;
                7
            }

            // XCHG (Exchange D & E, H & L registers)
            0xEB => {
                mem::swap(&mut self.h, &mut self.d);
                mem::swap(&mut self.l, &mut self.e);
                4
            }

            // XRA M (Exclusive Or memory with A)
            0xAE => {
                let address = u16::from_le_bytes([self.l, self.h]);
                self.logical_xor(memory[address]);
                7
            }

            // XRA B (Exclusive Or B with A)
            0xA8 => {
                self.logical_xor(self.b);
                4
            }
            // XRA C (Exclusive Or C with A)
            0xA9 => {
                self.logical_xor(self.c);
                4
            }
            // XRA D (Exclusive Or D with A)
            0xAA => {
                self.logical_xor(self.d);
                4
            }
            // XRA E (Exclusive Or E with A)
            0xAB => {
                self.logical_xor(self.e);
                4
            }
            // XRA H (Exclusive Or H with A)
            0xAC => {
                self.logical_xor(self.h);
                4
            }
            // XRA L (Exclusive Or L with A)
            0xAD => {
                self.logical_xor(self.l);
                4
            }
            // XRA A (Exclusive Or A with A)
            0xAF => {
                self.logical_xor(self.a);
                4
            }

            // XRI (Exclusive Or immediate with A)
            0xEE => {
                self.logical_xor(instruction[1]);
                7
            }

            // XTHL (Exchange top of stack with HL)
            0xE3 => {
                mem::swap(&mut self.l, &mut memory[self.sp]);
                mem::swap(&mut self.h, &mut memory[self.sp.wrapping_add(1)]);
                18
            }
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

    fn call(&mut self, instruction: Instruction, memory: &mut Memory) {
        memory[self.sp.wrapping_sub(1)] = ((self.pc & 0xFF00) >> 8) as u8;
        memory[self.sp.wrapping_sub(2)] = (self.pc & 0x00FF) as u8;
        self.sp = self.sp.wrapping_sub(2);
        self.pc = u16::from_le_bytes([instruction[1], instruction[2]]);
    }

    fn logical_and(&mut self, byte: u8) {
        self.condition_flags.remove(ConditionFlags::CARRY);
        // There is a discrepancy in the behavior of the auxiliary carry flag between the manuals.
        //
        // > The CY and AC flags are cleared.
        // > (Intel 8080 Microcomputer Systems User's Manual, Rev. C, 1976, p. 4-9)
        //
        // > The 8080 logical AND instructions set the (auxiliary carry) flag to reflect the
        // > logical OR of bit 3 of the values involved in the AND operation.
        // > (Intel 8080/8085 Assembly Language Programming Manual, 1981, p. 1-12)
        //
        // The CPU test programs (8080EXER, 8080EXEM, and CPUTEST) requires the flag to behave as
        // described in "Intel 8080/8085 Assembly Language Programming Manual."
        //
        // See also https://github.com/superzazu/8080/issues/1.
        self.condition_flags.set(ConditionFlags::AUX_CARRY, ((self.a | byte) & 0x08) > 0);
        let result = self.a & byte;
        self.update_parity_zero_sign_flags(result);
        self.a = result;
    }

    // Or byte with A.
    fn logical_or(&mut self, byte: u8) {
        self.condition_flags.remove(ConditionFlags::CARRY);
        self.condition_flags.remove(ConditionFlags::AUX_CARRY);
        let result = self.a | byte;
        self.update_parity_zero_sign_flags(result);
        self.a = result;
    }

    // Exclusive Or byte with A.
    fn logical_xor(&mut self, byte: u8) {
        self.condition_flags.remove(ConditionFlags::CARRY);
        self.condition_flags.remove(ConditionFlags::AUX_CARRY);
        let result = self.a ^ byte;
        self.update_parity_zero_sign_flags(result);
        self.a = result;
    }

    fn restart(&mut self, instruction: u8, memory: &mut Memory) {
        memory[self.sp.wrapping_sub(1)] = ((self.pc & 0xFF00) >> 8) as u8;
        memory[self.sp.wrapping_sub(2)] = (self.pc & 0x00FF) as u8;
        self.sp = self.sp.wrapping_sub(2);
        self.pc = u16::from(instruction & 0x38);
    }

    fn ret(&mut self, memory: &Memory) {
        self.pc = u16::from_le_bytes([memory[self.sp], memory[self.sp.wrapping_add(1)]]);
        self.sp = self.sp.wrapping_add(2);
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

#[derive(Clone, Copy, Default)]
enum Interruptable {
    #[default]
    Disabled,
    Enabling,
    Enabled,
}

#[cfg(test)]
mod tests;
