use crate::{Code, Vars};
use std::{
    fs::File,
    io::prelude::*,
    sync::{Arc, Mutex},
};

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
            &Code::from_string(s, Arc::new(Mutex::new(fo)))
                .run(Vars::new(), &mut Vec::new())
                .to_string()
                .into_bytes();
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
