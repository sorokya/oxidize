use mem::Mem;
use util::Save;

use std::fs::File;
use std::ops::Deref;

#[cfg(cpuspew)]
use disasm::Disassembler;

const CARRY_FLAG:    u8 = 1 << 0;
const ZERO_FLAG:     u8 = 1 << 1;
const IRQ_FLAG:      u8 = 1 << 2;
const DECIMAL_FLAG:  u8 = 1 << 3;
const BREAK_FLAG:    u8 = 1 << 4;
const OVERFLOW_FLAG: u8 = 1 << 6;
const NEGATIVE_FLAG: u8 = 1 << 7;

const NMI_VECTOR:   u16 = 0xfffa;
const RESET_VECTOR: u16 = 0xfffc;
const BRK_VECTOR:   u16 = 0xfffe;

/// The number of cycles that each machine operation takes. Indexed by opcode number.
///
/// FIXME: This is copied from FCEU.
static CYCLE_TABLE: [u8; 256] = [
	/*0x00*/ 7,6,2,8,3,3,5,5,3,2,2,2,4,4,6,6,
	/*0x10*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
	/*0x20*/ 6,6,2,8,3,3,5,5,4,2,2,2,4,4,6,6,
	/*0x30*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
	/*0x40*/ 6,6,2,8,3,3,5,5,3,2,2,2,3,4,6,6,
	/*0x50*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
	/*0x60*/ 6,6,2,8,3,3,5,5,4,2,2,2,5,4,6,6,
	/*0x70*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
	/*0x80*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
	/*0x90*/ 2,6,2,6,4,4,4,4,2,5,2,5,5,5,5,5,
	/*0xA0*/ 2,6,2,6,3,3,3,3,2,2,2,2,4,4,4,4,
	/*0xB0*/ 2,5,2,5,4,4,4,4,2,4,2,4,4,4,4,4,
	/*0xC0*/ 2,6,2,8,3,3,5,5,2,2,2,2,4,4,6,6,
	/*0xD0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
	/*0xE0*/ 2,6,3,8,3,3,5,5,2,2,2,2,4,4,6,6,
	/*0xF0*/ 2,5,2,8,4,4,6,6,2,4,2,7,4,4,7,7,
];

/// CPU Registers
struct Regs {
	a: u8,
	x: u8,
	y: u8,
	s: u8,
	flags: u8,
	pc: u16
}