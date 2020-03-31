mod with_false_left;
mod with_true_left;

use proptest::prop_assert_eq;
use proptest::test_runner::{Config, TestRunner};

use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::erlang::orelse_2::native;
use crate::test::{external_arc_node, strategy};
use crate::test::{with_process, with_process_arc};
use lumen_rt_full::process::SchedulerDependentAlloc;

#[test]
fn without_boolean_left_errors_badarg() {
    crate::test::without_boolean_left_errors_badarg(file!(), native);
}

#[test]
fn with_false_left_returns_right() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::term(arc_process.clone()), |right| {
                prop_assert_eq!(native(false.into(), right), Ok(right));

                Ok(())
            })
            .unwrap();
    });
}

#[test]
fn with_true_left_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::term(arc_process.clone()), |right| {
                prop_assert_eq!(native(true.into(), right), Ok(true.into()));

                Ok(())
            })
            .unwrap();
    });
}
