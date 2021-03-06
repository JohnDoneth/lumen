mod with_function;

use std::convert::TryInto;

use proptest::prop_assert_eq;
use proptest::strategy::Just;

use liblumen_alloc::erts::process::Status;
use liblumen_alloc::erts::term::prelude::*;

use crate::erlang::spawn_1::native;
use crate::test::strategy::term::function;
use crate::test::{prop_assert_exits_badarity, strategy};
use lumen_rt_full::registry::pid_to_process;

#[test]
fn without_function_errors_badarg() {
    crate::test::without_function_errors_badarg(file!(), native);
}
