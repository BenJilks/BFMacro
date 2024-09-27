use crate::bf::{Instruction, BF};

fn resolve_combined_move(out: &mut BF, count: &mut i32) {
    for _ in 0..count.abs() {
        if *count > 0 {
            out.push(Instruction::Right);
        } else {
            out.push(Instruction::Left);
        }
    }

    *count = 0;
}

fn resolve_combined_change(out: &mut BF, count: &mut i32) {
    for _ in 0..count.abs() {
        if *count > 0 {
            out.push(Instruction::Add);
        } else {
            out.push(Instruction::Subtract);
        }
    }

    *count = 0;
}

pub fn simplify_program(program: &BF) -> BF {
    let mut simplified = BF::new();

    let mut combined_move = 0i32;
    let mut combined_change = 0i32;
    for instruction in program.code() {
        match instruction {
            Instruction::Left => {
                resolve_combined_change(&mut simplified, &mut combined_change);
                combined_move -= 1;
            }
            Instruction::Right => {
                resolve_combined_change(&mut simplified, &mut combined_change);
                combined_move += 1;
            }

            Instruction::Add => {
                resolve_combined_move(&mut simplified, &mut combined_move);
                combined_change += 1;
            }
            Instruction::Subtract => {
                resolve_combined_move(&mut simplified, &mut combined_move);
                combined_change -= 1;
            }

            instruction => {
                resolve_combined_move(&mut simplified, &mut combined_move);
                resolve_combined_change(&mut simplified, &mut combined_change);
                simplified.push(*instruction);
            }
        }
    }

    resolve_combined_move(&mut simplified, &mut combined_move);
    resolve_combined_change(&mut simplified, &mut combined_change);
    simplified
}
