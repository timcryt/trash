use crate::Code;
use std::{fs::File, io::prelude::*};

fn run_test(test_name: &str) {
    let mut s = String::new();
    let mut result = String::new();
    let mut answer = String::new();
    {
        let filename = "src/test/".to_string() + test_name + ".trash";
        let mut fi = File::open(&filename).unwrap();
        fi.read_to_string(&mut s).unwrap();
    }

    {
        let filename = "src/test/".to_string() + test_name + ".out";
        {
            let fo = File::create(&filename).unwrap();
            let (vars, mut scope) = crate::stdlib::stdlib(fo);

            Code::from_string(s).run(vars, &mut scope);
        }
        {
            let mut fi = File::open(&filename).unwrap();
            fi.read_to_string(&mut result).unwrap();
            std::fs::remove_file(&filename).unwrap();
        }
    }

    {
        let filename = "src/test/".to_string() + test_name + ".ans";
        let mut fi = File::open(&filename).unwrap();
        fi.read_to_string(&mut answer).unwrap();
    }

    assert_eq!(result, answer);
}

#[test]
fn test_puts() {
    run_test("core/core/puts");
}

#[test]
fn test_copy() {
    run_test("core/core/copy");
}

#[test]
fn test_call_string() {
    run_test("core/core/call_string");
}

#[test]
fn test_call_object() {
    run_test("core/core/call_object");
}

#[test]
fn test_call_closure() {
    run_test("core/core/call_closure");
}

#[test]
fn test_closure_args() {
    run_test("core/core/closure_args");
}

#[test]
fn test_call_raw_closure() {
    run_test("core/core/call_raw_closure");
}

#[test]
fn test_closure_scope() {
    run_test("core/core/closure_scope");
}

#[test]
fn test_quoted_strings() {
    run_test("core/core/quoted_strings");
}

#[test]
fn test_multiple_assign() {
    run_test("core/core/multiple_assign");
}

#[test]
fn test_nested_assign() {
    run_test("core/core/nested_assign");
}

#[test]
fn test_tuple_puts() {
    run_test("core/core/tuple_puts");
}

#[test]
fn test_string_len() {
    run_test("core/string/string_len");
}

#[test]
fn test_string_split() {
    run_test("core/string/string_split");
}

#[test]
fn test_string_push() {
    run_test("core/string/string_push");
}

#[test]
fn test_string_eq() {
    run_test("core/string/string_eq");
}

#[test]
fn test_tuple_push_pop() {
    run_test("core/tuple/push_pop");
}

#[test]
fn test_tuple_empty() {
    run_test("core/tuple/empty");
}

#[test]
fn test_tuple_index() {
    run_test("core/tuple/index");
}

#[test]
fn test_tuple_with() {
    run_test("core/tuple/with");
}

#[test]
fn test_move_clousre() {
    run_test("core/core/move_closure")
}

#[test]
fn test_if_statement() {
    run_test("stdlib/if_statement");
}

#[test]
fn test_while_statement() {
    run_test("stdlib/while_statement");
}

#[test]
fn test_integers() {
    run_test("stdlib/integers")
}
