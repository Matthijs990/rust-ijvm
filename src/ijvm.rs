use std::io::{Read, Write};
use std::fs::File;
use std::convert::TryInto;

// Constants from ijvm.h
pub const MAGIC_NUMBER: u32 = 0x1DEADFAD;

pub mod op {
    pub const BIPUSH: u8        = 0x10;
    pub const DUP: u8           = 0x59;
    pub const ERR: u8           = 0xFE;
    pub const GOTO: u8          = 0xA7;
    pub const HALT: u8          = 0xFF;
    pub const IADD: u8          = 0x60;
    pub const IAND: u8          = 0x7E;
    pub const IFEQ: u8          = 0x99;
    pub const IFLT: u8          = 0x9B;
    pub const IF_ICMPEQ: u8     = 0x9F;
    pub const IINC: u8          = 0x84;
    pub const ILOAD: u8         = 0x15;
    pub const IN: u8            = 0xFC;
    pub const INVOKEVIRTUAL: u8 = 0xB6;
    pub const IOR: u8           = 0xB0;
    pub const IRETURN: u8       = 0xAC;
    pub const ISTORE: u8        = 0x36;
    pub const ISUB: u8          = 0x64;
    pub const LDC_W: u8         = 0x13;
    pub const NOP: u8           = 0x00;
    pub const OUT: u8           = 0xFD;
    pub const POP: u8           = 0x57;
    pub const SWAP: u8          = 0x5F;
    pub const WIDE: u8          = 0xC4;
    
    // Bonus
    pub const TAILCALL: u8      = 0xCB;
    pub const NEWARRAY: u8      = 0xD1;
    pub const IALOAD: u8        = 0xD2;
    pub const IASTORE: u8       = 0xD3;
    pub const ANEWARRAY: u8     = 0xBD;
    pub const AIALOAD: u8       = 0x32;
    pub const AIASTORE: u8      = 0x53;
    pub const GC: u8            = 0xD4;
    pub const NETBIND: u8       = 0xE1;
    pub const NETCONNECT: u8    = 0xE2;
    pub const NETIN: u8         = 0xE3;
    pub const NETOUT: u8        = 0xE4;
    pub const NETCLOSE: u8      = 0xE5;
}

pub struct ProgramCounter {
    pc: u32,
    breakpoints: Vec<u32>,
    PassedBreakpoint: bool,
}

impl ProgramCounter {
    pub fn new() -> Self {
        ProgramCounter {
            pc: 0,
            breakpoints: Vec::new(),
            PassedBreakpoint: false,
        }
    }
    pub fn get_pc(&self) -> u32 {
        self.pc
    }
    pub fn set_pc(&mut self, new_pc: u32) {
        self.pc = new_pc;
    }
    pub fn add_breakpoint(&mut self, bp: u32) {
        self.breakpoints.push(bp);
    }
    pub fn remove_breakpoint(&mut self, bp: u32) {
        self.breakpoints.retain(|&x| x != bp);
    }
    pub fn is_breakpoint(&self, pc: u32) -> bool {
        self.breakpoints.contains(&pc)
    }
    pub fn clear_breakpoints(&mut self) {
        self.breakpoints.clear();
    }
    pub fn increment(&mut self, offset: u32) {
        // check if any breakpoint is passed with loop from current pc to new pc
        self.PassedBreakpoint = false;
        for bp in &self.breakpoints {
            if *bp > self.pc && *bp <= self.pc + offset {
                self.PassedBreakpoint = true;
            }
        }
        self.pc += offset;
    }
    pub fn has_passed_breakpoint(&self) -> bool {
        self.PassedBreakpoint
    }
}

pub struct Ijvm {
    // Input/Output streams (equivalent to FILE* in and out)
    input: Box<dyn Read>,
    output: Box<dyn Write>,

    // TODO: Add your variables here (internal state)
    // program_counter: u32,
    // stack: Vec<i32>,
    // text: Vec<u8>,
    program_counter: ProgramCounter,
    stack: Vec<i32>,
    text: Vec<u8>,
}

impl Ijvm {
    /// Initializes the IJVM with the binary file found at the provided argument.
    /// input gives the stream where the ijvm reads from for the IN command
    /// output gives the stream where the ijvm writes to for the OUT command
    pub fn new(binary_path: &str, input: Box<dyn Read>, output: Box<dyn Write>) -> Result<Self, std::io::Error> {
        // Read the binary file
        let mut file = File::open(binary_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // TODO: specific initialization logic using the buffer

        Ok(Ijvm {
            input,
            output,
            program_counter: ProgramCounter::new(),
            stack: Vec::new(),
            text: buffer,
            // Initialize your fields here
        })
    }

    /// Run the vm with the current state until the machine halts.
    pub fn run(&mut self) {
        while !self.finished() {
            self.step();
        }
    }

    /// Step (perform) one instruction.
    pub fn step(&mut self) {
        // TODO: implement me
    }

    /// Check whether the machine has any more instructions to execute.
    pub fn finished(&self) -> bool {
        // TODO: implement me
        false
    }

    /// Returns the currently loaded program text.
    pub fn get_text(&self) -> &[u8] {
        // TODO: implement me
        &[]
    }

    /// Returns the size of the currently loaded program text.
    pub fn get_text_size(&self) -> u32 {
        // TODO: implement me
        0
    }

    /// Returns the constant at location i in the constant pool.
    pub fn get_constant(&self, i: u32) -> i32 {
        // TODO: implement me
        0
    }

    /// Returns the value of the program counter (as an offset from the first instruction).
    pub fn get_program_counter(&self) -> u32 {
        // TODO: implement me
        0
    }

    /// This function should return the word at the top of the stack of the current
    /// frame, interpreted as a signed integer.
    /// This operation should NOT pop (remove top element from stack)
    pub fn tos(&self) -> i32 {
        // TODO: implement me
        -1
    }

    /// Returns the i:th local variable of the current frame.
    pub fn get_local_variable(&self, i: u32) -> i32 {
        // TODO: implement me
        0
    }

    /// Returns the value of the current instruction represented as a byte.
    /// This should NOT increase the program counter.
    pub fn get_instruction(&self) -> u8 {
        // TODO: implement me
        0
    }

    // Bonus Methods
    
    pub fn get_call_stack_size(&self) -> u32 {
        // TODO: implement me if doing tail call bonus
        0
    }

    pub fn is_heap_freed(&self, reference: i32) -> bool {
        // TODO: implement me if doing garbage collection bonus
        false
    }

    pub fn is_tos_reference(&self) -> bool {
        // TODO: implement me if doing precise garbage collection bonus
        false
    }
}

// NOTE: Rust handles endianness natively.
// C's swap_uint32(x) is equivalent to x.swap_bytes() or u32::from_be_bytes(...)
// C's read_uint32(buf) is equivalent to u32::from_be_bytes(buf[0..4].try_into().unwrap())
