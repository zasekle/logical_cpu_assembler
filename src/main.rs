use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, Read};

const NUMBER_BITS: usize = 8;

#[derive(Clone, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
}

impl Register {
    fn binary(reg: Register) -> &'static str {
        match reg {
            Register::R0 => "00",
            Register::R1 => "01",
            Register::R2 => "10",
            Register::R3 => "11",
        }
    }

    fn reg_from_instr(reg: &str, real_line_number: usize) -> Register {
        match reg {
            "R0" => Register::R0,
            "R1" => Register::R1,
            "R2" => Register::R2,
            "R3" => Register::R3,
            _ => panic!("{} Invalid register number found of {}.", real_line_number, reg)
        }
    }
}

#[allow(dead_code)]
pub enum Instructions {
    Add { reg_a: Register, reg_b: Register },
    Shr { reg_a: Register, reg_b: Register },
    Shl { reg_a: Register, reg_b: Register },
    Not { reg_a: Register, reg_b: Register },
    And { reg_a: Register, reg_b: Register },
    Or { reg_a: Register, reg_b: Register },
    XOr { reg_a: Register, reg_b: Register },
    Store { reg_a: Register, reg_b: Register },
    Load { reg_a: Register, reg_b: Register },
    Data { reg: Register, data: usize },
    JumpRegister { reg: Register },
    JumpAddress { mark: String },
    JumpIf { carry: bool, a_larger: bool, equal: bool, zero: bool, mark: String },
    ClearFlags,
    End,
}

