use nix::{
    unistd::{Pid, fork, ForkResult, execvpe},
    sys::{ptrace}, 
    errno::Errno,
};
use lazy_static::lazy_static;

use std::{
    sync::Mutex,
    vec::Vec,
    ffi::CString,
};

/// Error originating from tomr's dbg module's API
#[derive(Debug)]
pub enum Error {
    UnixError { errno: Errno },
}


#[derive(Debug)]
struct Debugee {
    pid: Pid,
    origin: DebugeeOrigin,
}

#[derive(Debug)]
enum DebugeeOrigin {
    Spawned,
    Attached,
}

lazy_static! {
    static ref DEBUGEES: Mutex<Vec<Debugee>> = Mutex::new(Vec::new());
}


/// Creates a new traced process, and sets it as the active process
pub fn spawn(path: &str, args: &[&str], env: &[&str]) -> Result<Pid, Error> {
    // Since we're about to call nix functions,
    // we need to convert our string slices to CStrings
    let path = CString::new(path)
        .expect("Error: path passed to `spawn` must be convertible to CStrings.");
    let args: Vec<CString> = args.iter()
        .map(|arg| CString::new(*arg)
            .expect("Error: args passed to `spawn` must be convertible to CStrings."))
        .collect();
    let env: Vec<CString> = env.iter()
        .map(|var| CString::new(*var)
            .expect("Error: env passed to `spawn` must be convertible to CStrings."))
        .collect();

    match unsafe { fork() } {
        // fork successful, update DEBUGEES with the new child's details
        Ok(ForkResult::Parent { child }) => {
            DEBUGEES.lock().unwrap()
                .push(Debugee {
                    pid: child,
                    origin: DebugeeOrigin::Spawned,
                });

            return Ok(child);
        }

        // fork successful, we are the child process. now traceme and exec!
        Ok(ForkResult::Child) => {
            ptrace::traceme().ok();
            execvpe(&path, &args, &env)
                .expect("Child Error: exec failed");
            unreachable!();
        }

        // fork failed
        Err(errno) => {
            return Err(Error::UnixError { errno })
        }
    }
}