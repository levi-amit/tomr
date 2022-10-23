use super::*;


/// Starts a new thread for signal handling
pub fn setup_signal_handlers() -> Result<(), Error> {
    let mut signals: SignalsInfo<WithOrigin> = SignalsInfo::<WithOrigin>::new(&[SIGINT, SIGCHLD])
        .expect("Could not set up signal iterator through signal-hook");

    thread::spawn(move || {
        for siginfo in signals.forever() {
            // this line is to help rust-analyzer to detect the type of `siginfo`
            let siginfo: siginfo::Origin = siginfo;

            match siginfo.signal {
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
fn handle_sigchld(siginfo: siginfo::Origin) {
    // determine signaling child debugee
    let dbgee = DEBUGEES.read().unwrap()
        .from_pid(Pid::from_raw(
            siginfo.process
                .expect("SIGCHLD unexpecedly did not contain an originating process")
                .pid
            )
        )
        .expect("Non-debugee process sent SIGCHLD, currently unhandled")
        .clone();

    match siginfo.cause {
        siginfo::Cause::Chld(siginfo::Chld::Trapped) => {
            println!("\nDebugee {} (PID {}) was trapped", dbgee.dbgid, dbgee.pid);
        }
        siginfo::Cause::Chld(siginfo::Chld::Exited) => {
            // TODO: add printing of exit code. seems to require using the raw siginfo instead of the origin.
            println!("\nDebugee {} (PID {}) has exited", dbgee.dbgid, dbgee.pid);
            DEBUGEES.write().unwrap().remove(dbgee.dbgid).ok();
        }
        _ => {
            println!("\nDebugee {:?} sent SIGCHLD: {:?}", dbgee, siginfo);
        }
    }
}


fn handle_sigint(_siginfo: siginfo::Origin) {
    unimplemented!();
}