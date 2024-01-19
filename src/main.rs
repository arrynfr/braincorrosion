use std::{fs, env, io::{Read, BufRead, Write}};

const MEMORY_SIZE: usize = 30000;

enum LoopOp {
    Open,
    Close
}

#[derive(Debug)]
struct BrainfuckMachine {
    program: Vec<u8>,
    instruction_pointer: usize,
    data_pointer: usize,
    memory: [u8; MEMORY_SIZE]
}

impl BrainfuckMachine {
    fn init(file_path: &str) -> Option<Self> {
        let program = fs::read_to_string(file_path)
                                .expect(format!("Error reading file {file_path}").as_str())
                                .into_bytes();
        let instruction_pointer: usize = 0;
        let data_pointer: usize = 0;
        let memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        let bf = Self { program, instruction_pointer, data_pointer, memory};
        if bf.is_valid_program() {
            Some(bf)
        } else {
            None
        }
    }

    fn run(&mut self) { 
        while self.instruction_pointer < self.program.len() {
            match self.program[self.instruction_pointer] {
                b'>' => { self.data_pointer = (self.data_pointer + 1) % MEMORY_SIZE; },
                b'<' => { self.data_pointer = self.data_pointer.checked_sub(1).or(Some(MEMORY_SIZE-1)).unwrap(); },
                b'+' => { self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_add(1) },
                b'-' => { self.memory[self.data_pointer] = self.memory[self.data_pointer].wrapping_sub(1) },
                b'.' => self.output(),
                b',' => self.input(),
                b'[' => self.handle_loop(&LoopOp::Open),
                b']' => self.handle_loop(&LoopOp::Close),
                _ =>    {}
            }
            self.instruction_pointer += 1;
        }
    }

    fn output(&self) {
        print!("{}", self.memory[self.data_pointer] as char);
        let _ = std::io::stdout().flush();
    }

    fn input(&mut self) {
        let mut buf:[u8; 1] = [0];
        let _ = std::io::stdin().lock().read_exact(&mut buf);
        let amount = std::io::stdin().lock().fill_buf().unwrap().len();
        std::io::stdin().lock().consume(amount);
        self.memory[self.data_pointer] = buf[0];
    }

    fn handle_loop(&mut self, operation: &LoopOp) {
        let params = match operation {
            LoopOp::Open => {   (b']', 
                                b'[', 
                                u8::eq as fn(&u8, &u8) -> bool,
                                usize::wrapping_add as fn(usize, usize) -> usize)
                            },
            LoopOp::Close => {      (b'[', 
                                    b']', 
                                    u8::ne as fn(&u8, &u8) -> bool,
                                    usize::wrapping_sub as fn(usize, usize) -> usize)
            }
        };

        if params.2(&self.memory[self.data_pointer], &0) {
            while self.program[self.instruction_pointer] != params.0 {
                self.instruction_pointer = params.3(self.instruction_pointer, 1);
                if self.program[self.instruction_pointer] == params.1 {
                    self.handle_loop(operation);
                    self.instruction_pointer = params.3(self.instruction_pointer, 1);
                }
            }
        }
    }

    fn _handle_loop_open(&mut self) {
        if self.memory[self.data_pointer] == 0 {
            while self.program[self.instruction_pointer] != b']' {
                self.instruction_pointer += 1;
                if self.program[self.instruction_pointer] == b'[' {
                    self._handle_loop_open();
                    self.instruction_pointer += 1;
                }
            }
        }
    }

    fn _handle_loop_close(&mut self) {
        if self.memory[self.data_pointer] != 0 {
            while self.program[self.instruction_pointer] != b'[' {
                self.instruction_pointer -= 1;
                if self.program[self.instruction_pointer] == b']' {
                    self._handle_loop_close();
                    self.instruction_pointer -= 1;
                }
            }
        }
    }

    fn is_valid_program(&self) -> bool {
        let obc = self.program.iter().filter(|&&c| c == '[' as u8).count();
        let cbc = self.program.iter().filter(|&&c| c == ']' as u8).count();
        obc == cbc
    } 

    fn _single_step(&self) {
        println!{   "IP: {}; Inst: {}; DP: {}; Mem: {:#x} ({})", 
                    self.instruction_pointer,
                    self.program[self.instruction_pointer] as char,
                    self.data_pointer,
                    self.memory[self.data_pointer],
                    self.memory[self.data_pointer] as char
        };
        let mut s = String::from(""); 
        let _ = std::io::stdin().lock().read_line(&mut s);
    }
}

fn main() -> Result<(),()> {
    let args: Vec<String> = env::args().collect();
    let mut bf =    BrainfuckMachine::init(args[1].as_str())
                                        .expect("There are unmatched braces in your program!");
    bf.run();
    Ok(())
}
