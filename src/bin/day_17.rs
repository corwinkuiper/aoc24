use itertools::Itertools;

#[derive(Debug, Clone)]
struct Machine {
    instruction_pointer: usize,
    registers: [i64; 3],
}

#[derive(Clone, Copy, Debug)]
enum ComboOperand {
    Literal(i64),
    Register(usize),
}

#[derive(Clone, Copy, Debug)]
struct Operand(u8);

impl From<Operand> for ComboOperand {
    fn from(value: Operand) -> Self {
        match value.0 {
            x @ 0..4 => ComboOperand::Literal(x as i64),
            reg @ 4..7 => ComboOperand::Register(reg as usize - 4),
            unexpected => panic!("Unexpected combo operand value {}", unexpected),
        }
    }
}

impl From<Operand> for i64 {
    fn from(value: Operand) -> Self {
        value.0.into()
    }
}

impl Operand {
    fn to_value(self) -> i64 {
        self.0.into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            invalid => panic!("invalid opcode: {}", invalid),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

#[derive(Debug)]
struct Program(Vec<u8>);

impl Program {
    fn get(&self, instruction_pointer: usize) -> Option<Instruction> {
        let &a = self.0.get(instruction_pointer)?;
        let &b = self.0.get(instruction_pointer + 1)?;
        let opcode = a.into();

        Some(Instruction {
            opcode,
            operand: Operand(b),
        })
    }
}

impl Machine {
    fn get_operand_value(&self, operand: ComboOperand) -> i64 {
        match operand {
            ComboOperand::Literal(x) => x,
            ComboOperand::Register(x) => self.registers[x],
        }
    }

    fn execute_instruction(&mut self, program: &Program, output: &mut Vec<u8>) {
        let Some(instruction) = program.get(self.instruction_pointer) else {
            return;
        };

        match instruction.opcode {
            Opcode::Adv => {
                self.registers[0] /= 2i64.pow(
                    self.get_operand_value(instruction.operand.into())
                        .try_into()
                        .unwrap(),
                );
            }
            Opcode::Bxl => self.registers[1] ^= instruction.operand.to_value(),
            Opcode::Bst => {
                self.registers[1] = self.get_operand_value(instruction.operand.into()) & 0b111
            }
            Opcode::Jnz => {
                if self.registers[0] != 0 {
                    self.instruction_pointer = instruction.operand.0.into();
                    return;
                }
            }
            Opcode::Bxc => self.registers[1] ^= self.registers[2],
            Opcode::Out => output.push(
                (self.get_operand_value(instruction.operand.into()) & 0b111)
                    .try_into()
                    .unwrap(),
            ),
            Opcode::Bdv => {
                self.registers[1] = self.registers[0]
                    / 2i64.pow(
                        self.get_operand_value(instruction.operand.into())
                            .try_into()
                            .unwrap(),
                    )
            }
            Opcode::Cdv => {
                self.registers[2] = self.registers[0]
                    / 2i64.pow(
                        self.get_operand_value(instruction.operand.into())
                            .try_into()
                            .unwrap(),
                    )
            }
        }
        self.instruction_pointer += 2;
    }

    fn halted(&self, program: &Program) -> bool {
        program.get(self.instruction_pointer).is_none()
    }
}

fn parse(input: &str) -> (Machine, Program) {
    let mut lines = input.lines();
    let mut get_register = || {
        lines
            .next()
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse::<i64>()
            .unwrap()
    };

    let registers = [get_register(), get_register(), get_register()];

    lines.next().unwrap();
    let program = lines
        .next()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    (
        Machine {
            registers,
            instruction_pointer: 0,
        },
        Program(program),
    )
}

fn part_1(input: &str) -> String {
    let (mut machine, program) = parse(input);
    let mut output = Vec::new();
    while !machine.halted(&program) {
        machine.execute_instruction(&program, &mut output);
    }

    output.into_iter().map(|x| x.to_string()).join(",")
}

// specific for my program

fn reverse(a: u64, expected_output: &[u8]) -> Option<u64> {
    let Some(&e) = expected_output.last() else {
        return Some(a);
    };
    let e = u64::from(e);

    for lower_bits in 0..8 {
        let a = (a << 3) + lower_bits;
        let b = a & 0b111;
        let b = b ^ 3;
        let c = a >> b;
        let b = b ^ 5;
        let b = b ^ c;
        if b & 0b111 != e {
            continue;
        }
        if let Some(r) = reverse(a, &expected_output[0..expected_output.len() - 1]) {
            return Some(r);
        }
    }

    None
}

fn part_2(input: &str) -> i64 {
    let (_, program) = parse(input);
    reverse(0, &program.0).unwrap() as i64
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}
