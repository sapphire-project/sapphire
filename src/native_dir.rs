use crate::gc::{GcHeap, GcRef};
use crate::vm::{HeapObject, VmError, VmValue, define_native_class_method};
use VmValue::Str;

pub fn register_class_methods(heap: &mut GcHeap<HeapObject>, class_ref: GcRef) {
    define_native_class_method(heap, class_ref, "children", 1, dir_children);
    define_native_class_method(heap, class_ref, "delete", 1, dir_delete);
    define_native_class_method(heap, class_ref, "exist?", 1, dir_exist_q);
    define_native_class_method(heap, class_ref, "mkdir", 1, dir_mkdir);
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
    _args: &[VmValue],
    _line: u32,
) -> Result<VmValue, VmError> {
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
        _ => unreachable!("Dir.exist? arity checked before native dispatch"),
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
        _ => unreachable!("Dir.children arity checked before native dispatch"),
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
        _ => unreachable!("Dir.mkdir arity checked before native dispatch"),
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
        _ => unreachable!("Dir.delete arity checked before native dispatch"),
    }
}
