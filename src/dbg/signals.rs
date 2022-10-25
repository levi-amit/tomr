use super::*;



use nix::libc::{
    siginfo_t,
    CLD_TRAPPED, CLD_EXITED,
};
use signal_hook::{
    iterator::{SignalsInfo, exfiltrator::WithRawSiginfo},
    consts::{SIGINT, SIGCHLD},
};


/// Starts a new thread for signal handling
pub fn setup_signal_handlers() -> Result<(), Error> {
    let mut signals: SignalsInfo<WithRawSiginfo> = SignalsInfo::<WithRawSiginfo>::new(&[SIGINT, SIGCHLD])
        .expect("Could not set up signal iterator through signal-hook");

    thread::spawn(move || {
        for siginfo in signals.forever() {
            match siginfo.si_signo {
                SIGCHLD => { handle_sigchld(siginfo); }
                SIGINT => { handle_sigint(siginfo); }
                _ => {}
            }
        }
    });
    
    Ok(())
}


/// Handles all received SIGCHLD
/// NOT FINISHED, SHOULD NOT PRINT DIRECTLY FROM dbg MODULE
fn handle_sigchld(siginfo: siginfo_t) {
    // TODO: Replace panics with smth else?
    // TODO: Make this into a nice rusty enum
    // underlying siginfo_t is a C union, meaning access to some fields is unsafe - 
    // we must ensure si_signo is one which is really supposed to have this field in the instance of the union.
    // Check the man page for `sigaction` for a full description of which fields every signal fills in.
    if siginfo.si_signo != SIGCHLD { 
        panic!("Incorrect signal handler called! handle_sigchld was called for a non SIGCHLD signal.");
    }
    #[allow(unused_variables)]
    let (si_pid, si_status, si_uid, si_utime, si_stime) = unsafe {
        (siginfo.si_pid(), siginfo.si_status(), siginfo.si_uid(), siginfo.si_utime(), siginfo.si_stime())
    };
    
    // determine signaling child debugee
    let dbgee = DEBUGEES.read().unwrap()
        .from_pid(Pid::from_raw(si_pid))
        .expect("Non-debugee process sent SIGCHLD, currently unhandled")
        .clone();

    match siginfo.si_code {
        CLD_TRAPPED => {
            println!("\nDebugee {} (PID {}) was trapped (status {})", dbgee.dbgid, dbgee.pid, si_status);
        }
        CLD_EXITED => {
            println!("\nDebugee {} (PID {}) has exited (code {})", dbgee.dbgid, dbgee.pid, si_status);
            DEBUGEES.write().unwrap().remove(dbgee.dbgid).ok();
        }
        _ => {
            println!("\nDebugee {:?} sent SIGCHLD: {:?}", dbgee, siginfo);
        }
    }
}


fn handle_sigint(_siginfo: siginfo_t) {
    unimplemented!();
}