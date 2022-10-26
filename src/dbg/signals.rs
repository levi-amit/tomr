use super::*;

use nix::libc::{
    siginfo_t, uid_t, clock_t,
    CLD_TRAPPED, CLD_EXITED,
};
use signal_hook::{
    iterator::{SignalsInfo, exfiltrator::WithRawSiginfo},
    consts::{SIGINT, SIGCHLD},
};


enum SigInfo {
    SIGCHLD { si_signo: i32, si_errno: i32, si_code: i32, si_pid: i32, si_status: i32, si_uid: uid_t, si_utime: clock_t, si_stime: clock_t },
    SIGINT  { si_signo: i32, si_errno: i32, si_code: i32, si_pid: i32, si_uid: uid_t },
}

impl SigInfo {
    pub fn new<T: Into<Self>>(siginfo: T) -> Self {
        siginfo.into()
    }
}

impl From<siginfo_t> for SigInfo {
    fn from(siginfo: siginfo_t) -> Self {
        // underlying siginfo_t is a C union, meaning access to some fields is unsafe - 
        // we must ensure si_signo is one which is really supposed to have any field in the union.
        // Check the man page for `sigaction` for a full description of which fields every signal fills in.
        match siginfo.si_signo {
            SIGCHLD => {
                SigInfo::SIGCHLD {
                    si_signo:   siginfo.si_signo,
                    si_code:    siginfo.si_code,
                    si_errno:   siginfo.si_errno,
                    si_pid:     unsafe { siginfo.si_pid() },
                    si_status:  unsafe { siginfo.si_status() },
                    si_uid:     unsafe { siginfo.si_uid() },
                    si_utime:   unsafe { siginfo.si_utime() },
                    si_stime:   unsafe { siginfo.si_utime() },
                }
            }
            SIGINT => {
                SigInfo::SIGINT { 
                    si_signo:   siginfo.si_signo,
                    si_errno:   siginfo.si_code, 
                    si_code:    siginfo.si_errno,
                    si_pid:     unsafe { siginfo.si_pid() },
                    si_uid:     unsafe { siginfo.si_uid() },
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}


/// Starts a new thread for signal handling
pub fn setup_signal_handlers() -> Result<(), Error> {
    let mut signals: SignalsInfo<WithRawSiginfo> = SignalsInfo::<WithRawSiginfo>::new(&[SIGINT, SIGCHLD])
        .expect("Could not set up signal iterator through signal-hook");

    thread::spawn(move || {
        for siginfo in signals.forever() {
            match SigInfo::from(siginfo) {
                SigInfo::SIGCHLD { si_signo, si_errno, si_code, si_pid, si_status, si_uid, si_utime, si_stime } => {
                   handle_sigchld(si_signo, si_errno, si_code, si_pid, si_status, si_uid, si_utime, si_stime) 
                }
                SigInfo::SIGINT { si_signo, si_errno, si_code, si_pid, si_uid } => {
                    handle_sigint(si_signo, si_errno, si_code, si_pid, si_uid)
                }
            }
        }
    });
    
    Ok(())
}


/// Handles all received SIGCHLD
/// NOT FINISHED, SHOULD NOT PRINT DIRECTLY FROM dbg MODULE
fn handle_sigchld(_si_signo: i32, _si_errno: i32, si_code: i32, si_pid: i32, si_status: i32, _si_uid: uid_t, _si_utime: clock_t, _si_stime: clock_t) {
    // TODO: replace panics w/ something which'll kill the main thread as well

    // determine signaling child debugee
    let dbgee = DEBUGEES.read().unwrap()
        .from_pid(Pid::from_raw(si_pid))
        .expect("Non-debugee process sent SIGCHLD, currently unhandled")
        .clone();

    match si_code {
        CLD_TRAPPED => {
            println!("\nDebugee {} (PID {}) was trapped (status {})", dbgee.dbgid, dbgee.pid, si_status);
        }
        CLD_EXITED => {
            println!("\nDebugee {} (PID {}) has exited (code {})", dbgee.dbgid, dbgee.pid, si_status);
            DEBUGEES.write().unwrap().remove(dbgee.dbgid).ok();
        }
        _ => {
            println!("\nDebugee {:?} sent SIGCHLD", dbgee);
        }
    }
}


fn handle_sigint(_si_signo: i32, _si_errno: i32, _si_code: i32, _si_pid: i32, _si_uid: uid_t) {
    unimplemented!();
}