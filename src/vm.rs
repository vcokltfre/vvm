use crate::{Instruction, Program};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Value {
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::UInt(a), Value::UInt(b)) => Ok(Value::UInt(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            _ => Err("Type mismatch in ADD operation".to_string()),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::UInt(a), Value::UInt(b)) => Ok(Value::UInt(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            _ => Err("Type mismatch in SUB operation".to_string()),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::UInt(a), Value::UInt(b)) => Ok(Value::UInt(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            _ => Err("Type mismatch in MUL operation".to_string()),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::UInt(a), Value::UInt(b)) => {
                if *b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::UInt(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            _ => Err("Type mismatch in DIV operation".to_string()),
        }
    }

    pub fn mod_op(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            (Value::UInt(a), Value::UInt(b)) => {
                if *b == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::UInt(a % b))
                }
            }
            _ => Err("Type mismatch in MOD operation".to_string()),
        }
    }

    pub fn exp(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.pow(*b as u32))),
            (Value::UInt(a), Value::UInt(b)) => Ok(Value::UInt(a.pow(*b as u32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            _ => Err("Type mismatch in EXP operation".to_string()),
        }
    }
}

pub struct VM {
    program: Program,
    ptr: usize,
    call_stack: Vec<usize>,
    data_stack: Vec<Value>,
    memory: std::collections::HashMap<String, Value>,
    native_handlers: std::collections::HashMap<String, fn(&mut VM)>,
}

impl VM {
    pub fn new(program: Program) -> Self {
        VM {
            program,
            ptr: 0,
            call_stack: Vec::new(),
            data_stack: Vec::new(),
            memory: std::collections::HashMap::new(),
            native_handlers: std::collections::HashMap::new(),
        }
    }

    pub fn pop(&mut self) -> Value {
        match self.data_stack.pop() {
            Some(v) => v,
            None => {
                eprintln!("Error: Stack underflow");
                std::process::exit(1);
            }
        }
    }

    pub fn push(&mut self, value: Value) {
        self.data_stack.push(value);
    }

    pub fn dup(&mut self) {
        if let Some(top) = self.data_stack.last() {
            self.data_stack.push(top.clone());
        } else {
            eprintln!("Error: Stack underflow on DUP");
            std::process::exit(1);
        }
    }

    pub fn swap(&mut self) {
        if self.data_stack.len() < 2 {
            eprintln!("Error: Stack underflow on SWAP");
            std::process::exit(1);
        }
        let len = self.data_stack.len();
        self.data_stack.swap(len - 1, len - 2);
    }

    pub fn set_memory(&mut self, name: &str, value: Value) {
        self.memory.insert(name.to_string(), value);
    }

    pub fn get_memory(&self, name: &str) -> Option<&Value> {
        self.memory.get(name)
    }

    pub fn free_memory(&mut self, name: &str) {
        self.memory.remove(name);
    }

    pub fn register_native_handler(&mut self, name: &str, handler: fn(&mut VM)) {
        self.native_handlers.insert(name.to_string(), handler);
    }

