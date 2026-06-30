use std::collections::HashMap;

use crate::gc::{GcHeap, GcRef};
use crate::vm::{HeapObject, VmError, VmValue, define_native_class_method};

pub fn register_class_methods(heap: &mut GcHeap<HeapObject>, class_ref: GcRef) {
    define_native_class_method(heap, class_ref, "parse", 1, json_parse);
}

fn json_parse(
    heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    let input = match args {
        [VmValue::Str(s)] => s,
        [_] => {
            return Err(VmError::TypeError {
                message: "JSON.parse: input must be a string".into(),
                line,
            });
        }
        _ => {
            return Err(VmError::TypeError {
                message: format!("JSON.parse expects 1 argument, got {}", args.len()),
                line,
            });
        }
    };

    let parsed: serde_json::Value = serde_json::from_str(input)
        .map_err(|e| VmError::Raised(VmValue::Str(format!("invalid JSON: {e}"))))?;

    Ok(json_to_vm(heap, parsed))
}

fn json_to_vm(heap: &mut GcHeap<HeapObject>, value: serde_json::Value) -> VmValue {
    match value {
        serde_json::Value::Null => VmValue::Nil,
        serde_json::Value::Bool(b) => VmValue::Bool(b),
        serde_json::Value::String(s) => VmValue::Str(s),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                VmValue::Int(i)
            } else if let Some(f) = n.as_f64() {
                VmValue::Float(f)
            } else {
                VmValue::Float(0.0)
            }
        }
        serde_json::Value::Array(items) => {
            let values = items.into_iter().map(|v| json_to_vm(heap, v)).collect();
            VmValue::List(heap.alloc(HeapObject::List(values)))
        }
        serde_json::Value::Object(obj) => {
            let map: HashMap<String, VmValue> = obj
                .into_iter()
                .map(|(k, v)| (k, json_to_vm(heap, v)))
                .collect();
            VmValue::Map(heap.alloc(HeapObject::Map(map)))
        }
    }
}
