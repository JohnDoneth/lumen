use crate::erlang::monotonic_time_0::native;
use crate::test::with_process;

use lumen_rt_full::time::monotonic;

#[test]
fn increases_after_2_native_time_units() {
    with_process(|process| {
        let start_time_in_milliseconds = monotonic::freeze_time_in_milliseconds();

        let first = native(process).unwrap();

        monotonic::freeze_at_time_in_milliseconds(start_time_in_milliseconds + 2);

        let second = native(process).unwrap();

        assert!(first < second);
    });
}
