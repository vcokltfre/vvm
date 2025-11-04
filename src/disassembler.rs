use std::collections::HashMap;

use crate::{Program, optimise};

pub fn disasm(program: Program) -> String {
    let program = optimise(&program);

    let mut labels = HashMap::new();

    program.labels.iter().for_each(|(name, addr)| {
        labels.insert(*addr, name.clone());
    });

    let output = program
        .instructions
        .iter()
        .enumerate()
        .map(|(idx, instr)| {
            let mut line = String::new();
            if let Some(label) = labels.get(&idx) {
                line.push_str(&format!("LABEL {}\n", label));
            }
            line.push_str(&format!("    {}", instr));
            line
        })
        .collect::<Vec<String>>()
        .join("\n");

    output
}