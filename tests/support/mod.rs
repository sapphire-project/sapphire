#![allow(dead_code)]
mod typecheck;

use std::path::PathBuf;

use sapphire::ast::Expr;
use sapphire::compiler::compile;
use sapphire::error::SapphireError;
use sapphire::lexer::Lexer;
use sapphire::parser::Parser;
use sapphire::vm::{Vm, VmError, VmValue};

#[allow(unused_imports)]
pub use typecheck::{check_types_ok, typecheck_err_msg, typecheck_ok};

#[macro_export]
macro_rules! assert_typecheck_error {
    ($src:expr, $($substring:expr),+ $(,)?) => {{
        let msg = $crate::support::typecheck_err_msg($src);
        $(
        assert!(
            msg.contains($substring),
            "expected first type error to contain:\n{}\n\nmessage:\n{}",
            $substring,
            msg
        );
        )+
    }};
}

pub fn parse_stmts(src: &str) -> Vec<Expr> {
    let tokens = Lexer::new(src).scan_tokens();
    Parser::new(tokens).parse().expect("parse error")
}

struct EvalOpts {
    stdlib: bool,
    path: PathBuf,
}

impl Default for EvalOpts {
    fn default() -> Self {
        Self {
            stdlib: false,
            path: PathBuf::new(),
        }
    }
}

fn run(src: &str, opts: EvalOpts) -> Result<Option<VmValue>, VmError> {
    let func = compile(&parse_stmts(src)).expect("compile error");
    let mut vm = Vm::new(func, opts.path);
    if opts.stdlib {
        vm.load_stdlib().expect("stdlib");
    }
    vm.run()
}

pub fn eval(src: &str) -> VmValue {
    run(src, EvalOpts::default())
        .expect("vm error")
        .expect("empty stack")
}

pub fn eval_stdlib(src: &str) -> VmValue {
    run(
        src,
        EvalOpts {
            stdlib: true,
            ..Default::default()
        },
    )
    .expect("vm error")
    .expect("empty stack")
}

pub fn eval_in_dir(src: &str, dir: PathBuf) -> VmValue {
    run(
        src,
        EvalOpts {
            stdlib: true,
            path: dir,
        },
    )
    .expect("vm error")
    .expect("empty stack")
}

pub fn eval_err(src: &str) -> VmError {
    run(src, EvalOpts::default()).expect_err("expected vm error")
}

pub fn eval_stdlib_err(src: &str) -> VmError {
    run(
        src,
        EvalOpts {
            stdlib: true,
            ..Default::default()
        },
    )
    .expect_err("expected vm error")
}

pub fn parse_err(src: &str) -> SapphireError {
    let tokens = Lexer::new(src).scan_tokens();
    Parser::new(tokens).parse().expect_err("expected parse error")
}

pub fn parse_err_msg(src: &str) -> String {
    format!("{}", parse_err(src))
}

pub fn eval_int(src: &str) -> i64 {
    match eval_stdlib(src) {
        VmValue::Int(n) => n,
        other => panic!("expected Int, got {:?}", other),
    }
}

pub fn eval_bool(src: &str) -> bool {
    match eval_stdlib(src) {
        VmValue::Bool(b) => b,
        other => panic!("expected Bool, got {:?}", other),
    }
}

pub fn eval_str(src: &str) -> String {
    match eval_stdlib(src) {
        VmValue::Str(s) => s,
        other => panic!("expected Str, got {:?}", other),
    }
}
