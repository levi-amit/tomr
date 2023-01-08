mod signal_handling;

pub use nix::unistd::Pid;
pub use signal_handling::*;

use nix::{
    unistd::{fork, ForkResult, execvpe},
    sys::{ptrace, signal::Signal},
    errno::Errno,
};
use lazy_static::lazy_static;
use std::{
    sync::{RwLock},
    vec::Vec,
    ffi::CString,
    thread,
    fmt::{Debug, Display, Formatter},
};


/// Identifier for debugees
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Dbgid(i32);

impl Display for Dbgid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl Default for Dbgid {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i32> for Dbgid {
    fn from(dbgid: i32) -> Self {
        Self(dbgid)
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Origin {
    Spawned,
    Attached,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Debugee {
    pub dbgid: Dbgid,
    pub pid: Pid,
    pub origin: Origin,
}


/// Errors originating from tomr's dbg module's API
#[derive(Debug)]
pub enum Error {
    Errno { errno: Errno },
    NoSuchDebugee,
    NoSuchProcess,
}


pub trait Debugger {
    
    fn get_debugee(&self) -> Result<Debugee, Error>;
    fn cont(&self) -> Result<(), Error>;
    fn send_signal(&self, signal: Signal) -> Result<(), Error>;

}


impl Debugger for Debugee {

    fn get_debugee(&self) -> Result<Debugee, Error> {
        Ok(*self)
    }

    fn cont(&self) -> Result<(), Error> {
        ptrace::cont(self.pid, None)
        .or_else(|errno| {
            match errno {
                Errno::ESRCH => {
                    Err(Error::NoSuchProcess)
                }
                _ => {
                    Err(Error::Errno { errno })
                }
            }
        })?;

        Ok(())
    }

    fn send_signal(&self, signal: Signal) -> Result<(), Error> {
        nix::sys::signal::kill(self.pid, signal)
            .or_else(|e| Err(Error::Errno { errno: e }))?;
        
        Ok(())
    }

}


impl Debugger for Dbgid {

    fn get_debugee(&self) -> Result<Debugee, Error> {
        Ok(DEBUGEES.read().unwrap()
                .by_dbgid(*self).ok_or(Error::NoSuchDebugee)?
                .clone())
    }

    fn cont(&self) -> Result<(), Error> {
        self.get_debugee()?.cont()
    }

    fn send_signal(&self, signal: Signal) -> Result<(), Error> {
        self.get_debugee()?.send_signal(signal)
    }

}


impl Debugger for Pid {

    fn get_debugee(&self) -> Result<Debugee, Error> {
        Ok(DEBUGEES.read().unwrap()
                .by_pid(*self).ok_or(Error::NoSuchDebugee)?
                .clone())
    }

    fn cont(&self) -> Result<(), Error> {
        self.get_debugee()?.cont()
    }

    fn send_signal(&self, signal: Signal) -> Result<(), Error> {
        nix::sys::signal::kill(*self, signal)
            .or_else(|e| Err(Error::Errno { errno: e }))?;
        
        Ok(())
    }

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
        let mut dbgid: i32 = 0;
        for dbgee in self.vec.iter() {
            if dbgee.dbgid == dbgid.into() { dbgid += 1; }
        }
        dbgid.into()
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


lazy_static! {
    static ref DEBUGEES: RwLock<DebugeeList> = RwLock::new(DebugeeList::new());
}


/// Sets up the required environment for debugging functionalities to work, comprising:
/// - Setting the debugger's signal handlers for the current process
pub fn setup_dbg() {
    signal_handling::setup_signal_handlers().ok();
}


/// Performs an operation on an immutable view of the global DebugeeList
pub fn with_debugees<F, T>(f: F) -> T
where F: FnOnce(&DebugeeList) -> T {
    let debugees_guard = DEBUGEES.read().unwrap();
    f(&debugees_guard)
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
                origin: Origin::Spawned,
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
            return Err(Error::Errno { errno })
        }
    }
}


/// Attaches to a running process
pub fn attach(pid: Pid) -> Result<Debugee, Error> {
    ptrace::attach(pid).map_err(|errno| Error::Errno { errno })?;
    let attached = Debugee {
        dbgid: 0.into(),
        origin: Origin::Attached,
        pid,
    };
    Ok(DEBUGEES.write().unwrap().add(attached)?.clone())
}
