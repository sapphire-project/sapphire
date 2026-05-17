use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::gc::{GcHeap, GcRef};
use crate::vm::{define_native_class_method, HeapObject, NativeArity, VmError, VmValue};
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
        [arg] => Ok(VmValue::Bool(
            Path::new(path_arg("File", "exist?", arg, line)?).exists(),
        )),
        _ => unreachable!("File.exist? arity checked before native dispatch"),
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
        _ => unreachable!("File.file? arity checked before native dispatch"),
    }
}

fn file_read(
    _heap: &mut GcHeap<HeapObject>,
    _recv: &VmValue,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match args {
        [arg] => {
            let path = path_arg("File", "read", arg, line)?;
            std::fs::read_to_string(path)
                .map(Str)
                .map_err(|e| io_raised(path, e))
        }
        _ => unreachable!("File.read arity checked before native dispatch"),
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
        _ => unreachable!("File.delete arity checked before native dispatch"),
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
        _ => unreachable!("File.rename arity checked before native dispatch"),
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
        _ => unreachable!("File.size arity checked before native dispatch"),
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
        _ => unreachable!("File.mtime arity checked before native dispatch"),
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
        [path, content] => {
            let path = path_arg("File", "write", path, line)?;
            let content = match content {
                Str(content) => content.as_str(),
                _ => {
                    return Err(VmError::TypeError {
                        message: "File.write: content must be a string".to_string(),
                        line,
                    })
                }
            };
            std::fs::write(path, content)
                .map(|_| VmValue::Nil)
                .map_err(|e| io_raised(path, e))
        }
        _ => unreachable!("File.write arity checked before native dispatch"),
    }
}