    pub fn run(&mut self) {
        while self.ptr < self.program.instructions.len() {
            let instr = &self.program.instructions[self.ptr].clone();
            match instr {
                Instruction::Exit => match self.pop() {
                    Value::Int(code) => std::process::exit(code as i32),
                    Value::UInt(code) => std::process::exit(code as i32),
                    _ => {
                        eprintln!("Error: EXIT expects an integer return code");
                        std::process::exit(1);
                    }
                },
                Instruction::ExitImmediate(code) => {
                    std::process::exit(*code as i32);
                }

                Instruction::PushInt(v) => {
                    self.push(Value::Int(*v));
                }
                Instruction::PushUInt(v) => {
                    self.push(Value::UInt(*v));
                }
                Instruction::PushFloat(v) => {
                    self.push(Value::Float(*v));
                }
                Instruction::PushBool(v) => {
                    self.push(Value::Bool(*v));
                }
                Instruction::PushString(s) => {
                    self.push(Value::String(s.clone()));
                }
                Instruction::Pop => {
                    self.pop();
                }
                Instruction::Dup => {
                    self.dup();
                }
                Instruction::Swap => {
                    self.swap();
                }

                Instruction::Add => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    match lhs.add(&rhs) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::AddI(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.add(&Value::Int(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::AddU(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.add(&Value::UInt(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::AddF(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.add(&Value::Float(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::Sub => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    match lhs.sub(&rhs) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::SubI(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.sub(&Value::Int(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::SubU(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.sub(&Value::UInt(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::SubF(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.sub(&Value::Float(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::Mul => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    match lhs.mul(&rhs) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::MulI(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.mul(&Value::Int(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::MulU(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.mul(&Value::UInt(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::MulF(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.mul(&Value::Float(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::Div => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    match lhs.div(&rhs) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::DivI(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.div(&Value::Int(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::DivU(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.div(&Value::UInt(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::DivF(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.div(&Value::Float(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::Mod => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    match lhs.mod_op(&rhs) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::ModI(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.mod_op(&Value::Int(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::ModU(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.mod_op(&Value::UInt(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::Exp => {
                    let rhs = self.pop();
                    let lhs = self.pop();

                    match lhs.exp(&rhs) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::ExpI(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.exp(&Value::Int(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::ExpU(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.exp(&Value::UInt(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::ExpF(v) => {
                    let lhs = self.pop();
                    let rhs = v.clone();

                    match lhs.exp(&Value::Float(rhs)) {
                        Ok(result) => self.push(result),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }

                Instruction::Load => {
                    let loc = self.pop();
                    match loc {
                        Value::String(v) => match self.get_memory(&v) {
                            Some(val) => self.push(val.clone()),
                            None => {
                                eprintln!("Error: Undefined variable '{}'", v);
                                std::process::exit(1);
                            }
                        },
                        _ => {
                            eprintln!("Error: LOAD expects a string as variable name");
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::LoadImm(name) => match self.get_memory(name) {
                    Some(val) => self.push(val.clone()),
                    None => {
                        eprintln!("Error: Undefined variable '{}'", name);
                        std::process::exit(1);
                    }
                },
                Instruction::Store => {
                    let val = self.pop();
                    let loc = self.pop();
                    match loc {
                        Value::String(v) => {
                            self.set_memory(&v, val);
                        }
                        _ => {
                            eprintln!("Error: STORE expects a string as variable name");
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::StoreImm(name) => {
                    let val = self.pop();
                    self.set_memory(name, val);
                }
                Instruction::Free => {
                    let loc = self.pop();
                    match loc {
                        Value::String(v) => {
                            self.free_memory(&v);
                        }
                        _ => {
                            eprintln!("Error: FREE expects a string as variable name");
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::FreeImm(name) => {
                    self.free_memory(name);
                }

                Instruction::CmpEqual => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(Value::Bool(lhs == rhs));
                }
                Instruction::CmpNotEqual => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(Value::Bool(lhs != rhs));
                }
                Instruction::CmpGreaterThan => {
                    let rhs = self.pop();
                    let lhs = self.pop();

                    match (lhs, rhs) {
                        (Value::Int(a), Value::Int(b)) => self.push(Value::Bool(a > b)),
                        (Value::UInt(a), Value::UInt(b)) => self.push(Value::Bool(a > b)),
                        (Value::Float(a), Value::Float(b)) => self.push(Value::Bool(a > b)),
                        _ => {
                            eprintln!("Error: Type mismatch in CMP_GREATER_THAN");
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::CmpLessThan => {
                    let rhs = self.pop();
                    let lhs = self.pop();

                    match (lhs, rhs) {
                        (Value::Int(a), Value::Int(b)) => self.push(Value::Bool(a < b)),
                        (Value::UInt(a), Value::UInt(b)) => self.push(Value::Bool(a < b)),
                        (Value::Float(a), Value::Float(b)) => self.push(Value::Bool(a < b)),
                        _ => {
                            eprintln!("Error: Type mismatch in CMP_LESS_THAN");
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::CmpGreaterEqual => {
                    let rhs = self.pop();
                    let lhs = self.pop();

                    match (lhs, rhs) {
                        (Value::Int(a), Value::Int(b)) => self.push(Value::Bool(a >= b)),
                        (Value::UInt(a), Value::UInt(b)) => self.push(Value::Bool(a >= b)),
                        (Value::Float(a), Value::Float(b)) => self.push(Value::Bool(a >= b)),
                        _ => {
                            eprintln!("Error: Type mismatch in CMP_GREATER_EQUAL");
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::CmpLessEqual => {
                    let rhs = self.pop();
                    let lhs = self.pop();

                    match (lhs, rhs) {
                        (Value::Int(a), Value::Int(b)) => self.push(Value::Bool(a <= b)),
                        (Value::UInt(a), Value::UInt(b)) => self.push(Value::Bool(a <= b)),
                        (Value::Float(a), Value::Float(b)) => self.push(Value::Bool(a <= b)),
                        _ => {
                            eprintln!("Error: Type mismatch in CMP_LESS_EQUAL");
                            std::process::exit(1);
                        }
                    }
                }

                Instruction::Jump(label) => {
                    if let Some(&target) = self.program.labels.get(label) {
                        self.ptr = target;
                        continue;
                    } else {
                        eprintln!("Error: Undefined label '{}'", label);
                        std::process::exit(1);
                    }
                }
                Instruction::JumpIf(label) => {
                    let cond = self.pop();
                    match cond {
                        Value::Bool(true) => {
                            if let Some(&target) = self.program.labels.get(label) {
                                self.ptr = target;
                                continue;
                            } else {
                                eprintln!("Error: Undefined label '{}'", label);
                                std::process::exit(1);
                            }
                        }
                        Value::Bool(false) => {
                            // Do nothing, just continue
                        }
                        _ => {
                            eprintln!("Error: JMPIF expects a boolean condition");
                            std::process::exit(1);
                        }
                    }
                }
                Instruction::Call(label) => {
                    if let Some(&target) = self.program.labels.get(label) {
                        self.call_stack.push(self.ptr + 1);
                        self.ptr = target;
                        continue;
                    } else {
                        eprintln!("Error: Undefined label '{}'", label);
                        std::process::exit(1);
                    }
                }
                Instruction::CallNative(name) => {
                    if let Some(&handler) = self.native_handlers.get(name) {
                        handler(self);
                    } else {
                        eprintln!("Error: Undefined native handler '{}'", name);
                        std::process::exit(1);
                    }
                    self.ptr += 1;
                    continue;
                }
                Instruction::Ret => {
                    if let Some(return_addr) = self.call_stack.pop() {
                        self.ptr = return_addr;
                        continue;
                    } else {
                        eprintln!("Error: Call stack underflow on RET");
                        std::process::exit(1);
                    }
                }
            }

            self.ptr += 1;
        }
    }
}
