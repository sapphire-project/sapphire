// Stdlib tests organized by class
// This mirrors the stdlib/ directory structure (int.spr, float.spr, string.spr, etc.)

use sapphire::compiler::compile;
use sapphire::lexer::Lexer;
use sapphire::parser::Parser;
use sapphire::vm::{Vm, VmError, VmValue};

fn eval(src: &str) -> VmValue {
    let tokens = Lexer::new(src).scan_tokens();
    let stmts = Parser::new(tokens).parse().expect("parse error");
    let func = compile(&stmts).expect("compile error");
    let mut vm = Vm::new(func, std::path::PathBuf::new());
    vm.load_stdlib().expect("stdlib");
    vm.run().expect("vm error").expect("empty stack")
}

fn eval_err(src: &str) -> VmError {
    let tokens = Lexer::new(src).scan_tokens();
    let stmts = Parser::new(tokens).parse().expect("parse error");
    let func = compile(&stmts).expect("compile error");
    let mut vm = Vm::new(func, std::path::PathBuf::new());
    vm.load_stdlib().expect("stdlib");
    vm.run().expect_err("expected vm error")
}

#[path = "stdlib/int.rs"]
mod int;

#[path = "stdlib/float.rs"]
mod float;

#[path = "stdlib/string.rs"]
mod string;

#[path = "stdlib/list.rs"]
mod list;

#[path = "stdlib/map.rs"]
mod map;

#[path = "stdlib/range.rs"]
mod range;

#[path = "stdlib/num.rs"]
mod num;

#[path = "stdlib/object.rs"]
mod object;

#[path = "stdlib/math.rs"]
mod math;

#[path = "stdlib/datetime.rs"]
mod datetime;

#[path = "stdlib/env.rs"]
mod env;

#[path = "stdlib/file.rs"]
mod file;

#[path = "stdlib/dir.rs"]
mod dir;

#[path = "stdlib/socket.rs"]
mod socket;

#[path = "stdlib/io.rs"]
mod io;

#[test]
fn stdlib_defines_cli_global() {
    let tokens = Lexer::new("nil").scan_tokens();
    let stmts = Parser::new(tokens).parse().expect("parse");
    let func = compile(&stmts).expect("compile");
    let mut vm = Vm::new(func, std::path::PathBuf::new());
    vm.load_stdlib().expect("stdlib");
    let keys: Vec<&String> = vm.globals.keys().collect();
    assert!(
        vm.globals.contains_key("CLI"),
        "expected CLI in globals, have: {:?}",
        keys
    );
    assert!(
        matches!(vm.globals.get("OptionParser"), Some(VmValue::Class { .. })),
        "OptionParser global: {:?}",
        vm.globals.get("OptionParser")
    );
}
