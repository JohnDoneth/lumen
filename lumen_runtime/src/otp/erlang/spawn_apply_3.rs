use std::convert::TryInto;

use liblumen_alloc::badarg;
use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::{Atom, Term};

use crate::process::spawn::options::Options;
use crate::scheduler::Scheduler;

pub(in crate::otp::erlang) fn native(
    process: &Process,
    options: Options,
    module: Term,
    function: Term,
    arguments: Term,
) -> exception::Result {
    let module_atom: Atom = module.try_into()?;
    let function_atom: Atom = function.try_into()?;

    if arguments.is_proper_list() {
        let child_arc_process =
            Scheduler::spawn_apply_3(process, options, module_atom, function_atom, arguments)?;

        Ok(child_arc_process.pid_term())
    } else {
        Err(badarg!().into())
    }
}
