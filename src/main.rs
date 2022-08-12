use std::io::{self};

#[derive(Clone, Copy, PartialEq)]
enum Commands {
    Add,
    Subtract,
    MoveL,
    MoveR,
    JumpL,
    JumpR,
    Input,
    Output,
    Outputln,
    Conditional,
    Pass
}

static mut TAPE: Vec<u8> = Vec::new();
static mut TAPE_PTR: usize = 0;
static mut IS_CONDITIONAL: bool = false;
fn main() {
    let mut code: String = String::new();

    io::stdin().read_line(&mut code).expect("Could not read code");
    let code = code.replace("\n", "");

    let mut cmds: Vec<Commands> = Vec::new();

    let mut dp: usize = 0;
    unsafe {
        TAPE = vec![0; 3000];
        for i in 0..4 {
            TAPE[i] = 1;
        }
    }
    
    for c in code.chars() {
        cmds.push(match c {
            '+' => Commands::Add,
            '-' => Commands::Subtract,
            '<' => Commands::MoveL,
            '>' => Commands::MoveR,
            '{' => Commands::JumpL,
            '}' => Commands::JumpR,
            ',' => Commands::Input,
            '.' => Commands::Output,
            '#' => Commands::Outputln,
            '!' => Commands::Conditional,
            _ => Commands::Pass
        });
    }

    cmds.retain(|&cmd| cmd != Commands::Pass);

    while dp < cmds.len() {
        unsafe {
            match cmds.get(dp).unwrap() {
                Commands::Add => {
                    if conditional_check() {
                        TAPE[TAPE_PTR] += TAPE[0];        
                    }
                    dp += 1;
                },
                Commands::Subtract => {
                    if conditional_check() {
                        TAPE[TAPE_PTR] = backarr(TAPE[TAPE_PTR], 0);
                    }
                    dp += 1;
                },
                Commands::MoveL => {
                    if conditional_check() {
                        TAPE_PTR = back(TAPE_PTR, 1);
                    }
                    dp += 1;
                },
                Commands::MoveR => {
                    if conditional_check() {
                        if TAPE_PTR + (TAPE[1] as usize) >= TAPE.len() {
                            for _j in 0..(TAPE_PTR + (TAPE[2] as usize) + 1 - TAPE.len()) {
                                TAPE.push(0);
                            }
                        }
                        TAPE_PTR += TAPE[2] as usize;
                    }
                    dp += 1;
                },
                Commands::JumpL => {
                    if conditional_check() {
                        dp = back(dp, 2); 
                    }
                },
                Commands::JumpR => {
                    if conditional_check() {
                        dp = if dp + (TAPE[2] as usize) < cmds.len() {
                            dp + (TAPE[2] as usize)
                        } else {
                            dp
                        };
                    }
                },
                Commands::Input => {
                    if conditional_check(){
                        let mut inp: String = String::new();
                        io::stdin().read_line(&mut inp).ok().expect("Could not read input");
                        TAPE[TAPE_PTR] = inp.chars().next().map(|char| char as u8).unwrap();
                    }
    
                    dp += 1;
                },
                Commands::Output => {
                    if conditional_check() {
                        print!("{}", TAPE[TAPE_PTR]);
                    }
                    dp += 1;
                },
                Commands::Outputln => {
                    if conditional_check() {
                        println!("{}", TAPE[TAPE_PTR] as char)
                    }
                    dp += 1;
                },
                Commands::Conditional => {
                    IS_CONDITIONAL = true;
                }
                _ => ()
            }
        }
    }
    
}

unsafe fn conditional_check() -> bool {
    let cond: bool = !IS_CONDITIONAL || (IS_CONDITIONAL && TAPE[TAPE_PTR] == TAPE[3]);
    IS_CONDITIONAL = false;
    return cond;
}

unsafe fn backarr(mut v: u8, i: usize) -> u8 {
    v = if v >= TAPE[i] {
        v - TAPE[i]
    } else {
        v
    };

    return v;
}

unsafe fn back(mut v: usize, i: usize) -> usize {
    v = if v >= (TAPE[i] as usize) {
        v - (TAPE[i] as usize)
    } else {
        v
    };

    return v;
}
