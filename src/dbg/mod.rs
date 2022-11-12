#![allow(dead_code)]

pub mod signal_handling;

use nix::{
    unistd::{fork, ForkResult, execvpe},
    sys::{ptrace, signal::{Signal}},
    errno::Errno,
};
pub use nix::unistd::{
    Pid,
};
use lazy_static::lazy_static;

use std::{
    sync::{RwLock},
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
    CStringConversionError {index: isize},
    NoSuchDebugee,
    NoSuchProcess,
}


#[derive(Debug, Clone)]
pub struct DebugeeList {
    vec: Vec<Debugee>,
}

impl DebugeeList {

    /// Creates a new Debugees struct with a new owned Vec
    fn new() -> Self {
        DebugeeList {
            vec: Vec::new(),
        }
    }

    /// Returns an unused dbgid - this is used to determine the dbgid of newly added debugees.
    fn get_free_dbgid(&self) -> Dbgid {
        // iterate DEBUGEES vector to find lowest unused dbgid
        let mut dbgid = 0;
        for dbgee in self.vec.iter() {
            if dbgee.dbgid == dbgid { dbgid += 1; }
        }
        dbgid
    }

    /// Adds a new `Debugee` struct with a new generated unique `dbgid`.
    fn add(&mut self, mut dbgee: Debugee) -> Result<&Debugee, Error> {
        // push new Debugee with an unused generated dbgid to the Debugees vec
        dbgee.dbgid = self.get_free_dbgid();
        self.vec.push(dbgee);

        // return a reference to the pushed Debugee struct
        Ok(self.vec.last().expect("Unexpected empty DebugeeList vector"))
    }

    /// Removes a debugee from this debugee listing, and return the removed Debugee struct
    fn remove(&mut self, dbgid: Dbgid) -> Result<Debugee, Error> {
        let index = self.vec.iter()
        .position(|dbgee| dbgee.dbgid == dbgid)
        .ok_or(Error::NoSuchDebugee)?;
        
        Ok(self.vec.remove(index))
    }

    pub fn by_dbgid(&self, dbgid: Dbgid) -> Option<&Debugee> {
        self.vec.iter().find(|&dbgee| dbgee.dbgid == dbgid)
    }

    pub fn by_pid(&self, pid: Pid) -> Option<&Debugee> {
        self.vec.iter().find(|&dbgee|dbgee.pid == pid)
    }

}

// impl<'a> IntoIterator for &'a DebugeeList {
//     type Item = &'a Debugee;
//     type IntoIter = std::vec::IntoIter<&'a Debugee>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.vec.into_iter()
//     }
// }


lazy_static! {
    /// Global state of debugged processes -
    /// Every debugged process has a `Debugee` struct entry here
    static ref DEBUGEES: RwLock<DebugeeList> = RwLock::new(DebugeeList::new());
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


/// Sets up the required environment for debugging functionalities to work, comprising:
/// - Setting the debugger's signal handlers for the current process
pub fn setup_dbg() {
    signal_handling::setup_signal_handlers().ok();
}


/// Creates a new traced process from an executable path and argv
pub fn spawn(path: &str, args: &[&str], env: &[&str]) -> Result<Debugee, Error> {
    /// Errorless conversion to CString -
    /// Any characters after the first internal null byte in a supplied string (if there exists one) will be ignored.
    fn to_cstring(s: &str) -> CString {
        CString::new(&s[..s.find('\0').unwrap_or(s.len())])
        .expect("Unexpected error converting to CString")
    }

    let path = to_cstring(path);
    let args: Vec<CString> = args.iter().map(|arg| to_cstring(*arg)).collect();
    let env: Vec<CString> = env.iter().map(|var| to_cstring(*var)).collect();

    // get a hold of the lock guard before any child process is born,
    // to prevent the signal handling thread from attempting to remove the Debugee from DEBUGEES
    // upon an early child death. 
    let mut dbgees_guard = DEBUGEES.write().unwrap();

    match unsafe { fork() } {
        // fork successful, update DEBUGEES with the new child's details
        Ok(ForkResult::Parent { child }) => {
            let dbgee = Debugee {
                dbgid:  Dbgid::default(),
                pid:    child,
                origin: DebugeeOrigin::Spawned,
            };
            let dbgee = dbgees_guard.add(dbgee)
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

// TODO: Consider making this fallible and use `try_read` instead to prevent external locking for long periods?
/// Performs an operation on an immutable view of the global DebugeeList
pub fn with_debugees<F, T>(f: F) -> T
where F: FnOnce(&DebugeeList) -> T {
    let debugees_guard = DEBUGEES.read().unwrap();
    f(&debugees_guard)
}


// Continues the execution of a debugee
pub fn cont(dbgid: Dbgid) -> Result<(), Error> {
    // resolve pid of process from dbgid, or return Error if dbgid is not a debugee
    let pid = DEBUGEES.read().unwrap()
    .by_dbgid(dbgid).ok_or(Error::NoSuchDebugee)?.pid;

    // call ptrace cont for the found PID, or returns UnixError with Errno on ptrace failure
    ptrace::cont(pid, None)
        .or_else(|errno| {
            match errno {
                Errno::ESRCH => {
                    Err(Error::NoSuchProcess)
                }
                _ => {
                    Err(Error::UnixError { errno })
                }
            }
        })?;

    Ok(())
}


/// Send a signal to a debugee by a dbgid
pub fn send_signal(dbgid: Dbgid, signal: Signal) -> Result<(), Error> {
    let pid = DEBUGEES.read().unwrap()
    .by_dbgid(dbgid).ok_or(Error::NoSuchDebugee)?.pid;

    nix::sys::signal::kill(pid, signal)
    .or_else(|e| Err(Error::UnixError { errno: e }))?;
    Ok(())
}


// /// Exit routine for the dbg module:
// /// - Kill all spawned debugees
// /// - Detach from all attached debugees
// pub fn exit() -> Result<(), Error> {
//     DEBUGEES.read().unwrap().

//     Ok(())
// }