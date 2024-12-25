use anyhow::{Context, Result};
use std::fs::File;
use std::io;
use std::io::BufRead;

#[allow(non_snake_case)]
#[derive(Clone)]
struct Registers {
    A: u64,
    B: u64,
    C: u64,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
enum Instruction {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl Instruction {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Instruction::adv),
            1 => Some(Instruction::bxl),
            2 => Some(Instruction::bst),
            3 => Some(Instruction::jnz),
            4 => Some(Instruction::bxc),
            5 => Some(Instruction::out),
            6 => Some(Instruction::bdv),
            7 => Some(Instruction::cdv),
            _ => None,
        }
    }
}

struct Computer {
    registers: Registers,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn from_file(file: File) -> Result<Self> {
        let reader = io::BufReader::new(file);
        let mut lines = reader.lines();
        let register_a: u64 = lines
            .next()
            .context("Register A does not exist")??
            .split(": ")
            .nth(1)
            .context("Invalid line for Register A")?
            .parse()?;
        let register_b: u64 = lines
            .next()
            .context("Register B does not exist")??
            .split(": ")
            .nth(1)
            .context("Invalid line for Register B")?
            .parse()?;
        let register_c: u64 = lines
            .next()
            .context("Register C does not exist")??
            .split(": ")
            .nth(1)
            .context("Invalid line for Register C")?
            .parse()?;

        let registers = Registers {
            A: register_a,
            B: register_b,
            C: register_c,
        };

        lines.next(); //Skip blank line

        let program_line = lines.next().context("Instructions not present")??;
        let instructions: Vec<Instruction> = program_line
            .split(": ")
            .nth(1)
            .context("Invalid line for instructions")?
            .split(',')
            .map(|s| {
                Instruction::from_u8(s.parse().context("Failed to parse instruction")?)
                    .context("Invalid Instruction")
            })
            .collect::<Result<Vec<Instruction>>>()?;

        Ok(Computer {
            registers,
            instructions,
        })
    }

    fn do_operations(&self) -> Vec<u64> {
        let mut pointer = 0;
        let mut output = Vec::new();
        let mut registers = self.registers.clone();

        while pointer < self.instructions.len() {
            let instruction = self.instructions[pointer].clone();
            let operand = self.instructions[pointer + 1].clone();

            let combo = |operand: Instruction| -> u64 {
                match operand {
                    Instruction::adv => 0,
                    Instruction::bxl => 1,
                    Instruction::bst => 2,
                    Instruction::jnz => 3,
                    Instruction::bxc => registers.A,
                    Instruction::out => registers.B,
                    Instruction::bdv => registers.C,
                    Instruction::cdv => panic!("Unrecognized combo operand"),
                }
            };

            match instruction {
                Instruction::adv => registers.A >>= combo(operand),
                Instruction::bxl => registers.B ^= operand as u64,
                Instruction::bst => registers.B = combo(operand) % 8,
                Instruction::jnz => {
                    if registers.A != 0 {
                        pointer = operand as usize;
                        continue;
                    }
                }
                Instruction::bxc => registers.B ^= registers.C,
                Instruction::out => output.push(combo(operand) % 8),
                Instruction::bdv => registers.B = registers.A >> combo(operand),
                Instruction::cdv => registers.C = registers.A >> combo(operand),
            }

            pointer += 2;
        }

        output
    }

