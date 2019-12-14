// --- Day 7: Amplification Circuit ---
//
// https://adventofcode.com/2019/day/7
//
use std::sync::mpsc;

type Program = Vec<i32>;
type Sequence = Vec<i32>;
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

#[derive(Debug, PartialEq)]
enum ComputerState {
    Running,
    Halted,
}

#[derive(Debug, PartialEq)]
pub enum AmpMode {
    Linear,
    Feedback,
}

#[derive(Debug)]
/// Customized int code computer
pub struct Amplifier {
    pub pc: usize,
    pub mem: Memory,
    state: ComputerState,
    mode: AmpMode,
    pc_mod: bool,
    output: Option<mpsc::Sender<i32>>,
    input: Option<mpsc::Receiver<i32>>,
    phase: Option<i32>,
    final_output: Option<mpsc::Sender<i32>>,
}

impl Amplifier {
    pub fn new(mem: Memory) -> Amplifier {
        Amplifier {
            pc: 0,
            mem,
            state: ComputerState::Running,
            pc_mod: false,
            mode: AmpMode::Linear,
            output: None,
            input: None,
            phase: None,
            final_output: None,
        }
    }

    pub fn set_input(&mut self, r: mpsc::Receiver<i32>) {
        self.input = Some(r);
    }
    pub fn set_output(&mut self, t: mpsc::Sender<i32>) {
        self.output = Some(t);
    }
    pub fn set_final(&mut self, t: mpsc::Sender<i32>) {
        self.final_output = Some(t);
    }

    pub fn with_phase(&mut self, ph: i32) -> &mut Self {
        self.phase = Some(ph);
        self
    }

    pub fn with_mode(&mut self, m: AmpMode) -> &mut Self {
        self.mode = m;
        self
    }

    pub fn run(&mut self) {
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
                let inp = self.get_input().unwrap();
                self.write_mem(inp);
            }
            Opcode::LOAD => {
                let val = self.read_mem(ins.pm1);
                if self.mode == AmpMode::Feedback {
                    if let Some(outbox) = self.output.as_ref() {
                        match outbox.send(val) {
                            Ok(val) => return val,
                            Err(_) => {
                                if let Some(t) = self.final_output.as_ref() {
                                    t.send(val).unwrap();
                                } else {
                                    eprintln!("Error writing to outbox!: {}", val);
                                }
                            }
                        }
                    }
                    //self.output.as_ref().expect("Outbox").send(val).expect("Failed to write to outbox");
                }
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

    fn get_input(&mut self) -> Option<i32> {
        if let Some(value) = self.phase.take() {
            return Some(value);
        }
        if self.mode == AmpMode::Feedback {
            if let Some(inbox) = self.input.as_ref() {
                match inbox.recv() {
                    Ok(val) => return Some(val),
                    Err(_) => println!("Failed to read inbox"),
                }
            }
        }
        None
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
struct AmpController {
    amps: Vec<Amplifier>,
}

impl AmpController {
    fn new(prog: Program, seq: Sequence) -> AmpController {
        let amps = {
            let mut v = vec![];
            for s in seq {
                let mut c = Amplifier::new(prog.clone());
                c.with_phase(s);
                c.with_mode(AmpMode::Feedback);
                v.push(c);
            }
            for i in 0..v.len() - 1 {
                Self::chain_amps(&mut v, i, i + 1);
            }
            let last = v.len() - 1;
            Self::chain_amps(&mut v, last, 0);
            v
        };
        AmpController { amps }
    }

    fn chain_amps(amps: &mut Vec<Amplifier>, a: usize, b: usize) {
        let (output, input) = mpsc::channel();
        amps[a].set_output(output);
        amps[b].set_input(input);
    }

    fn run(mut self, inp: i32) -> i32 {
        let last = self.amps.len() - 1;
        let tx = self.amps[last].output.clone().unwrap();
        let (finaltx, finalrx) = mpsc::channel();
        self.amps[last].set_final(finaltx);
        let mut handles = vec![];
        for mut amp in self.amps {
            handles.push(std::thread::spawn(move || amp.run()));
        }
        tx.send(inp).unwrap();
        for handle in handles {
            handle.join().unwrap()
        }
        finalrx.recv().unwrap()
    }
}

fn main() {
    let program = include_str!("input7.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    let mut output = 0;

    for seq in combinations(vec![0, 1, 2, 3, 4]) {
        let res = AmpController::new(program.clone(), seq).run(0);
        if res > output {
            output = res;
        }
    }
    println!("Part one: {:?}", output);

    output = 0;
    for seq in combinations(vec![5, 6, 7, 8, 9]) {
        let res = AmpController::new(program.clone(), seq).run(0);
        if res > output {
            output = res;
        }
    }
    println!("Part two: {:?}", output);
}

/// wikipedia is one hell of a resource
fn combinations(mut a: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let n = a.len();
    let mut c = vec![0; n];
    res.push(a.clone());
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            res.push(a.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    res.iter()
        .map(|v| v.iter().map(|x| *x as i32).collect())
        .collect()
}

#[test]
fn test_amp_controller() {
    let progs = vec![
        (
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            vec![4, 3, 2, 1, 0],
            43210,
        ),
        (
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            vec![0, 1, 2, 3, 4],
            54321,
        ),
        (
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            vec![1, 0, 4, 3, 2],
            65210,
        ),
    ];
    for (prog, seq, res) in progs {
        assert_eq!(res, run_amp_controller(prog, seq));
    }
}

#[test]
fn test_controller() {
    let progs = vec![
        (
            vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ],
            vec![9, 8, 7, 6, 5],
            139629729,
        ),
        (
            vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ],
            vec![9, 7, 8, 5, 6],
            18216,
        ),
    ];
    for (prog, seq, res) in progs {
        let amc = AmpController::new(prog, seq);
        assert_eq!(res, amc.run(0));
    }
}
