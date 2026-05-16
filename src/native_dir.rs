use crate::gc::{GcHeap, GcRef};
use crate::vm::{HeapObject, VmError, VmValue, define_native_class_method};
use VmValue::Str;

pub fn register_class_methods(heap: &mut GcHeap<HeapObject>, class_ref: GcRef) {
    define_native_class_method(heap, class_ref, "children", 1, dir_children);
    define_native_class_method(heap, class_ref, "delete", 1, dir_delete);
    define_native_class_method(heap, class_ref, "entries", 1, dir_entries);
    define_native_class_method(heap, class_ref, "exist?", 1, dir_exist_q);
    define_native_class_method(heap, class_ref, "mkdir", 1, dir_mkdir);
    define_native_class_method(heap, class_ref, "mkdir_p", 1, dir_mkdir_p);
    define_native_class_method(heap, class_ref, "pwd", 0, dir_pwd);
}

fn path_arg(method: &str, arg: &VmValue, line: u32) -> Result<String, VmError> {
    match arg {
        Str(path) => Ok(path.clone()),
        _ => Err(VmError::TypeError {
            message: format!("Dir.{method}: path must be a string"),
            line,
        }),
    }
}

fn io_raised(path: &str, e: std::io::Error) -> VmError {
    VmError::Raised(Str(format!("{path}: {e}")))
}

fn read_child_names(path: &str) -> Result<Vec<VmValue>, VmError> {
    let mut names: Vec<String> = std::fs::read_dir(path)
        .map_err(|e| io_raised(path, e))?
        .map(|entry| {
            entry
                .map_err(|e| io_raised(path, e))
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
        })
        .collect::<Result<_, _>>()?;
    names.sort();
    Ok(names.into_iter().map(Str).collect())
}

fn dir_pwd(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    if !args.is_empty() {
        return Err(VmError::TypeError {
            message: format!("Dir.pwd expects 0 arguments, got {}", args.len()),
            line,
        });
    }
    std::env::current_dir()
        .map(|path| Str(path.to_string_lossy().into_owned()))
        .map_err(|e| VmError::Raised(Str(format!("Dir.pwd: {e}"))))
}

fn dir_exist_q(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => Ok(VmValue::Bool(
            std::path::Path::new(&path_arg("exist?", arg, line)?).is_dir(),
        )),
        _ => Err(VmError::TypeError {
            message: format!("Dir.exist? expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn dir_children(
    heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("children", arg, line)?;
            Ok(VmValue::List(
                heap.alloc(HeapObject::List(read_child_names(&path)?)),
            ))
        }
        _ => Err(VmError::TypeError {
            message: format!("Dir.children expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn dir_entries(
    heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("entries", arg, line)?;
            let mut names = vec![Str(".".to_string()), Str("..".to_string())];
            names.extend(read_child_names(&path)?);
            Ok(VmValue::List(heap.alloc(HeapObject::List(names))))
        }
        _ => Err(VmError::TypeError {
            message: format!("Dir.entries expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn dir_mkdir(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("mkdir", arg, line)?;
            std::fs::create_dir(&path)
                .map(|_| VmValue::Nil)
                .map_err(|e| io_raised(&path, e))
        }
        _ => Err(VmError::TypeError {
            message: format!("Dir.mkdir expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn dir_mkdir_p(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("mkdir_p", arg, line)?;
            std::fs::create_dir_all(&path)
                .map(|_| VmValue::Nil)
                .map_err(|e| io_raised(&path, e))
        }
        _ => Err(VmError::TypeError {
            message: format!("Dir.mkdir_p expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn dir_delete(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("delete", arg, line)?;
            std::fs::remove_dir(&path)
                .map(|_| VmValue::Nil)
                .map_err(|e| io_raised(&path, e))
        }
        _ => Err(VmError::TypeError {
            message: format!("Dir.delete expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}
