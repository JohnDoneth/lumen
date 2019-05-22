use super::*;

use num_traits::Num;

use crate::process::IntoProcess;

#[test]
fn with_atom_errors_badarg() {
    errors_badarg(|_| Term::str_to_atom("😈🤘", DoNotCare).unwrap());
}

#[test]
fn with_local_reference_errors_badarg() {
    errors_badarg(|process| Term::next_local_reference(process));
}

#[test]
fn with_empty_list_errors_badarg() {
    errors_badarg(|_| Term::EMPTY_LIST);
}

#[test]
fn with_list_errors_badarg() {
    errors_badarg(|process| list_term(&process));
}

#[test]
fn with_small_integer_errors_badarg() {
    errors_badarg(|process| 0usize.into_process(&process));
}

#[test]
fn with_big_integer_errors_badarg() {
    errors_badarg(|process| {
        <BigInt as Num>::from_str_radix("18446744073709551616", 10)
            .unwrap()
            .into_process(&process)
    });
}

#[test]
fn with_float_errors_badarg() {
    errors_badarg(|process| 1.0.into_process(&process));
}

#[test]
fn with_local_pid_errors_badarg() {
    errors_badarg(|_| Term::local_pid(0, 0).unwrap());
}

#[test]
fn with_external_pid_errors_badarg() {
    errors_badarg(|process| Term::external_pid(1, 0, 0, &process).unwrap());
}

#[test]
fn with_tuple_errors_badarg() {
    errors_badarg(|process| Term::slice_to_tuple(&[], &process));
}

#[test]
fn with_map_errors_badarg() {
    errors_badarg(|process| Term::slice_to_map(&[], &process));
}

#[test]
fn with_heap_binary_with_integer_errors_badarg() {
    errors_badarg(|process| Term::slice_to_binary("1".as_bytes(), &process));
}

#[test]
fn with_heap_binary_with_min_f64_returns_float() {
    with_process(|process| {
        let heap_binary_term =
            Term::slice_to_binary("-179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0".as_bytes(), &process);

        assert_eq!(
        erlang::binary_to_float_1(heap_binary_term, &process),
        Ok((-179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64).into_process(&process)),
    );
    });
}

#[test]
fn with_heap_binary_with_max_f64_returns_float() {
    with_process(|process| {
        let heap_binary_term =
            Term::slice_to_binary("179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0".as_bytes(), &process);

        assert_eq!(
        erlang::binary_to_float_1(heap_binary_term, &process),
        Ok(179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64.into_process(&process)),
    );
    });
}

#[test]
fn with_heap_binary_with_less_than_min_f64_errors_badarg() {
    errors_badarg(|process| {
        Term::slice_to_binary("-1797693134862315700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0".as_bytes(), &process)
    });
}

#[test]
fn with_heap_binary_with_greater_than_max_f64_errors_badarg() {
    errors_badarg(|process| {
        Term::slice_to_binary("17976931348623157000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0".as_bytes(), &process)
    });
}

#[test]
fn with_subbinary_with_min_f64_returns_float() {
    with_process(|process| {
        // <<1::1, "-179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0">>
        let heap_binary_term = Term::slice_to_binary(
            &[
                150,
                152,
                155,
                156,
                155,
                155,
                28,
                153,
                152,
                153,
                154,
                28,
                27,
                25,
                25,
                152,
                154,
                155,
                152,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                23,
                24,
                0b0000_0000,
            ],
            &process,
        );
        let subbinary_term = Term::subbinary(heap_binary_term, 0, 1, 312, 0, &process);

        assert_eq!(
        erlang::binary_to_float_1(subbinary_term, &process),
        Ok((-179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64).into_process(&process)),
    );
    });
}

#[test]
fn with_subbinary_with_max_f64_returns_float() {
    with_process(|process| {
        // <<1::1, "179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0">>
        let heap_binary_term = Term::slice_to_binary(
            &[
                152,
                155,
                156,
                155,
                155,
                28,
                153,
                152,
                153,
                154,
                28,
                27,
                25,
                25,
                152,
                154,
                155,
                152,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                23,
                24,
                0b0000_0000,
            ],
            &process,
        );
        let subbinary_term = Term::subbinary(heap_binary_term, 0, 1, 311, 0, &process);

        assert_eq!(
        erlang::binary_to_float_1(subbinary_term, &process),
        Ok(179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64.into_process(&process))
    );
    });
}

#[test]
fn with_subbinary_with_less_than_min_f64_returns_bag_argument() {
    errors_badarg(|process| {
        // <<1::1, "-1797693134862315700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0">>
        let heap_binary_term = Term::slice_to_binary(
            &[
                150,
                152,
                155,
                156,
                155,
                155,
                28,
                153,
                152,
                153,
                154,
                28,
                27,
                25,
                25,
                152,
                154,
                155,
                152,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                23,
                24,
                0b0000_0000,
            ],
            &process,
        );
        Term::subbinary(heap_binary_term, 0, 1, 313, 0, &process)
    });
}

#[test]
fn with_subbinary_with_greater_than_max_f64_errors_badarg() {
    errors_badarg(|process| {
        // <<1::1, "576460752303423488">>
        let heap_binary_term = Term::slice_to_binary(
            &[
                150,
                152,
                155,
                156,
                155,
                155,
                28,
                153,
                152,
                153,
                154,
                28,
                27,
                25,
                25,
                152,
                154,
                155,
                152,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                24,
                23,
                24,
                0b0000_0000,
            ],
            &process,
        );
        Term::subbinary(heap_binary_term, 0, 1, 313, 0, &process)
    });
}

fn errors_badarg<F>(binary: F)
where
    F: FnOnce(&Process) -> Term,
{
    super::errors_badarg(|process| erlang::binary_to_float_1(binary(&process), &process));
}