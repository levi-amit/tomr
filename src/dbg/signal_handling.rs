pub use nix::libc::{
    CLD_TRAPPED, CLD_EXITED,
    uid_t, clock_t,
};

use super::*;
use nix::libc::siginfo_t;
use signal_hook::{
    iterator::{SignalsInfo, exfiltrator::WithRawSiginfo},
    consts::{SIGINT, SIGCHLD},
};


pub enum SigInfo {
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
        // Check the man page for `sigaction` for a full description of which fields of the union every signal fills in.
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


/// Type describing an external signal handling function - receiving a SigInfo and returning nothing.
pub type SignalHandler = fn(&SigInfo) -> ();


lazy_static! {
    /// The global list of external modules' signal handler functions
    static ref SIGNAL_HANDLERS: RwLock<Vec<SignalHandler>> = RwLock::new(Vec::new());
}


/// # SIGNAL HANDLING MODEL
/// While there are some necessary actions to be taken upon reception of some signals (such as removing a Debugee member upon a child's death),
/// a debugger UI might want to make more stuff happen upon signal reception, like printing an alert to terminal or opening an alert window.
/// To enable this, upon signal reception two types of functions will be called:
/// - This module's signal handling function corresponding to the received signal, called `handle_sig*` where `*` is whatever
/// - A set of signal handling functions receiving a SigInfo enum which should do the handling needed for the external module's thingy.
/// 
/// To add an external signal handler function, append to the static `SIGNAL_HANDLERS` vector.


/// Starts a new thread for signal handling
pub (in super) fn setup_signal_handlers() -> Result<(), Error> {
    let mut signals: SignalsInfo<WithRawSiginfo> =
        SignalsInfo::<WithRawSiginfo>::new(&[SIGINT, SIGCHLD])
        .expect("Could not set up signal iterator through signal-hook");

    thread::spawn(move || {
        for siginfo in signals.forever() {
            let siginfo = SigInfo::from(siginfo);
            SIGNAL_HANDLERS.read().unwrap().iter()
                .for_each(|handler| handler(&siginfo));
            main_signal_handler(&siginfo);
        }
    });
    
    Ok(())
}


fn main_signal_handler(siginfo: &SigInfo) -> () {
    // TODO: replace panics w/ something which'll kill the main thread as well

    match siginfo {
        SigInfo::SIGCHLD { si_code, si_pid, .. } => {
            // determine signaling child debugee
            let dbgee = DEBUGEES.read().unwrap()
                .by_pid(Pid::from_raw(*si_pid))
                .expect("Non-debugee process sent SIGCHLD")
                .clone();

            match si_code {
                &CLD_EXITED => {
                    DEBUGEES.write().unwrap().remove(dbgee.dbgid).ok();
                }
                _ => {}
            }
        }
        SigInfo::SIGINT { .. } => {
            unimplemented!()
        }
    }
}


/// Add a new signal handling function to be called for each received signal.
/// The added function is called in a unique thread used only for signal handling.
pub fn add_signal_handler(f: SignalHandler) {
    SIGNAL_HANDLERS.write().unwrap().push(f);
}
