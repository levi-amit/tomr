use std::{
    process::exit,
    io::Write,
};

use crate::dbg;


pub fn start_cli() -> Result<(), ()> {
    loop {
        let input = prompt_for_line("tomr# ");
        handle_command_line(input);
    }

    #[allow(unreachable_code)]
    Ok(())
}


fn prompt_for_line(prompt: &str) -> String {
    let mut line = String::new();

    print!("{}", prompt);

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line)
        .expect("Error: Could not read_line");
    
    line.trim().to_string()
}


/// Performs actions based on a command string
fn handle_command_line(input: String) {
    // the first word (text until the first whitespace) is taken as the command,
    // the rest of the string is taken as the command argument(s).
    let mut words = input.splitn(2, ' ');
    match words.next() {
        Some("help") => { 
            println!("This is tomr the debugger. \n\
            help - show this message \n\
            exit - terminate this process \n\
            spawn <exec_path> [args ...] - create a new debugged process \n\
            info - show info about currently debugged processes \
            ");
        }
        Some("exit") => { 
            exit(0);
        }

        Some("spawn") => {
            let argstr = words.next();
            if argstr.is_none() {
                println!("no arguments supplied. correct usage is:\n\
                spawn <executable> [argv1 [...]]");
                return;
            }

            let mut args = argstr.unwrap().split(' ');
            // the path is the first arg to the command
            let path: &str = args.next().unwrap();
            // for the moment no env variable settings are available
            let env: Vec<&str> = Vec::new();
            
            // Create vector of &str and store as argv
            let mut argv: Vec<&str> = vec![path];
            if let Some(argv_str) = args.next() {
                argv.extend(argv_str.split(' '));
            }

            dbg::spawn(path, &argv, &env).unwrap();
        }

        Some("attach") => { unimplemented!() }

        Some("info") => {
            println!("Debugged Processes:\n{:?}", dbg::debugees());
        }

        Some("continue") => {}
        Some("freeze") => {}

        None | Some("") => {}
        Some(unknown) => {
            println!("unknown command {}", unknown);
        }
    }
}
