use std::collections::HashMap;
use std::fmt::Display;

use crate::opcodes::*;
use crate::parse;

#[derive(Debug, Clone)]
pub enum Instruction {
    Exit,              // 0x00 EXIT
    ExitImmediate(u8), // 0x01 EXIT [imm]

    PushInt(i64),       // 0x10 PUSHI [imm]
    PushUInt(u64),      // 0x11 PUSHU [imm]
    PushFloat(f64),     // 0x12 PUSHF [imm]
    PushBool(bool),     // 0x13 PUSHB [imm]
    PushString(String), // 0x14 PUSHS [imm]
    Pop,                // 0x15 POP
    Dup,                // 0x16 DUP
    Swap,               // 0x17 SWAP

    Add,       // 0x20 ADD
    AddI(i64), // 0x21 ADDI [imm]
    AddU(u64), // 0x22 ADDU [imm]
    AddF(f64), // 0x23 ADDF [imm]
    Sub,       // 0x24 SUB
    SubI(i64), // 0x25 SUBI [imm]
    SubU(u64), // 0x26 SUBU [imm]
    SubF(f64), // 0x27 SUBF [imm]
    Mul,       // 0x28 MUL
    MulI(i64), // 0x29 MULI [imm]
    MulU(u64), // 0x2a MULU [imm]
    MulF(f64), // 0x2b MULF [imm]
    Div,       // 0x2c DIV
    DivI(i64), // 0x2d DIVI [imm]
    DivU(u64), // 0x2e DIVU [imm]
    DivF(f64), // 0x2f DIVF [imm]
    Mod,       // 0x30 MOD
    ModI(i64), // 0x31 MODI [imm]
    ModU(u64), // 0x32 MODU [imm]
    Exp,       // 0x33 EXP
    ExpI(i64), // 0x34 EXPI [imm]
    ExpU(u64), // 0x35 EXPU [imm]
    ExpF(f64), // 0x36 EXPF [imm]

    Load,             // 0x40 LOAD
    LoadImm(String),  // 0x41 LOAD [imm]
    Store,            // 0x42 STORE
    StoreImm(String), // 0x43 STORE [imm]
    Free,             // 0x44 FREE
    FreeImm(String),  // 0x45 FREE [imm]

    CmpEqual,        // 0x50 CMPEQ
    CmpNotEqual,     // 0x51 CMPNE
    CmpGreaterThan,  // 0x52 CMPGT
    CmpLessThan,     // 0x53 CMPLT
    CmpGreaterEqual, // 0x54 CMPGE
    CmpLessEqual,    // 0x55 CMPLE

    Jump(String),       // 0x60 JMP [label]
    JumpIf(String),     // 0x61 JMPIF [label]
    Call(String),       // 0x62 CALL [label]
    CallNative(String), // 0x63 CALLNATIVE [label]
    Ret,                // 0x64 RET
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Exit => write!(f, "EXIT"),
            Instruction::ExitImmediate(imm) => write!(f, "EXIT {}", imm),

            Instruction::PushInt(imm) => write!(f, "PUSHI {}", imm),
            Instruction::PushUInt(imm) => write!(f, "PUSHU {}", imm),
            Instruction::PushFloat(imm) => write!(f, "PUSHF {}", imm),
            Instruction::PushBool(imm) => write!(f, "PUSHB {}", imm),
            Instruction::PushString(imm) => write!(f, "PUSHS \"{}\"", imm),
            Instruction::Pop => write!(f, "POP"),
            Instruction::Dup => write!(f, "DUP"),
            Instruction::Swap => write!(f, "SWAP"),

