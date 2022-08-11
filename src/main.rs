use std::io::{self};

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

static mut TAPE: Vec<usize> = Vec::new();
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
        for i in 0..5 {
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

    for cmd in 0..cmds.len() {
        if let Commands::Pass = cmds.get(cmd).unwrap() {
            cmds.remove(cmd);
        }
    }

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
                        TAPE[TAPE_PTR] = back(TAPE[TAPE_PTR], 1);
                    }
                    dp += 1;
                },
                Commands::MoveL => {
                    if conditional_check() {
                        TAPE_PTR = back(TAPE_PTR, 2);
                    }
                    dp += 1;
                },
                Commands::MoveR => {
                    if conditional_check() {
                        if TAPE_PTR + TAPE[2] >= TAPE.len() {
                            for j in 0..(TAPE_PTR + TAPE[2] + 1 - TAPE.len()) {
                                TAPE.push(0);
                            }
                        }
                        TAPE_PTR += TAPE[2];
                    }
                    dp += 1;
                },
                Commands::JumpL => {
                    if conditional_check() {
                        dp = back(dp, 3); 
                    }
                },
                Commands::JumpR => {
                    if conditional_check() {
                        dp = if dp + TAPE[3] < cmds.len() {
                            dp + TAPE[3]
                        } else {
                            dp
                        };
                    }
                },
                Commands::Input => {
                    if conditional_check(){
                        let mut inp: String = String::new();
                        io::stdin().read_line(&mut inp).ok().expect("Could not read input");
                        TAPE[TAPE_PTR] = inp.trim().parse().expect("Only number inputs accepted.");
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
                        println!("{}", TAPE[TAPE_PTR])
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
    let cond: bool = !IS_CONDITIONAL || (IS_CONDITIONAL && TAPE[TAPE_PTR] == TAPE[4]);
    IS_CONDITIONAL = false;
    return cond;
}

unsafe fn back(mut v: usize, i: usize) -> usize {
    v = if v >= TAPE[i] {
        v - TAPE[i]
    } else {
        v
    };

    return v;
}