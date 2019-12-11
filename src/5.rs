// --- Day 5: Sunny with a Chance of Asteroids ---
//
// https://adventofcode.com/2019/day/5
//
//

type Memory = Vec<i32>;

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
}
impl Opcode {
    fn from_chars(cs: &[char]) -> Opcode {
        match cs {
            ['0', '1'] => Self::ADD,
            ['0', '2'] => Self::MUL,
            ['0', '3'] => Self::STO,
            ['0', '4'] => Self::LOAD,
            ['0', '5'] => Self::JNZ,
            ['0', '6'] => Self::JZ,
            ['0', '7'] => Self::LT,
            ['0', '8'] => Self::EQ,
            ['9', '9'] => Self::HLT,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Immediate,
    Position,
}

impl ParameterMode {
    fn from_char(c: char) -> ParameterMode {
        match c {
            '1' => Self::Immediate,
            '0' => Self::Position,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum ComputerState {
    Running,
    Halted,
}

struct IntCodeComputer {
    pub pc: usize,
    pub mem: Memory,
    pub state: ComputerState,
    pub inp: Option<i32>,
    pc_mod: bool,
}

impl IntCodeComputer {
    fn new(mem: Memory) -> IntCodeComputer {
        IntCodeComputer {
            pc: 0,
            mem: mem,
            state: ComputerState::Running,
            inp: None,
            pc_mod: false,
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
                self.write_mem(left_operand + right_operand)
            }
            Opcode::MUL => {
                let left_operand = self.read_mem(ins.pm1);
                let right_operand = self.read_mem(ins.pm2);
                self.write_mem(left_operand * right_operand)
            }
            Opcode::STO => {
                let inp = self.read_from_terminal();
                self.write_mem(inp);
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
                    self.write_mem(1);
                } else {
                    self.write_mem(0);
                }
            }
            Opcode::LT => {
                let lft = self.read_mem(ins.pm1);
                let rgt = self.read_mem(ins.pm2);
                if lft < rgt {
                    self.write_mem(1);
                } else {
                    self.write_mem(0);
                }
            }
        }
    }

    fn increment_pc(&mut self) {
        self.pc += 1;
    }

    fn read_from_terminal(&self) -> i32 {
        if let Some(inp) = self.inp {
            return inp;
        }
        let mut input = String::new();
        print!("Enter Input: ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    fn read_mem(&mut self, mode: ParameterMode) -> i32 {
        self.increment_pc();
        match mode {
            ParameterMode::Immediate => self.mem[self.pc],
            ParameterMode::Position => self.mem[self.mem[self.pc] as usize],
        }
    }

    fn write_mem(&mut self, value: i32) {
        let addr = self.read_mem(ParameterMode::Immediate) as usize;
        self.mem[addr] = value;
    }
}

#[derive(Debug)]
struct Instruction {
    pub opcode: Opcode,
    pub pm1: ParameterMode,
    pub pm2: ParameterMode,
    pub pm3: ParameterMode,
}
impl Instruction {
    fn from(i: i32) -> Instruction {
        let i = format!("{:05}", i);
        let mut parts = i.chars();
        Instruction {
            pm3: ParameterMode::from_char(parts.next().unwrap()),
            pm2: ParameterMode::from_char(parts.next().unwrap()),
            pm1: ParameterMode::from_char(parts.next().unwrap()),
            opcode: Opcode::from_chars(&parts.collect::<Vec<_>>()),
        }
    }
}

fn main() {
    let program: Vec<i32> = include_str!("input5.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut computer = IntCodeComputer::new(program);
    computer.run();
}

#[test]
fn test_intcode_computer() {
    let values = vec![
        (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
        (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
        (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
        (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ),
        (vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99]),
    ];
    for (inp, out) in values {
        let mut c = IntCodeComputer::new(inp);
        c.run();
        assert_eq!(c.mem, out);
    }
}

#[test]
fn test_checking8() {
    let p = vec![
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    let mut c = IntCodeComputer::new(p);
    c.inp = Some(9);
    c.run();
}

#[test]
fn test_instruction_decoding() {
    let inst = Instruction::from(104);
    assert_eq!(inst.opcode, Opcode::LOAD);
    assert_eq!(inst.pm1, ParameterMode::Immediate);
    assert_eq!(inst.pm2, ParameterMode::Position);
    assert_eq!(inst.pm3, ParameterMode::Position);
}
