use std::io::{BufRead, Write};

use crate::gc::{GcHeap, GcRef};
use crate::vm::{define_native_class_method, HeapObject, VmError, VmValue};
use VmValue::Str;

pub fn register_class_methods(heap: &mut GcHeap<HeapObject>, class_ref: GcRef) {
    define_native_class_method(heap, class_ref, "puts", 1, io_puts);
    define_native_class_method(heap, class_ref, "print", 1, io_print);
    define_native_class_method(heap, class_ref, "gets", 0, io_gets);
}

fn io_puts(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [Str(s)] => {
            println!("{s}");
            Ok(VmValue::Nil)
        }
        [_] => Err(VmError::TypeError {
            message: "IO.puts: argument must be a string".to_string(),
            line,
        }),
        _ => Err(VmError::TypeError {
            message: format!("IO.puts expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn io_print(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [Str(s)] => {
            print!("{s}");
            std::io::stdout().flush().ok();
            Ok(VmValue::Nil)
        }
        [_] => Err(VmError::TypeError {
            message: "IO.print: argument must be a string".to_string(),
            line,
        }),
        _ => Err(VmError::TypeError {
            message: format!("IO.print expects 1 argument, got {}", args.len()),
            line,
        }),
    }
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
            Ok(Str(buf))
        }
        Err(e) => Err(VmError::Raised(Str(format!("IO.gets failed: {e}")))),
    }
}