impl Instructions {
    fn binary(instruction: Self) -> String {
        let binary_string =
            match instruction {
                Instructions::Add { reg_a, reg_b } => {
                    format!("1000{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::Shr { reg_a, reg_b } => {
                    format!("1001{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::Shl { reg_a, reg_b } => {
                    format!("1010{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::Not { reg_a, reg_b } => {
                    format!("1011{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::And { reg_a, reg_b } => {
                    format!("1100{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::Or { reg_a, reg_b } => {
                    format!("1101{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::XOr { reg_a, reg_b } => {
                    format!("1110{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::Store { reg_a, reg_b } => {
                    format!("0001{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::Load { reg_a, reg_b } => {
                    format!("0000{}{}", Register::binary(reg_a), Register::binary(reg_b))
                }
                Instructions::Data { reg, data } => {
                    let mut binary_data = format!("{:0width$b}", data, width = NUMBER_BITS);
                    while binary_data.len() > NUMBER_BITS {
                        binary_data.remove(0);
                    }
                    format!("001000{}\n{}", Register::binary(reg), binary_data)
                }
                Instructions::JumpRegister { reg } => {
                    format!("001100{}", Register::binary(reg))
                }
                Instructions::JumpAddress { .. } => {
                    format!("01000000")
                }
                Instructions::JumpIf { carry, a_larger, equal, zero, .. } => {
                    fn bool_char(b: bool) -> char {
                        match b {
                            true => '1',
                            false => '0',
                        }
                    }
                    format!(
                        "0101{}{}{}{}",
                        bool_char(carry),
                        bool_char(a_larger),
                        bool_char(equal),
                        bool_char(zero),
                    )
                }
                Instructions::ClearFlags => {
                    format!("01100000")
                }
                Instructions::End => "11001111".to_string(),
            };

        binary_string
    }
}

fn main() {
    let file_name = "code";
    let max_num_ram_cells = usize::pow(2, 8);

    let mut file = File::open(format!("programs/{}", file_name)).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut marks_to_machine_code = HashMap::new();
    let mut real_line_number = 0;
    let mut machine_code_line_number = 0;
    let mut instructions = Vec::new();
    for line in content.lines() {
        real_line_number += 1;

        let words: Vec<&str> = line.split_whitespace().collect();

        //Empty line.
        if words.is_empty() {
            continue;
        }

        //Comment.
        if words[0].starts_with('#') {
            continue;
        }

        //Marked for a jump point.
        if words[0] == "MARK" {
            if words.len() != 2 {
                panic!("{} Invalid formatting for registers.", real_line_number)
            }

            let mark_variable = words[1];

            marks_to_machine_code.insert(mark_variable.to_string(), machine_code_line_number + 1);
            continue;
        }

        match words[0] {
            //Values that use at least two registers.
            "ADD" | "SHR" | "SHL" | "NOT" | "AND" | "OR" | "XOR" | "ST" | "LD" => {
                if words.len() != 3 {
                    panic!("{} Invalid formatting for registers.", real_line_number)
                }

                let reg_a = Register::reg_from_instr(
                    words[1], real_line_number,
                );

                let reg_b = Register::reg_from_instr(
                    words[2], real_line_number,
                );

                machine_code_line_number += 1;

                instructions.push(
                    match words[0] {
                        "ADD" => {
                            Instructions::Add { reg_a, reg_b }
                        }
                        "SHR" => {
                            Instructions::Shr { reg_a, reg_b }
                        }
                        "SHL" => {
                            Instructions::Shl { reg_a, reg_b }
                        }
                        "NOT" => {
                            Instructions::Not { reg_a, reg_b }
                        }
                        "AND" => {
                            Instructions::And { reg_a, reg_b }
                        }
                        "OR" => {
                            Instructions::Or { reg_a, reg_b }
                        }
                        "XOR" => {
                            Instructions::XOr { reg_a, reg_b }
                        }
                        "ST" => {
                            Instructions::Store { reg_a, reg_b }
                        }
                        "LD" => {
                            Instructions::Load { reg_a, reg_b }
                        }
                        _ => panic!("{} Unknown instruction used, {}", real_line_number, words[0])
                    }
                );
            }
            "DATA" => {
                if words.len() != 3 {
                    panic!("{} Invalid formatting for registers.", real_line_number)
                }

                let reg = Register::reg_from_instr(
                    words[1], real_line_number,
                );

                let data: usize = words[2].parse().expect(
                    format!("{} Invalid number passed as data {}.", real_line_number, words[2]).as_str()
                );

                machine_code_line_number += 2;

                instructions.push(
                    Instructions::Data { reg, data }
                );
            }
            "JMPR" => {
                if words.len() != 2 {
                    panic!("{} Invalid formatting for registers.", real_line_number)
                }

                let reg = Register::reg_from_instr(
                    words[1], real_line_number,
                );

                machine_code_line_number += 1;

                instructions.push(
                    Instructions::JumpRegister { reg }
                );
            }
            "JMP" => {
                if words.len() != 2 {
                    panic!("{} Invalid formatting for registers.", real_line_number)
                }

                let mark_variable = words[1];

                machine_code_line_number += 2;

                instructions.push(
                    Instructions::JumpAddress { mark: mark_variable.to_string() }
                );
            }
            "JIF" => {
                if words.len() != 3 {
                    panic!("{} Invalid formatting for registers.", real_line_number)
                }

                let mut carry = false;
                let mut a_larger = false;
                let mut equal = false;
                let mut zero = false;

                for c in words[1].chars() {
                    match c {
                        'C' => {
                            carry = true;
                        }
                        'A' => {
                            a_larger = true;
                        }
                        'E' => {
                            equal = true;
                        }
                        'Z' => {
                            zero = true;
                        }
                        _ => panic!("{} Invalid formatting for JIF command {}.", real_line_number, c)
                    }
                }

                let mark_variable = words[1];

                machine_code_line_number += 2;

                instructions.push(
                    Instructions::JumpIf { carry, a_larger, equal, zero, mark: mark_variable.to_string() }
                );
            }
            "CLF" => {
                machine_code_line_number += 1;

                instructions.push(
                    Instructions::ClearFlags
                );
            }
            "END" => {
                machine_code_line_number += 1;

                instructions.push(
                    Instructions::End
                );
            }
            _ => panic!("{} Unknown instruction used, {}", real_line_number, words[0])
        };
    }

    let mut final_build: Vec<String> = Vec::new();
    for instruction in instructions {
        let mark =
            match &instruction {
                Instructions::JumpAddress { mark } => {
                    let machine_line = marks_to_machine_code.get(mark).expect(
                        format!("Mark {} not found.", mark).as_str()
                    );

                    let binary_input_number = format!("{:0width$b}", machine_line, width = NUMBER_BITS);

                    Some(binary_input_number)
                }
                Instructions::JumpIf { mark, .. } => {
                    let machine_line = marks_to_machine_code.get(mark).expect(
                        format!("Mark {} not found.", mark).as_str()
                    );

                    let binary_input_number = format!("{:0width$b}", machine_line, width = NUMBER_BITS);

                    Some(binary_input_number)
                }
                _ => None
            };

        final_build.push(
            Instructions::binary(
                instruction
            )
        );

        if let Some(mark) = mark {
            final_build.push(mark);
        }
    }

    final_build.push(
        Instructions::binary(
            Instructions::End
        )
    );

    if machine_code_line_number > max_num_ram_cells {
        panic!("File contains too many instructions. {} found, {} maximum.", machine_code_line_number, max_num_ram_cells);
    }

    let mut output_file = File::create(format!("machine_code/{}.ms", file_name).as_str()).expect("Failed to create output file.");

    for s in final_build {
        writeln!(output_file, "{}", s).expect("Unable to write to file.");
    }
}
