#![allow(dead_code)]


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

/// Identifier for debugees
#[derive(Debug, Clone, PartialEq)]
pub struct Dbgid(i32);

/// Error originating from tomr's dbg module's API
#[derive(Debug)]
pub enum Error {
    UnixError { errno: Errno },
}


#[derive(Debug, Clone)]
pub struct Debugees {
    vec: Vec<Debugee>,
}

impl Debugees {
    fn new() -> Debugees {
        Debugees {
            vec: Vec::new(),
        }
    }

    /// Extend the debugees vector with a new Debugee struct, having a generated dbgid
    fn add(&mut self, pid: Pid, origin: DebugeeOrigin) -> Result<(), ()> {
        // iterate DEBUGEES vector to find lowest unused dbgid
        let mut dbgid = 0;
        for dbgee in self.vec.iter() {
            if dbgee.dbgid == Dbgid(dbgid) { dbgid += 1; }
        }

        // push new Debugee with the generated dbgid to self
        self.vec.push(Debugee {
            dbgid: Dbgid(dbgid),
            pid,
            origin,
        });

        Ok(())
    }

    fn from_dbgid(&self, dbgid: Dbgid) -> Option<&Debugee> {
        for dbgee in self.vec.iter() {
            if dbgee.dbgid == dbgid { return Some(dbgee) }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Debugee {
    dbgid: Dbgid,
    pid: Pid,
    origin: DebugeeOrigin,
}


#[derive(Debug, Clone)]
pub enum DebugeeOrigin {
    Spawned,
    Attached,
}

lazy_static! {
    static ref DEBUGEES: Mutex<Debugees> = Mutex::new(Debugees::new());
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
            DEBUGEES.lock().unwrap().add(child, DebugeeOrigin::Spawned).ok();
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


/// Returns a cloned copy of the static DEBUGEES vector
pub fn debugees() -> Result<Debugees, Error> {
    Ok(DEBUGEES
        .lock()
        .unwrap()
        .clone())
}


// Continues the execution of a debugee
// pub fn cont(dbgid: Dbgid) -> Result<(), Error> {
//     //// let pid = DEBUGEES.lock().unwrap().;

//     Ok(())
// }