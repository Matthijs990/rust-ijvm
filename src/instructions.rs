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
    fn halt(&mut self) {
        self.is_running = false;
    }

}
