use crate::ijvm::Ijvm;

/// This trait defines the instruction set as an extension to Ijvm.
pub trait Instructions {
    fn ipush(&mut self) -> Result<(), String>;
    fn dup(&mut self) -> Result<(), String>;
    fn err(&mut self) -> Result<(), String>;
    fn goto(&mut self) -> Result<(), String>;
    fn iadd(&mut self) -> Result<(), String>;
    fn iand(&mut self) -> Result<(), String>;
    fn ifeq(&mut self) -> Result<(), String>;
    fn iflt(&mut self) -> Result<(), String>;
    fn if_icmpeq(&mut self) -> Result<(), String>;
    fn iinc(&mut self) -> Result<(), String>;
    fn iload(&mut self) -> Result<(), String>;
    fn in_command(&mut self) -> Result<(), String>;
    fn invokevirtual(&mut self) -> Result<(), String>;
    fn ior(&mut self) -> Result<(), String>;
    fn ireturn(&mut self) -> Result<(), String>;
    fn istore(&mut self) -> Result<(), String>;
    fn isub(&mut self) -> Result<(), String>;
    fn ldc_w(&mut self) -> Result<(), String>;
    fn nop(&mut self) -> Result<(), String>;
    fn out(&mut self) -> Result<(), String>;
    fn pop(&mut self) -> Result<(), String>;
    fn swap(&mut self) -> Result<(), String>;
    fn wide(&mut self) -> Result<(), String>;
    fn halt(&mut self);
    
    // Bonus
    fn tailcall(&mut self) -> Result<(), String>;
    fn newarray(&mut self) -> Result<(), String>;
    fn iaload(&mut self) -> Result<(), String>;
    fn iastore(&mut self) -> Result<(), String>;
    fn anewarray(&mut self) -> Result<(), String>;
    fn aiaload(&mut self) -> Result<(), String>;
    fn aiastore(&mut self) -> Result<(), String>;
    fn gc(&mut self) -> Result<(), String>;
    fn netbind(&mut self) -> Result<(), String>;
    fn netconnect(&mut self) -> Result<(), String>;
    fn netin(&mut self) -> Result<(), String>;
    fn netout(&mut self) -> Result<(), String>;
    fn netclose(&mut self) -> Result<(), String>;
}

/// Implement the trait for the Ijvm struct.
/// This allows us to use `self.ipush()`, `self.stack.push()`, etc.
impl Instructions for Ijvm {
    fn ipush(&mut self) -> Result<(), String> {
        // Example implementation accessing internal helpers:
        // let byte = self.read_int8()?;
        // self.stack.push(byte as i32);
        let byte = self.read_int8()
            .map_err(|e| format!("ipush: failed to read byte: {}", e))?;
        self.stack.push(byte as i32);
        Ok(())
        
    }

    fn dup(&mut self) -> Result<(), String> {
        let value = *self.stack.last()
            .ok_or("dup: stack underflow".to_string())?;
        self.stack.push(value);
        Ok(())
    }

    fn err(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn goto(&mut self) -> Result<(), String> {
        Err("not implemented".to_string()) 
    }

    fn iadd(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn iand(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn ifeq(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn iflt(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn if_icmpeq(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn iinc(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn iload(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn in_command(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn invokevirtual(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn ior(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn ireturn(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn istore(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn isub(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn ldc_w(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn nop(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn out(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn pop(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn swap(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn wide(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    // Bonus Instructions

    fn tailcall(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn newarray(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn iaload(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn iastore(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn anewarray(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn aiaload(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn aiastore(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn gc(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn netbind(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn netconnect(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn netin(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn netout(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }

    fn netclose(&mut self) -> Result<(), String> {
        Err("not implemented".to_string())
    }
    fn halt(&mut self) {
        self.is_running = false;
    }

}
