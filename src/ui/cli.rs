use std::{
    process::exit,
    io::Write,
};

use crate::dbg;


pub fn start_cli() -> Result<(), ()> {
    loop {
        let input = prompt_for_line("tomr# ");
        handle_input(input);
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


fn handle_input(input: String) {
    let mut words = input.splitn(2, ' ');
    match words.next() {
        Some("help") => { 
            println!("This is tomr the debugger. \n\
            help - show this message \n\
            exit - terminate this process")
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
            let path: &str = args.next().unwrap();
            let env: Vec<&str> = Vec::new();
            
            // Create vector of &str and store as argv
            let mut argv: Vec<&str> = vec![path];
            if let Some(argv_str) = args.next() {
                argv.extend(argv_str.split(' '));
            }

            dbg::spawn(path, &argv, &env).unwrap();
        }

        Some("attach") => { unimplemented!() }
        Some("info") => { unimplemented!() }

        None | Some("") => {}
        Some(unknown) => {
            println!("unknown command {}", unknown)
        }
    }
}
