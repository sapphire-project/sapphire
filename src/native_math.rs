use crate::gc::{GcHeap, GcRef};
use crate::vm::{define_native_class_method, HeapObject, VmError, VmValue};

fn math_arg(args: &[VmValue], method: &str, line: u32) -> Result<f64, VmError> {
    match args {
        [VmValue::Float(f)] => Ok(*f),
        [VmValue::Int(i)]   => Ok(*i as f64),
        [_] => Err(VmError::TypeError {
            message: format!("Math.{method}: argument must be numeric"), line,
        }),
        _ => Err(VmError::TypeError {
            message: format!("Math.{method} expects 1 argument, got {}", args.len()), line,
        }),
    }
}

fn math_sin(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    math_arg(args, "sin", line).map(|f| VmValue::Float(f.sin()))
}

fn math_cos(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    math_arg(args, "cos", line).map(|f| VmValue::Float(f.cos()))
}

fn math_asin(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    math_arg(args, "asin", line).map(|f| VmValue::Float(f.asin()))
}

fn math_atan(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    math_arg(args, "atan", line).map(|f| VmValue::Float(f.atan()))
}

pub fn register_class_methods(heap: &mut GcHeap<HeapObject>, class_ref: GcRef) {
    define_native_class_method(heap, class_ref, "sin",  1, math_sin);
    define_native_class_method(heap, class_ref, "cos",  1, math_cos);
    define_native_class_method(heap, class_ref, "asin", 1, math_asin);
    define_native_class_method(heap, class_ref, "atan", 1, math_atan);
}
