// --- Day 9: Sensor Boost ---
//
// https://adventofcode.com/2019/day/9
//

type LargeNumber = i64;
type Memory = Vec<LargeNumber>;
type Program = Vec<LargeNumber>;

#[derive(Debug, PartialEq)]
enum Opcode {
    ADD,
    MUL,
    STO,
    LOAD,
    HLT,
    JNZ,
    JZ,
    LT,
    EQ,
    RBO,
}
impl Opcode {
    fn from_i64(n: i64) -> Opcode {
        match n {
            1 => Self::ADD,
            2 => Self::MUL,
            3 => Self::STO,
            4 => Self::LOAD,
            5 => Self::JNZ,
            6 => Self::JZ,
            7 => Self::LT,
            8 => Self::EQ,
            9 => Self::RBO,
            99 => Self::HLT,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum AddressingMode {
    Immediate,
    Position,
    Relative,
}

impl AddressingMode {
    fn from_i64(n: i64) -> AddressingMode {
        match n {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ComputerState {
    Running,
    Halted,
}

#[derive(Debug)]
struct IntCodeComputer {
    pub pc: usize,
    pub mem: Memory,
    pub state: ComputerState,
    pub inp: Option<LargeNumber>,
    pc_mod: bool,
    relbase: LargeNumber,
}

impl IntCodeComputer {
    fn new(mem: Memory) -> IntCodeComputer {
        IntCodeComputer {
            pc: 0,
            mem: mem,
            state: ComputerState::Running,
            inp: None,
            pc_mod: false,
            relbase: 0,
        }
    }

    fn run(&mut self) {
        while self.state != ComputerState::Halted {
            self.pc_mod = false;
            let ins = self.decode();
            self.execute(ins);
            if !self.pc_mod {
                self.increment_pc();
            }
        }
    }

    fn decode(&self) -> Instruction {
        let ins = self.mem.get(self.pc).unwrap();
        Instruction::from(*ins)
    }

    fn execute(&mut self, ins: Instruction) {
        match ins.opcode {
            Opcode::HLT => self.state = ComputerState::Halted,
            Opcode::ADD => {
                let left_operand = self.read_mem(ins.pm1);
                let right_operand = self.read_mem(ins.pm2);
                self.write_mem(left_operand + right_operand, ins.pm3)
            }
            Opcode::MUL => {
                let left_operand = self.read_mem(ins.pm1);
                let right_operand = self.read_mem(ins.pm2);
                self.write_mem(left_operand * right_operand, ins.pm3)
            }
            Opcode::STO => {
                let inp = self.read_from_terminal();
                self.write_mem(inp, ins.pm1);
            }
            Opcode::LOAD => {
                println!("{}", self.read_mem(ins.pm1));
            }
            Opcode::JNZ => {
                let lft = self.read_mem(ins.pm1);
                let rgt = self.read_mem(ins.pm2);
                if lft != 0 {
                    self.pc = rgt as usize;
                    self.pc_mod = true;
                }
            }
            Opcode::JZ => {
                let lft = self.read_mem(ins.pm1);
                let rgt = self.read_mem(ins.pm2);
                if lft == 0 {
                    self.pc = rgt as usize;
                    self.pc_mod = true;
                }
            }
            Opcode::EQ => {
                let lft = self.read_mem(ins.pm1);
                let rgt = self.read_mem(ins.pm2);
                if lft == rgt {
                    self.write_mem(1, ins.pm3);
                } else {
                    self.write_mem(0, ins.pm3);
                }
            }
            Opcode::LT => {
                let lft = self.read_mem(ins.pm1);
                let rgt = self.read_mem(ins.pm2);
                if lft < rgt {
                    self.write_mem(1, ins.pm3);
                } else {
                    self.write_mem(0, ins.pm3);
                }
            }
            Opcode::RBO => {
                self.relbase += self.read_mem(ins.pm1);
            }
        }
    }

    fn increment_pc(&mut self) {
        self.pc = self.check_bounds(self.pc + 1);
    }

    fn read_from_terminal(&self) -> LargeNumber {
        if let Some(inp) = self.inp {
            return inp;
        }
        let mut input = String::new();
        print!("Enter Input: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    fn check_bounds(&mut self, idx: usize) -> usize {
        if idx >= self.mem.len() {
            let diff = vec![0; (idx - self.mem.len()) + 1];
            self.mem.extend_from_slice(&diff);
        }
        idx
    }

    fn read_mem(&mut self, mode: AddressingMode) -> LargeNumber {
        self.increment_pc();

        let loc = match mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::Position => {
                let mut pos = self.pc;
                pos = self.mem[pos] as usize;
                self.check_bounds(pos)
            }
            AddressingMode::Relative => {
                let pos = (self.mem[self.pc] + self.relbase) as usize;
                self.check_bounds(pos)
            }
        };
        self.mem[loc]
    }

    fn write_mem(&mut self, value: LargeNumber, mode: AddressingMode) {
        self.increment_pc();
        let addr = match mode {
            AddressingMode::Position => self.mem[self.pc] as usize,
            AddressingMode::Relative => (self.mem[self.pc] + self.relbase) as usize,
            _ => unreachable!(),
        };
        self.check_bounds(addr);
        self.mem[addr] = value;
    }
}

#[derive(Debug)]
struct Instruction {
    pub opcode: Opcode,
    pub pm1: AddressingMode,
    pub pm2: AddressingMode,
    pub pm3: AddressingMode,
}
impl Instruction {
    fn from(i: LargeNumber) -> Instruction {
        let modes = i / 100;
        Instruction {
            pm3: AddressingMode::from_i64(modes / 10i64.pow(2) % 10),
            pm2: AddressingMode::from_i64(modes / 10i64.pow(1) % 10),
            pm1: AddressingMode::from_i64(modes / 10i64.pow(0) % 10),
            opcode: Opcode::from_i64(i % 100),
        }
    }
}

fn main() {
    let program: Program = include_str!("input9.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut computer = IntCodeComputer::new(program);
    computer.run();
}

#[test]
fn test_large_numbers() {
    let mut c = IntCodeComputer::new(vec![104, 1125899906842624, 99]);
    c.run(); // prints: 1125899906842624
}

#[test]
fn test_self_copier() {
    let mut c = IntCodeComputer::new(vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]);
    c.run(); // prints out itself
}
