#![allow(dead_code)]

mod signals;

use nix::{
    unistd::{Pid, fork, ForkResult, execvpe},
    sys::{ptrace}, 
    errno::Errno,
};
use lazy_static::lazy_static;
use signal_hook::{
    iterator::{SignalsInfo, exfiltrator::WithOrigin},
    consts::{SIGINT, SIGCHLD},
    low_level::siginfo,
};

use std::{
    sync::RwLock,
    vec::Vec,
    ffi::CString,
    thread,
};

/// Identifier for debugees
pub type Dbgid = i32;


/// Errors originating from tomr's dbg module's API
#[derive(Debug)]
pub enum Error {
    UnixError { errno: Errno },
    NoSuchDebugee,
}


#[derive(Debug, Clone)]
pub struct Debugees {
    // TODO: Convert this to a HashMap with Dbgid as key
    vec: Vec<Debugee>,
}

impl Debugees {

    /// Creates a new Debugees struct with a new owned Vec
    fn new() -> Debugees {
        Debugees {
            vec: Vec::new(),
        }
    }

    /// Extends the debugees vector with a new Debugee struct, having a generated dbgid
    fn add(&mut self, pid: Pid, origin: DebugeeOrigin) -> Result<&Debugee, ()> {
        // iterate DEBUGEES vector to find lowest unused dbgid
        let mut dbgid = 0;
        for dbgee in self.vec.iter() {
            if dbgee.dbgid == dbgid { dbgid += 1; }
        }

        // push new Debugee with the generated dbgid to self
        self.vec.push(Debugee {
            dbgid,
            pid,
            origin,
        });

        // return a reference to the pushed Debugee struct
        Ok(&self.vec[self.vec.len() - 1])
    }

    /// Removes a debugee from this struct's listing
    fn remove(&mut self, dbgid: Dbgid) -> Result<(), Error> {
        let index = self.vec.iter()
            .position(|dbgee| dbgee.dbgid == dbgid)
            .ok_or(Error::NoSuchDebugee)?;
        
        self.vec.remove(index);
        Ok(())
    }

    fn from_dbgid(&self, dbgid: Dbgid) -> Result<&Debugee, Error> {
        for dbgee in self.vec.iter() {
            if dbgee.dbgid == dbgid { return Ok(dbgee); }
        }
        Err(Error::NoSuchDebugee)
    }

    fn from_pid(&self, pid: Pid) -> Result<&Debugee, Error> {
        for dbgee in self.vec.iter() {
            if dbgee.pid == pid { return Ok(dbgee); }
        }
        Err(Error::NoSuchDebugee)
    }

}

#[derive(Debug, Clone)]
pub struct Debugee {
    pub dbgid: Dbgid,
    pub pid: Pid,
    pub origin: DebugeeOrigin,
}

#[derive(Debug, Clone)]
pub enum DebugeeOrigin {
    Spawned,
    Attached,
}


lazy_static! {
    /// Global state of debugged processes - 
    /// Every debugged process has a `Debugee` struct entry here
    static ref DEBUGEES: RwLock<Debugees> = RwLock::new(Debugees::new());
}


/// Sets up the required environment for debugging functionalities to work, comprising:
/// - Setting the debugger's signal handlers for the current process
pub fn setup_dbg() {
    signals::setup_signal_handlers().ok();
}


/// Creates a new traced process, and sets it as the active process
pub fn spawn(path: &str, args: &[&str], env: &[&str]) -> Result<Debugee, Error> {
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
            let mut dbgees_guard = DEBUGEES.write().unwrap();
            let dbgee = dbgees_guard.add(child, DebugeeOrigin::Spawned)
                .expect("Error: Could not add Debugee to DEBUGEES");
            return Ok(dbgee.clone());
        }

        // fork successful, we are the child process. now traceme and exec!
        Ok(ForkResult::Child) => {
            ptrace::traceme().ok();
            match execvpe(&path, &args, &env) {
                Err(errno) => {
                    std::process::exit(errno as i32)
                }
                // Ok case is when exec succeeded, therefore this is unreachable
                _ => unreachable!()
            }
        }

        // fork failed
        Err(errno) => {
            return Err(Error::UnixError { errno })
        }
    }
}


/// Returns a cloned copy of the static DEBUGEES Debugees struct
pub fn debugees() -> Result<Debugees, Error> {
    Ok(DEBUGEES.read().unwrap().clone())
}


// Continues the execution of a debugee
pub fn cont(dbgid: Dbgid) -> Result<(), Error> {
    // resolve pid of process from dbgid, or return Error if dbgid is not a debugee
    let pid = DEBUGEES.read().unwrap().from_dbgid(dbgid)?.pid;

    // call ptrace cont for the found PID, or returns UnixError with Errno on ptrace failure
    ptrace::cont(pid, None)
        .or_else(|errno| Err(Error::UnixError { errno: errno }))?;

    Ok(())
}


