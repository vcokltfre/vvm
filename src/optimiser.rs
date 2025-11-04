use crate::{Instruction, Program};

pub fn optimise(program: &Program) -> Program {
    let mut instructions = vec![];
    let labels = program.labels.clone();

    let mut prev_was_store_imm = false;
    let mut prev_store_imm_name = String::new();

    for instr in &program.instructions {
        instructions.push(instr.clone());

        match instr {
            Instruction::StoreImm (name) => {
                prev_was_store_imm = true;
                prev_store_imm_name = name.clone();
            }
            Instruction::LoadImm (name) if prev_was_store_imm && *name == prev_store_imm_name => {
                instructions.pop(); // Remove the LoadImm
                instructions.pop(); // Remove the StoreImm
                prev_was_store_imm = false;

                instructions.push(Instruction::Dup);
                instructions.push(Instruction::StoreImm (prev_store_imm_name.clone()));
            }
            _ => {
                prev_was_store_imm = false;
            }
        }
    }

    Program { instructions, labels }
}