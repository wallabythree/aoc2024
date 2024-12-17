use crate::Solution;

pub const SOLUTION: Solution<String, usize> = Solution { part1, part2 };

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Instruction {
    Adv(Combo),
    Bxl(Literal),
    Bst(Combo),
    Jnz(Literal),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from((opcode, operand): (u8, u8)) -> Result<Self, Self::Error> {
        match opcode {
            0 => Ok(Instruction::Adv(operand.try_into()?)),
            1 => Ok(Instruction::Bxl(operand.try_into()?)),
            2 => Ok(Instruction::Bst(operand.try_into()?)),
            3 => Ok(Instruction::Jnz(operand.try_into()?)),
            4 => Ok(Instruction::Bxc),
            5 => Ok(Instruction::Out(operand.try_into()?)),
            6 => Ok(Instruction::Bdv(operand.try_into()?)),
            7 => Ok(Instruction::Cdv(operand.try_into()?)),
            _ => Err("Invalid opcode".into()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Literal(i64);

impl TryFrom<u8> for Literal {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 7 {
            Ok(Self(value.into()))
        } else {
            Err("Invalid literal".into())
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Register(usize);

impl TryFrom<u8> for Register {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 3 {
            Ok(Self(value.into()))
        } else {
            Err("Invalid register".into())
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Combo {
    Literal(Literal),
    Register(Register),
}

impl TryFrom<u8> for Combo {
    type Error = Box<dyn std::error::Error>;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        if byte < 4 {
            Ok(Self::Literal(byte.try_into()?))
        } else if byte < 7 {
            Ok(Self::Register((byte - 4).try_into()?))
        } else {
            Err("Invalid operand".into())
        }
    }
}

#[derive(Debug)]
struct VM {
    regs: [i64; 3],
    ip: usize,
    text: Vec<u8>,
    output: Vec<u8>,
}

impl VM {
    fn read(&self, reg: Register) -> i64 {
        self.regs[reg.0]
    }

    fn write(&mut self, reg: Register, val: i64) {
        self.regs[reg.0] = val;
    }

    fn deref_combo(&self, combo: Combo) -> i64 {
       match combo {
            Combo::Literal(lit) => lit.0,
            Combo::Register(reg) => self.read(reg),
        }
    }

    fn shr(&mut self, reg: Register, operand: Combo) {
        let numerator = self.read(Register(0));
        let denominator = self.deref_combo(operand);

        let result = numerator >> denominator;
        self.write(reg, result);
    }

    fn execute(&mut self, operation: Instruction) {
        let mut jumped = false;

        match operation {
            Instruction::Adv(operand) => {
                self.shr(Register(0), operand);
            },
            Instruction::Bxl(operand) => {
                let b = self.read(Register(1));
                let result = b ^ operand.0;
                self.write(Register(1), result);
            },
            Instruction::Bst(operand) => {
                let val = self.deref_combo(operand);
                let result = val & 7;
                self.write(Register(1), result);
            },
            Instruction::Jnz(operand) => {
                let a = self.read(Register(0));

                if a != 0 {
                    self.ip = operand.0.try_into().expect("Invalid ip");
                    jumped = true;
                }
            },
            Instruction::Bxc => {
                let (b, c) = (self.read(Register(1)), self.read(Register(2)));
                let result = b ^ c;
                self.write(Register(1), result);
            },
            Instruction::Out(operand) => {
                let val = self.deref_combo(operand);
                let result = (val & 7) as u8;
                self.output.push(result);
            },
            Instruction::Bdv(operand) => {
                self.shr(Register(1), operand);
            },
            Instruction::Cdv(operand) => {
                self.shr(Register(2), operand);
            },
        }

        if !jumped {
            self.ip += 2;
        }
    }

    fn step(&mut self) {
        let instruction: Instruction = (
            self.text[self.ip],
            self.text[self.ip + 1])
            .try_into().expect("Invalid instruction");

        self.execute(instruction);
    }

    fn run(&mut self) {
        while self.ip < self.text.len() {
            self.step();
        }
    }

    fn reset(&mut self) {
        self.regs = [0; 3];
        self.ip = 0;
        self.output.clear();
    }

    fn output(&self) -> String {
        self.output.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")
    }
}

impl TryFrom<&str> for VM {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (regs_str, program_str) = input
            .split_once("\n\n")
            .ok_or("Invalid input")?;

        let regs = regs_str
            .lines()
            .flat_map(|line| line
                .split_once(':')
                .map(|(_, r)| r.trim().parse::<i64>())
                .ok_or("Invalid register value")
            )
            .collect::<Result<Vec<_>, _>>()?;

        let program = program_str
            .split_once(':')
            .map(|(_, r)| r
                .trim()
                .split(',')
                .map(|c| c.parse::<u8>())
                .collect::<Result<Vec<_>, _>>()
            )
            .ok_or("Invalid program")?;

        let vm = Self {
            regs: regs.try_into().map_err(|_| "Invalid number of registers")?,
            ip: 0,
            text: program?,
            output: vec!(),
        };

        Ok(vm)
    }
}

fn part1(input: &str) -> String {
    let mut vm = VM::try_from(input).unwrap();
    vm.run();
    vm.output()
}

fn part2(input: &str) -> usize {
    let mut vm = VM::try_from(input).unwrap();
    let mut a: i64 = -1;

    for i in 0.. {
        while vm.output.len() <= i || !vm.text.ends_with(&vm.output) {
            a += 1;

            vm.reset();
            vm.regs[0] = a;
            vm.run();
        }

        if vm.output == vm.text {
            break;
        }

        a *= 8;
        a -= 1;
    }

    a.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const TEST_INPUT_2: &str = "Register A: 2024
 Register B: 0
 Register C: 0

 Program: 0,3,5,4,3,0
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), 117440);
    }
}
