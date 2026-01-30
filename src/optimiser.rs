use std::collections::HashMap;

use crate::{Instruction, Program};

pub fn optimise(program: &Program) -> Program {
    let mut instructions = vec![];

    let mut prev_was_store_imm = false;
    let mut prev_store_imm_name = String::new();

    for instr in &program.instructions {
        instructions.push(instr.clone());

        match instr {
            Instruction::StoreImm(name) => {
                prev_was_store_imm = true;
                prev_store_imm_name = name.clone();
            }
            Instruction::LoadImm(name) if prev_was_store_imm && *name == prev_store_imm_name => {
                instructions.pop(); // Remove the LoadImm
                instructions.pop(); // Remove the StoreImm
                prev_was_store_imm = false;

                instructions.push(Instruction::Dup);
                instructions.push(Instruction::StoreImm(prev_store_imm_name.clone()));
            }
            _ => {
                prev_was_store_imm = false;
            }
        }
    }

    let mut names = HashMap::<String, usize>::new();
    let mut labels = HashMap::<String, usize>::new();

    for label in program.labels.clone() {
        let li = names.len();
        names.insert(label.0.clone(), li);
        labels.insert(format!("{}", li), label.1);
    }

    let mut final_instructions = vec![];

    for instr in &instructions {
        match instr {
            Instruction::Jump(l) => {
                let target = names.get(l).expect("Label not found");
                final_instructions.push(Instruction::Jump(format!("{}", target)));
            }
            Instruction::JumpIf(l) => {
                let target = names.get(l).expect("Label not found");
                final_instructions.push(Instruction::JumpIf(format!("{}", target)));
            }
            Instruction::Call(l) => {
                let target = names.get(l).expect("Label not found");
                final_instructions.push(Instruction::Call(format!("{}", target)));
            }
            _ => {
                final_instructions.push(instr.clone());
            }
        }
    }

    Program {
        instructions: final_instructions,
        labels,
    }
}
