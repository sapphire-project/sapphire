use std::io::BufRead;

use crate::gc::{GcHeap, GcRef};
use crate::output;
use crate::vm::{HeapObject, VmError, VmValue, define_native_class_method};

pub fn register_class_methods(heap: &mut GcHeap<HeapObject>, class_ref: GcRef) {
    define_native_class_method(heap, class_ref, "puts", 1, io_puts);
    define_native_class_method(heap, class_ref, "print", 1, io_print);
    define_native_class_method(heap, class_ref, "gets", 0, io_gets);
}

fn format_arg(method: &str, args: &[VmValue], line: u32) -> Result<String, VmError> {
    match args {
        [v] => Ok(format!("{v}")),
        _ => Err(VmError::TypeError {
            message: format!("IO.{method} expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn io_puts(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    output::emit_line(&format_arg("puts", args, line)?);
    Ok(VmValue::Nil)
}

fn io_print(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    output::emit_raw(&format_arg("print", args, line)?);
    Ok(VmValue::Nil)
}

fn io_gets(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    if !args.is_empty() {
        return Err(VmError::TypeError {
            message: format!("IO.gets expects 0 arguments, got {}", args.len()),
            line,
        });
    }
    let mut buf = String::new();
    match std::io::stdin().lock().read_line(&mut buf) {
        Ok(0) => Ok(VmValue::Nil),
        Ok(_) => {
            if buf.ends_with('\n') {
                buf.pop();
                if buf.ends_with('\r') {
                    buf.pop();
                }
            }
            Ok(VmValue::Str(buf))
        }
        Err(e) => Err(VmError::Raised(VmValue::Str(format!(
            "IO.gets failed: {e}"
        )))),
    }
}
