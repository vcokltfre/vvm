use crate::opcodes::*;

pub fn parse(source: &str) -> Result<Vec<u8>, String> {
    let lines = source.lines();
    let mut bytecode = Vec::new();

    for line in lines {
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        let line = line.trim();

        let (op, val) = match line.split_once(" ") {
            Some((o, v)) => (o.trim(), v.trim()),
            None => (line.trim(), ""),
        };

        match op {
            "EXIT" => bytecode.push(OP_EXIT),
            "EXIT_IMM" => {
                bytecode.push(OP_EXIT_IMMEDIATE);
                let imm: u8 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.push(imm);
            }

            "PUSHI" => {
                bytecode.push(OP_PUSH_INT);
                let imm: i64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "PUSHU" => {
                bytecode.push(OP_PUSH_UINT);
                let imm: u64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "PUSHF" => {
                bytecode.push(OP_PUSH_FLOAT);
                let imm: f64 = val
                    .parse()
                    .map_err(|e: std::num::ParseFloatError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "PUSHB" => {
                bytecode.push(OP_PUSH_BOOL);
                let imm: bool = val
                    .parse()
                    .map_err(|e: std::str::ParseBoolError| e.to_string())?;
                bytecode.push(if imm { 1 } else { 0 });
            }
            "PUSHS" => {
                bytecode.push(OP_PUSH_STRING);
                let val = val.replace("\\n", "\n");
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u32;
                bytecode.extend_from_slice(&len.to_le_bytes());
                bytecode.extend_from_slice(imm_bytes);
            }
            "POP" => bytecode.push(OP_POP),
            "DUP" => bytecode.push(OP_DUP),
            "SWAP" => bytecode.push(OP_SWAP),

            "ADD" => bytecode.push(OP_ADD),
            "ADDI" => {
                bytecode.push(OP_ADD_I);
                let imm: i64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "ADDU" => {
                bytecode.push(OP_ADD_U);
                let imm: u64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "ADDF" => {
                bytecode.push(OP_ADD_F);
                let imm: f64 = val
                    .parse()
                    .map_err(|e: std::num::ParseFloatError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "SUB" => bytecode.push(OP_SUB),
            "SUBI" => {
                bytecode.push(OP_SUB_I);
                let imm: i64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "SUBU" => {
                bytecode.push(OP_SUB_U);
                let imm: u64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "SUBF" => {
                bytecode.push(OP_SUB_F);
                let imm: f64 = val
                    .parse()
                    .map_err(|e: std::num::ParseFloatError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "MUL" => bytecode.push(OP_MUL),
            "MULI" => {
                bytecode.push(OP_MUL_I);
                let imm: i64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "MULU" => {
                bytecode.push(OP_MUL_U);
                let imm: u64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "MULF" => {
                bytecode.push(OP_MUL_F);
                let imm: f64 = val
                    .parse()
                    .map_err(|e: std::num::ParseFloatError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "DIV" => bytecode.push(OP_DIV),
            "DIVI" => {
                bytecode.push(OP_DIV_I);
                let imm: i64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "DIVU" => {
                bytecode.push(OP_DIV_U);
                let imm: u64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "DIVF" => {
                bytecode.push(OP_DIV_F);
                let imm: f64 = val
                    .parse()
                    .map_err(|e: std::num::ParseFloatError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "MOD" => bytecode.push(OP_MOD),
            "MODI" => {
                bytecode.push(OP_MOD_I);
                let imm: i64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "MODU" => {
                bytecode.push(OP_MOD_U);
                let imm: u64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "EXPI" => {
                bytecode.push(OP_EXP_I);
                let imm: i64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "EXPU" => {
                bytecode.push(OP_EXP_U);
                let imm: u64 = val
                    .parse()
                    .map_err(|e: std::num::ParseIntError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }
            "EXPF" => {
                bytecode.push(OP_EXP_F);
                let imm: f64 = val
                    .parse()
                    .map_err(|e: std::num::ParseFloatError| e.to_string())?;
                bytecode.extend_from_slice(&imm.to_le_bytes());
            }

            "LOAD" => bytecode.push(OP_LOAD),
            "LOAD_IMM" => {
                bytecode.push(OP_LOAD_IMM);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }
            "STORE" => bytecode.push(OP_STORE),
            "STORE_IMM" => {
                bytecode.push(OP_STORE_IMM);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }
            "FREE" => bytecode.push(OP_FREE),
            "FREE_IMM" => {
                bytecode.push(OP_FREE_IMM);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }

            "CMPEQ" => bytecode.push(OP_CMP_EQUAL),
            "CMPNE" => bytecode.push(OP_CMP_NOT_EQUAL),
            "CMPGT" => bytecode.push(OP_CMP_GREATER_THAN),
            "CMPLT" => bytecode.push(OP_CMP_LESS_THAN),
            "CMPGE" => bytecode.push(OP_CMP_GREATER_EQUAL),
            "CMPLE" => bytecode.push(OP_CMP_LESS_EQUAL),

            "JMP" => {
                bytecode.push(OP_JUMP);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }
            "JMPIF" => {
                bytecode.push(OP_JUMP_IF);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }
            "CALL" => {
                bytecode.push(OP_CALL);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }
            "CALLNATIVE" => {
                bytecode.push(OP_CALL_NATIVE);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }
            "RET" => bytecode.push(OP_RET),
            "LABEL" => {
                bytecode.push(OP_LABEL);
                let imm_bytes = val.as_bytes();
                let len = imm_bytes.len() as u8;
                bytecode.push(len);
                bytecode.extend_from_slice(imm_bytes);
            }

            _ => {
                return Err(format!("Unknown instruction: {}", op));
            }
        }
    }

    Ok(bytecode)
}