            Instruction::Add => write!(f, "ADD"),
            Instruction::AddI(imm) => write!(f, "ADDI {}", imm),
            Instruction::AddU(imm) => write!(f, "ADDU {}", imm),
            Instruction::AddF(imm) => write!(f, "ADDF {}", imm),
            Instruction::Sub => write!(f, "SUB"),
            Instruction::SubI(imm) => write!(f, "SUBI {}", imm),
            Instruction::SubU(imm) => write!(f, "SUBU {}", imm),
            Instruction::SubF(imm) => write!(f, "SUBF {}", imm),
            Instruction::Mul => write!(f, "MUL"),
            Instruction::MulI(imm) => write!(f, "MULI {}", imm),
            Instruction::MulU(imm) => write!(f, "MULU {}", imm),
            Instruction::MulF(imm) => write!(f, "MULF {}", imm),
            Instruction::Div => write!(f, "DIV"),
            Instruction::DivI(imm) => write!(f, "DIVI {}", imm),
            Instruction::DivU(imm) => write!(f, "DIVU {}", imm),
            Instruction::DivF(imm) => write!(f, "DIVF {}", imm),
            Instruction::Mod => write!(f, "MOD"),
            Instruction::ModI(imm) => write!(f, "MODI {}", imm),
            Instruction::ModU(imm) => write!(f, "MODU {}", imm),
            Instruction::Exp => write!(f, "EXP"),
            Instruction::ExpI(imm) => write!(f, "EXPI {}", imm),
            Instruction::ExpU(imm) => write!(f, "EXPU {}", imm),
            Instruction::ExpF(imm) => write!(f, "EXPF {}", imm),
            Instruction::Load => write!(f, "LOAD"),
            Instruction::LoadImm(imm) => write!(f, "LOAD {}", imm),
            Instruction::Store => write!(f, "STORE"),
            Instruction::StoreImm(imm) => write!(f, "STORE {}", imm),
            Instruction::Free => write!(f, "FREE"),
            Instruction::FreeImm(imm) => write!(f, "FREE {}", imm),
            Instruction::CmpEqual => write!(f, "CMPEQ"),
            Instruction::CmpNotEqual => write!(f, "CMPNE"),
            Instruction::CmpGreaterThan => write!(f, "CMPGT"),
            Instruction::CmpLessThan => write!(f, "CMPLT"),
            Instruction::CmpGreaterEqual => write!(f, "CMPGE"),
            Instruction::CmpLessEqual => write!(f, "CMPLE"),
            Instruction::Jump(label) => write!(f, "JMP {}", label),
            Instruction::JumpIf(label) => write!(f, "JMPIF {}", label),
            Instruction::Call(label) => write!(f, "CALL {}", label),
            Instruction::CallNative(label) => write!(f, "CALLNATIVE {}", label),
            Instruction::Ret => write!(f, "RET"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<String, usize>,
}

impl Program {
    pub fn from_bytecode(source: Vec<u8>) -> Result<Self, String> {
        let mut instructions = Vec::new();
        let mut labels = HashMap::new();

        let mut index = 0;
        while index < source.len() {
            let opcode = source[index];
            index += 1;

            match opcode {
                OP_EXIT => instructions.push(Instruction::Exit),
                OP_EXIT_IMMEDIATE => {
                    let imm = source[index];
                    index += 1;
                    instructions.push(Instruction::ExitImmediate(imm));
                }

                OP_PUSH_INT => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = i64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::PushInt(imm));
                }
                OP_PUSH_UINT => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = u64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::PushUInt(imm));
                }
                OP_PUSH_FLOAT => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = f64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::PushFloat(imm));
                }
                OP_PUSH_BOOL => {
                    let imm = source[index] != 0;
                    index += 1;
                    instructions.push(Instruction::PushBool(imm));
                }
                OP_PUSH_STRING => {
                    let len_bytes = &source[index..index + 4];
                    let len = u32::from_le_bytes(len_bytes.try_into().unwrap()) as usize;
                    index += 4;
                    let str_bytes = &source[index..index + len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += len;
                    instructions.push(Instruction::PushString(imm));
                }
                OP_POP => instructions.push(Instruction::Pop),
                OP_DUP => instructions.push(Instruction::Dup),
                OP_SWAP => instructions.push(Instruction::Swap),

                OP_ADD => instructions.push(Instruction::Add),
                OP_ADD_I => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = i64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::AddI(imm));
                }
                OP_ADD_U => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = u64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::AddU(imm));
                }
                OP_ADD_F => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = f64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::AddF(imm));
                }
                OP_SUB => instructions.push(Instruction::Sub),
                OP_SUB_I => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = i64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::SubI(imm));
                }
                OP_SUB_U => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = u64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::SubU(imm));
                }
                OP_SUB_F => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = f64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::SubF(imm));
                }
                OP_MUL => instructions.push(Instruction::Mul),
                OP_MUL_I => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = i64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::MulI(imm));
                }
                OP_MUL_U => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = u64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::MulU(imm));
                }
                OP_MUL_F => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = f64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::MulF(imm));
                }
                OP_DIV => instructions.push(Instruction::Div),
                OP_DIV_I => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = i64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::DivI(imm));
                }
                OP_DIV_U => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = u64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::DivU(imm));
                }
                OP_DIV_F => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = f64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::DivF(imm));
                }
                OP_MOD => instructions.push(Instruction::Mod),
                OP_MOD_I => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = i64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::ModI(imm));
                }
                OP_MOD_U => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = u64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::ModU(imm));
                }
                OP_EXP => instructions.push(Instruction::Exp),
                OP_EXP_I => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = i64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::ExpI(imm));
                }
                OP_EXP_U => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = u64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::ExpU(imm));
                }
                OP_EXP_F => {
                    let imm_bytes = &source[index..index + 8];
                    let imm = f64::from_le_bytes(imm_bytes.try_into().unwrap());
                    index += 8;
                    instructions.push(Instruction::ExpF(imm));
                }

                OP_LOAD => instructions.push(Instruction::Load),
                OP_LOAD_IMM => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    instructions.push(Instruction::LoadImm(imm));
                }
                OP_STORE => instructions.push(Instruction::Store),
                OP_STORE_IMM => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    instructions.push(Instruction::StoreImm(imm));
                }
                OP_FREE => instructions.push(Instruction::Free),
                OP_FREE_IMM => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    instructions.push(Instruction::FreeImm(imm));
                }

                OP_CMP_EQUAL => instructions.push(Instruction::CmpEqual),
                OP_CMP_NOT_EQUAL => instructions.push(Instruction::CmpNotEqual),
                OP_CMP_GREATER_THAN => instructions.push(Instruction::CmpGreaterThan),
                OP_CMP_LESS_THAN => instructions.push(Instruction::CmpLessThan),
                OP_CMP_GREATER_EQUAL => instructions.push(Instruction::CmpGreaterEqual),
                OP_CMP_LESS_EQUAL => instructions.push(Instruction::CmpLessEqual),

                OP_JUMP => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    instructions.push(Instruction::Jump(imm));
                }
                OP_JUMP_IF => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    instructions.push(Instruction::JumpIf(imm));
                }
                OP_CALL => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    instructions.push(Instruction::Call(imm));
                }
                OP_CALL_NATIVE => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let imm = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    instructions.push(Instruction::CallNative(imm));
                }
                OP_RET => instructions.push(Instruction::Ret),

                OP_LABEL => {
                    let str_len = source[index] as usize;
                    index += 1;
                    let str_bytes = &source[index..index + str_len];
                    let label = String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())?;
                    index += str_len;
                    labels.insert(label, instructions.len());
                }

                _ => {
                    return Err(format!("Unknown opcode: 0x{:02X}", opcode));
                }
            }
        }

        Ok(Program {
            instructions: instructions,
            labels: labels,
        })
    }

    pub fn from_source(source: &str) -> Result<Self, String> {
        let bytecode = parse(source)?;

        return Self::from_bytecode(bytecode);
    }
}