    fn part_1(&self) -> String {
        self.do_operations()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn find_register_a(&self) -> Option<u64> {
        for a in 0..=u64::MAX {
            let mut registers = self.registers.clone();
            registers.A = a;
            let output = self.do_operations_with_registers(registers);
            if output
                == self
                    .instructions
                    .iter()
                    .map(|i| i.clone() as u64)
                    .collect::<Vec<_>>()
            {
                return Some(a);
            }
        }
        None
    }

    fn do_operations_with_registers(&self, registers: Registers) -> Vec<u64> {
        let mut pointer = 0;
        let mut output = Vec::new();
        let mut registers = registers;

        while pointer < self.instructions.len() {
            let instruction = self.instructions[pointer].clone();
            let operand = self.instructions[pointer + 1].clone();

            let combo = |operand: Instruction| -> u64 {
                match operand {
                    Instruction::adv => 0,
                    Instruction::bxl => 1,
                    Instruction::bst => 2,
                    Instruction::jnz => 3,
                    Instruction::bxc => registers.A,
                    Instruction::out => registers.B,
                    Instruction::bdv => registers.C,
                    Instruction::cdv => panic!("Unrecognized combo operand"),
                }
            };

            match instruction {
                Instruction::adv => registers.A >>= combo(operand),
                Instruction::bxl => registers.B ^= operand as u64,
                Instruction::bst => registers.B = combo(operand) % 8,
                Instruction::jnz => {
                    if registers.A != 0 {
                        pointer = operand as usize;
                        continue;
                    }
                }
                Instruction::bxc => registers.B ^= registers.C,
                Instruction::out => output.push(combo(operand) % 8),
                Instruction::bdv => registers.B = registers.A >> combo(operand),
                Instruction::cdv => registers.C = registers.A >> combo(operand),
            }

            pointer += 2;
        }

        output
    }
}

use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam::channel;

impl Computer {
    fn find_register_a_parallel(&self, num_threads: usize) -> Option<u64> {
        let (sender, receiver) = channel::unbounded();
        let registers = Arc::new(self.registers.clone());
        let instructions = Arc::new(self.instructions.clone());

        let chunk_size = u64::MAX / num_threads as u64;
        let found = Arc::new(Mutex::new(None));

        for i in 0..num_threads {
            let sender = sender.clone();
            let registers = Arc::clone(&registers);
            let instructions = Arc::clone(&instructions);
            let found = Arc::clone(&found);

            let start = i as u64 * chunk_size;
            let end = if i == num_threads - 1 {
                u64::MAX
            } else {
                (i as u64 + 1) * chunk_size - 1
            };

            thread::spawn(move || {
                for a in start..=end {
                    let mut registers = (*registers).clone();
                    registers.A = a;
                    let output = Computer::do_operations_with_registers_static(&instructions, registers);
                    if output == instructions.iter().map(|i| i.clone() as u64).collect::<Vec<_>>() {
                        let mut found = found.lock().unwrap();
                        if found.is_none() || a < found.unwrap() {
                            *found = Some(a);
                            sender.send(a).unwrap();
                        }
                        break;
                    }
                }
            });
        }

        drop(sender); // Close the sender to stop the receiver from waiting for more messages
        receiver.recv().ok()
    }

    fn do_operations_with_registers_static(instructions: &[Instruction], mut registers: Registers) -> Vec<u64> {
        let mut pointer = 0;
        let mut output = Vec::new();

        while pointer < instructions.len() {
            let instruction = instructions[pointer].clone();
            let operand = instructions[pointer + 1].clone();

            let combo = |operand: Instruction| -> u64 {
                match operand {
                    Instruction::adv => 0,
                    Instruction::bxl => 1,
                    Instruction::bst => 2,
                    Instruction::jnz => 3,
                    Instruction::bxc => registers.A,
                    Instruction::out => registers.B,
                    Instruction::bdv => registers.C,
                    Instruction::cdv => panic!("Unrecognized combo operand"),
                }
            };

            match instruction {
                Instruction::adv => registers.A >>= combo(operand),
                Instruction::bxl => registers.B ^= operand as u64,
                Instruction::bst => registers.B = combo(operand) % 8,
                Instruction::jnz => {
                    if registers.A != 0 {
                        pointer = operand as usize;
                        continue;
                    }
                }
                Instruction::bxc => registers.B ^= registers.C,
                Instruction::out => output.push(combo(operand) % 8),
                Instruction::bdv => registers.B = registers.A >> combo(operand),
                Instruction::cdv => registers.C = registers.A >> combo(operand),
            }

            pointer += 2;
        }

        output
    }
}

fn main() -> Result<()> {
    let file = File::open("inputs/input17.txt")?;
    let computer = Computer::from_file(file)?;

    //Part-1
    println!("{}", computer.part_1());
    //7,3,5,7,5,7,4,3,0

    println!("{}", computer.find_register_a_parallel(8).unwrap());

    Ok(())
}
