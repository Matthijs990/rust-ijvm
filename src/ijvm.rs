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
        let instruction_result = self.get_instruction();
        match instruction_result {
            Ok(_) => {
            }
            Err(e) => {
                println!("Error fetching instruction: {}", e);
                self.halt();
                return;
            }
        }
        let instruction = instruction_result.unwrap();
        match instruction {
            op::BIPUSH => {
                if let Err(e) = self.ipush() {
                    println!("Error executing BIPUSH: {}", e);
                    self.halt();
                }
            }
            op::DUP => {
                if let Err(e) = self.dup() {
                    println!("Error executing DUP: {}", e);
                    self.halt();
                }
            }
            op::ERR => {
                if let Err(e) = self.err() {
                    println!("Error executing ERR: {}", e);
                    self.halt();
                }
            }
            op::GOTO => {
                if let Err(e) = self.goto() {
                    println!("Error executing GOTO: {}", e);
                    self.halt();
                }
            }
            op::IADD => {
                if let Err(e) = self.iadd() {
                    println!("Error executing IADD: {}", e);
                    self.halt();
                }
            }
            op::IAND => {
                if let Err(e) = self.iand() {
                    println!("Error executing IAND: {}", e);
                    self.halt();
                }
            }
            op::IFEQ => {
                if let Err(e) = self.ifeq() {
                    println!("Error executing IFEQ: {}", e);
                    self.halt();
                }
            }
            op::IFLT => {
                if let Err(e) = self.iflt() {
                    println!("Error executing IFLT: {}", e);
                    self.halt();
                }
            }
            op::IF_ICMPEQ => {
                if let Err(e) = self.if_icmpeq() {
                    println!("Error executing IF_ICMPEQ: {}", e);
                    self.halt();
                }
            }
            op::IINC => {
                if let Err(e) = self.iinc() {
                    println!("Error executing IINC: {}", e);
                    self.halt();
                }
            }
            op::ILOAD => {
                if let Err(e) = self.iload() {
                    println!("Error executing ILOAD: {}", e);
                    self.halt();
                }
            }
            op::IN => {
                if let Err(e) = self.in_command() {
                    println!("Error executing IN: {}", e);
                    self.halt();
                }
            }
            op::INVOKEVIRTUAL => {
                if let Err(e) = self.invokevirtual() {
                    println!("Error executing INVOKEVIRTUAL: {}", e);
                    self.halt();
                }
            }
            op::IOR => {
                if let Err(e) = self.ior() {
                    println!("Error executing IOR: {}", e);
                    self.halt();
                }
            }
            op::IRETURN => {
                if let Err(e) = self.ireturn() {
                    println!("Error executing IRETURN: {}", e);
                    self.halt();
                }
            }
            op::ISTORE => {
                if let Err(e) = self.istore() {
                    println!("Error executing ISTORE: {}", e);
                    self.halt();
                }
            }
            op::ISUB => {
                if let Err(e) = self.isub() {
                    println!("Error executing ISUB: {}", e);
                    self.halt();
                }
            }
            op::LDC_W => {
                if let Err(e) = self.ldc_w() {
                    println!("Error executing LDC_W: {}", e);
                    self.halt();
                }
            }
            op::NOP => {
                if let Err(e) = self.nop() {
                    println!("Error executing NOP: {}", e);
                    self.halt();
                }
            }
            op::OUT => {
                if let Err(e) = self.out() {
                    println!("Error executing OUT: {}", e);
                    self.halt();
                }
            }
            op::POP => {
                if let Err(e) = self.pop() {
                    println!("Error executing POP: {}", e);
                    self.halt();
                }
            }
            op::SWAP => {
                if let Err(e) = self.swap() {
                    println!("Error executing SWAP: {}", e);
                    self.halt();
                }
            }
            op::WIDE => {
                if let Err(e) = self.wide() {
                    println!("Error executing WIDE: {}", e);
                    self.halt();
                }
            }
            op::TAILCALL => {
                if let Err(e) = self.tailcall() {
                    println!("Error executing TAILCALL: {}", e);
                    self.halt();
                }
            }
            op::NEWARRAY => {
                if let Err(e) = self.newarray() {
                    println!("Error executing NEWARRAY: {}", e);
                    self.halt();
                }
            }
            op::IALOAD => {
                if let Err(e) = self.iaload() {
                    println!("Error executing IALOAD: {}", e);
                    self.halt();
                }
            }
            op::IASTORE => {
                if let Err(e) = self.iastore() {
                    println!("Error executing IASTORE: {}", e);
                    self.halt();
                }
            }
            op::ANEWARRAY => {
                if let Err(e) = self.anewarray() {
                    println!("Error executing ANEWARRAY: {}", e);
                    self.halt();
                }
            }
            op::AIALOAD => {
                if let Err(e) = self.aiaload() {
                    println!("Error executing AIALOAD: {}", e);
                    self.halt();
                }
            }
            op::AIASTORE => {
                if let Err(e) = self.aiastore() {
                    println!("Error executing AIASTORE: {}", e);
                    self.halt();
                }
            }
            op::GC => {
                if let Err(e) = self.gc() {
                    println!("Error executing GC: {}", e);
                    self.halt();
                }
            }
            op::NETBIND => {
                if let Err(e) = self.netbind() {
                    println!("Error executing NETBIND: {}", e);
                    self.halt();
                }
            }
            op::NETCONNECT => {
                if let Err(e) = self.netconnect() {
                    println!("Error executing NETCONNECT: {}", e);
                    self.halt();
                }
            }
            op::NETIN => {
                if let Err(e) = self.netin() {
                    println!("Error executing NETIN: {}", e);
                    self.halt();
                }
            }
            op::NETOUT => {
                if let Err(e) = self.netout() {
                    println!("Error executing NETOUT: {}", e);
                    self.halt();
                }
            }
            op::NETCLOSE => {
                if let Err(e) = self.netclose() {
                    println!("Error executing NETCLOSE: {}", e);
                    self.halt();
                }
            }
            


            op::HALT => {
                self.halt();
            
            }
            _ => {
                println!("Unknown instruction: 0x{:02X}", instruction);
                self.halt();
            }
        }
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
    pub fn get_constant(&self, i: u32) -> Result<i32, String> {
        // TODO: implement me
        Ok(0)
    }

    /// Returns the value of the program counter (as an offset from the first instruction).
    pub fn get_program_counter(&self) -> u32 {
        self.program_counter.get_pc() 
    }

    /// This function should return the word at the top of the stack of the current
    /// frame, interpreted as a signed integer.
    /// This operation should NOT pop (remove top element from stack)
    pub fn tos(&self) -> Result<i32, String> {
        // TODO: implement me
        Ok(0)
    }

    /// Returns the i:th local variable of the current frame.
    pub fn get_local_variable(&self, i: u32) -> Result<i32, String> {
        // TODO: implement me
        Ok(0)
    }

    /// Returns the value of the current instruction represented as a byte.
    /// This should NOT increase the program counter.
    pub fn get_instruction(&self) -> Result<u8, String> {
        if (self.program_counter.get_pc() as usize) < self.text.len() {
            Ok(self.text[self.program_counter.get_pc() as usize])
        } else {
            Err("Program counter out of bounds".to_string())
        }
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

    // internal methods
    fn get_uint32_at(&self, index: u32) -> Result<u32, String> {
        let start = index as usize;
        let end = start + 4;
        if end <= self.text.len() {
            let bytes: [u8; 4] = self.text[start..end].try_into().unwrap();
            Ok(u32::from_be_bytes(bytes))
        } else {
            Err("Index out of bounds".to_string())
        }
    }


    fn read_uint32(&mut self) -> Result<u32, String> {
        let pc = self.program_counter.get_pc();
        let value = self.get_uint32_at(pc)?;
        self.program_counter.increment(4);
        Ok(value)
    }

    fn read_int32(&mut self) -> Result<i32, String> {
        let pc = self.program_counter.get_pc();
        let value = self.get_uint32_at(pc)? as i32;
        self.program_counter.increment(4);
        Ok(value)
    }

    fn get_uint16_at(&self, index: u32) -> Result<u16, String> {
        let start = index as usize;
        let end = start + 2;
        if end <= self.text.len() {
            let bytes: [u8; 2] = self.text[start..end].try_into().unwrap();
            Ok(u16::from_be_bytes(bytes))
        } else {
            Err("Index out of bounds".to_string())
        }
    }
    fn read_uint16(&mut self) -> Result<u16, String> {
        let pc = self.program_counter.get_pc();
        let value = self.get_uint16_at(pc)?;
        self.program_counter.increment(2);
        Ok(value)
    }

    fn read_int16(&mut self) -> Result<i16, String> {
        let pc = self.program_counter.get_pc();
        let value = self.get_uint16_at(pc)? as i16;
        self.program_counter.increment(2);
        Ok(value)
    }


    fn read_int8(&mut self) -> Result<i8, String> {
        let pc = self.program_counter.get_pc();
        if (pc as usize) < self.text.len() {
            let value = self.text[pc as usize] as i8;
            self.program_counter.increment(1);
            Ok(value)
        } else {
            Err("Index out of bounds".to_string())
        }
    }
    fn ipush(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn dup(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn err(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn goto(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn halt(&mut self){
        
    }
    fn iadd(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn iand(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn ifeq(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn iflt(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn if_icmpeq(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn iinc(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn iload(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn in_command(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn invokevirtual(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn ior(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn ireturn(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn istore(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn isub(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn ldc_w(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn nop(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn out(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn pop(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn swap(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn wide(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn tailcall(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn newarray(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn iaload(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn iastore(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn anewarray(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn aiaload(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn aiastore(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn gc(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn netbind(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn netconnect(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn netin(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn netout(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn netclose(&mut self) -> Result<(), String> {
        Ok(())
    }
}

// NOTE: Rust handles endianness natively.
// C's swap_uint32(x) is equivalent to x.swap_bytes() or u32::from_be_bytes(...)
// C's read_uint32(buf) is equivalent to u32::from_be_bytes(buf[0..4].try_into().unwrap())
