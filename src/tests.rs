use crate::{Code, Vars};
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

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
    run_test("puts");
}

#[test]
fn test_copy() {
    run_test("copy");
}

#[test]
fn test_call_string() {
    run_test("call_string");
}

#[test]
fn test_call_object() {
    run_test("call_object");
}

#[test]
fn test_call_closure() {
    run_test("call_closure");
}

#[test]
fn test_closure_args() {
    run_test("closure_args");
}

#[test]
fn test_call_raw_closure() {
    run_test("call_raw_closure")
}

#[test]
fn test_closure_scope() {
    run_test("closure_scope")
}

#[test]
fn test_quoted_strings() {
    run_test("quoted_strings")
}
