use std::{collections::HashMap};

use crate::space_pool;

const FEE_PROGRAM_COUNTER: i32 = 1000;

#[derive(Debug)]
enum Value {
    Int(isize),
    Str(String),
}

enum Instruction {
    Mov(String, isize),
    MovStr(String, String),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Mod(String, String),
    Jmp(isize),
    If(String, String, String, isize),
    Else(String, String, isize),
}

struct Assembler {
    registers: HashMap<String, Value>,
    program_counter: isize,
    total_program_counter: i32,
    instructions: Vec<Instruction>,
}

impl Assembler {
    fn new() -> Assembler {
        Assembler {
            registers: HashMap::new(),
            total_program_counter: 0,
            program_counter: 0,
            instructions: Vec::new(),
        }
    }

    fn parse_instructions(&mut self, input: &str) {
        self.instructions = input.lines().filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                ["MOV", reg, val] => val.parse().ok().map(|val| Instruction::Mov(reg.to_string(), val)),
                ["MOVSTR", reg, val] => val.parse().ok().map(|val| Instruction::MovStr(reg.to_string(), val)),
                ["ADD", dest, src] => Some(Instruction::Add(dest.to_string(), src.to_string())),
                ["SUB", dest, src] => Some(Instruction::Sub(dest.to_string(), src.to_string())),
                ["MUL", dest, src] => Some(Instruction::Mul(dest.to_string(), src.to_string())),
                ["DIV", dest, src] => Some(Instruction::Div(dest.to_string(), src.to_string())),
                ["MOD", dest, src] => Some(Instruction::Mod(dest.to_string(), src.to_string())),
                ["JMP", offset] => offset.parse().ok().map(Instruction::Jmp),
                ["IF", reg1, rule, reg2, offset] => offset.parse().ok().map(|off| Instruction::If(reg1.to_string(), rule.to_string(), reg2.to_string(), off)),
                ["ELSE", reg1, reg2, offset] => offset.parse().ok().map(|off| Instruction::Else(reg1.to_string(), reg2.to_string(), off)),
                _ => None,
            }
        }).collect();
    }

    fn execute(&mut self) {
        while let Some(instruction) = self.instructions.get(self.program_counter as usize) {
            self.total_program_counter += 1;
            if self.total_program_counter >= FEE_PROGRAM_COUNTER {
                break;
            }
            match instruction {
                Instruction::Mov(reg, val) => {
                    self.registers.insert(reg.clone(), Value::Int(*val));
                    self.program_counter += 1;
                },
                Instruction::MovStr(reg, val) => {
                    self.registers.insert(reg.clone(), Value::Str(val.clone()));
                    self.program_counter += 1;
                },
                Instruction::Add(dest, src) => {
                    match (self.registers.get(src), self.registers.get(dest)) {
                        (Some(Value::Int(src_val)), Some(Value::Int(dest_val))) => {
                            self.registers.insert(dest.clone(), Value::Int(dest_val + src_val));
                        },
                        (Some(Value::Str(src_val)), Some(Value::Str(dest_val))) => {
                            self.registers.insert(dest.clone(),  Value::Str(dest_val.clone() + src_val));
                        },
                        _ => {
                        }
                    }
                    self.program_counter += 1;
                },
                Instruction::Sub(dest, src) => {
                    match (self.registers.get(src), self.registers.get(dest)) {
                        (Some(Value::Int(src_val)), Some(Value::Int(dest_val))) => {
                            self.registers.insert(dest.clone(), Value::Int(dest_val - src_val));
                        },
                        _ => {
                        }
                    }
                    self.program_counter += 1;
                },
                Instruction::Mul(dest, src) => {
                    if let (Some(Value::Int(dest_val)), Some(Value::Int(src_val))) = (self.registers.get(dest), self.registers.get(src)) {
                        self.registers.insert(dest.clone(), Value::Int(dest_val * src_val));
                    }
                    self.program_counter += 1;
                },
                Instruction::Div(dest, src) => {
                    if let (Some(Value::Int(dest_val)), Some(Value::Int(src_val))) = (self.registers.get(dest), self.registers.get(src)) {
                        if src_val != &0 {
                            self.registers.insert(dest.clone(), Value::Int(dest_val / src_val));
                        } else {
                            eprintln!("Error: Division by zero");
                        }
                    }
                    self.program_counter += 1;
                },
                Instruction::Mod(dest, src) => {
                    if let (Some(Value::Int(dest_val)), Some(Value::Int(src_val))) = (self.registers.get(dest), self.registers.get(src)) {
                        if src_val != &0 {
                            self.registers.insert(dest.clone(), Value::Int(dest_val % src_val));
                        } else {
                            eprintln!("Error: Division by zero");
                        }
                    }
                    self.program_counter += 1;
                },
                Instruction::Jmp(offset) => {
                    self.program_counter = 0 + offset;
                },
                Instruction::If(reg1, rule, reg2, offset) => {
                    let val1 = self.registers.get(reg1);
                    let val2 = self.registers.get(reg2);
                
                    match (val1, val2) {
                        (Some(Value::Int(v1)), Some(Value::Int(v2))) if v1 == v2 => {
                            self.program_counter += offset;
                        },
                        (Some(Value::Str(s1)), Some(Value::Str(s2))) if s1 == s2 => {
                            self.program_counter += offset;
                        },
                        _ => {
                            self.program_counter += 1;
                        }
                    }
                },
                Instruction::Else(reg1, reg2, offset) => {
                    let val1 = self.registers.get(reg1);
                    let val2 = self.registers.get(reg2);
                
                    match (val1, val2) {
                        (Some(Value::Int(v1)), Some(Value::Int(v2))) if v1 != v2 => {
                            self.program_counter += offset;
                        },
                        (Some(Value::Str(s1)), Some(Value::Str(s2))) if s1 != s2 => {
                            self.program_counter += offset;
                        },
                        _ => {
                            self.program_counter += 1;
                        }
                    }
                }
            }
        }
    }
}


pub fn message(program: String) -> String {
    let mut assembler = Assembler::new();
    assembler.parse_instructions(&program);
    assembler.execute();
    for (reg, val) in assembler.registers.iter() {
        println!("{} = {:#?}", reg, val);
        space_pool::get_space_value();
    }

    String::from("vm")
}
