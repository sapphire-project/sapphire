use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::gc::{GcHeap, GcRef};
use crate::vm::{HeapObject, NativeArity, VmError, VmValue, define_native_class_method};
use VmValue::Str;

pub fn register_class_methods(heap: &mut GcHeap<HeapObject>, class_ref: GcRef) {
    define_native_class_method(heap, class_ref, "delete", 1, file_delete);
    define_native_class_method(heap, class_ref, "exist?", 1, file_exist_q);
    define_native_class_method(heap, class_ref, "file?", 1, file_file_q);
    define_native_class_method(heap, class_ref, "join", NativeArity::at_least(0), file_join);
    define_native_class_method(heap, class_ref, "mtime", 1, file_mtime);
    define_native_class_method(heap, class_ref, "read", 1, file_read);
    define_native_class_method(heap, class_ref, "rename", 2, file_rename);
    define_native_class_method(heap, class_ref, "size", 1, file_size);
    define_native_class_method(heap, class_ref, "write", 2, file_write);
}

fn path_arg<'a>(
    class: &str,
    method: &str,
    arg: &'a VmValue,
    line: u32,
) -> Result<&'a str, VmError> {
    match arg {
        Str(path) => Ok(path.as_str()),
        _ => Err(VmError::TypeError {
            message: format!("{class}.{method}: path must be a string"),
            line,
        }),
    }
}

fn io_raised(path: &str, e: std::io::Error) -> VmError {
    VmError::Raised(Str(format!("{path}: {e}")))
}

fn path_string(path: &Path) -> String {
    let s = path.to_string_lossy();
    if s.is_empty() {
        ".".to_string()
    } else {
        s.into_owned()
    }
}

fn file_exist_q(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [Str(path)] => Ok(VmValue::Bool(std::path::Path::new(path.as_str()).exists())),
        [_] => Err(VmError::TypeError {
            message: "File.exist?: path must be a string".to_string(),
            line,
        }),
        _ => Err(VmError::TypeError {
            message: format!("File.exist? expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn file_file_q(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => Ok(VmValue::Bool(
            Path::new(path_arg("File", "file?", arg, line)?).is_file(),
        )),
        _ => Err(VmError::TypeError {
            message: format!("File.file? expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn file_read(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [Str(path)] => std::fs::read_to_string(path.as_str())
            .map(Str)
            .map_err(|e| VmError::Raised(Str(format!("{path}: {e}")))),
        [_] => Err(VmError::TypeError {
            message: "File.read: path must be a string".to_string(),
            line,
        }),
        _ => Err(VmError::TypeError {
            message: format!("File.read expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn file_delete(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("File", "delete", arg, line)?;
            std::fs::remove_file(path)
                .map(|_| VmValue::Nil)
                .map_err(|e| io_raised(path, e))
        }
        _ => Err(VmError::TypeError {
            message: format!("File.delete expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn file_rename(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [from, to] => {
            let from = path_arg("File", "rename", from, line)?;
            let to = path_arg("File", "rename", to, line)?;
            std::fs::rename(from, to)
                .map(|_| VmValue::Nil)
                .map_err(|e| io_raised(from, e))
        }
        _ => Err(VmError::TypeError {
            message: format!("File.rename expects 2 arguments, got {}", args.len()),
            line,
        }),
    }
}

fn file_size(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("File", "size", arg, line)?;
            let len = std::fs::metadata(path)
                .map_err(|e| io_raised(path, e))?
                .len();
            Ok(VmValue::Int(len as i64))
        }
        _ => Err(VmError::TypeError {
            message: format!("File.size expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn file_mtime(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("File", "mtime", arg, line)?;
            let modified = std::fs::metadata(path)
                .map_err(|e| io_raised(path, e))?
                .modified()
                .map_err(|e| io_raised(path, e))?;
            let seconds = modified
                .duration_since(UNIX_EPOCH)
                .map_err(|e| VmError::Raised(Str(format!("{path}: {e}"))))?
                .as_secs();
            Ok(VmValue::Int(seconds as i64))
        }
        _ => Err(VmError::TypeError {
            message: format!("File.mtime expects 1 argument, got {}", args.len()),
            line,
        }),
    }
}

fn file_join(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    let mut path = PathBuf::new();
    for arg in args {
        path.push(path_arg("File", "join", arg, line)?);
    }
    Ok(Str(path_string(&path)))
}

fn file_write(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [Str(path), Str(content)] => std::fs::write(path.as_str(), content.as_str())
            .map(|_| VmValue::Nil)
            .map_err(|e| VmError::Raised(Str(format!("{path}: {e}")))),
        [_, _] => Err(VmError::TypeError {
            message: "File.write: path and content must be strings".to_string(),
            line,
        }),
        _ => Err(VmError::TypeError {
            message: format!("File.write expects 2 arguments, got {}", args.len()),
            line,
        }),
    }
}
