use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::vm::{Vm, VmError, VmValue};

fn raise(msg: impl Into<String>) -> VmError {
    VmError::Raised(VmValue::Str(msg.into()))
}

pub fn socket_connect(host: &str, port: i64, _line: u32) -> Result<BufReader<TcpStream>, VmError> {
    TcpStream::connect(format!("{}:{}", host, port))
        .map(BufReader::new)
        .map_err(|e| raise(format!("Socket.connect: {}", e)))
}

pub fn socket_write(
    reader: &mut BufReader<TcpStream>,
    data: &str,
    _line: u32,
) -> Result<(), VmError> {
    reader
        .get_mut()
        .write_all(data.as_bytes())
        .map_err(|e| raise(format!("socket.write: {}", e)))
}

pub fn socket_read_line(reader: &mut BufReader<TcpStream>, _line: u32) -> Result<String, VmError> {
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .map_err(|e| raise(format!("socket.read_line: {}", e)))?;
    if buf.ends_with('\n') {
        buf.pop();
        if buf.ends_with('\r') {
            buf.pop();
        }
    }
    Ok(buf)
}

pub fn socket_read_bytes(
    reader: &mut BufReader<TcpStream>,
    n: i64,
    _line: u32,
) -> Result<String, VmError> {
    let n = n.max(0) as usize;
    let mut buf = vec![0u8; n];
    reader
        .read_exact(&mut buf)
        .map_err(|e| raise(format!("socket.read_bytes: {}", e)))?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

pub fn socket_read_all(reader: &mut BufReader<TcpStream>, _line: u32) -> Result<String, VmError> {
    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .map_err(|e| raise(format!("socket.read_all: {}", e)))?;
    Ok(buf)
}

pub fn extract_fd(fields: &HashMap<String, VmValue>, line: u32) -> Result<i64, VmError> {
    match fields.get("fd") {
        Some(VmValue::Int(n)) => Ok(*n),
        _ => Err(VmError::TypeError {
            message: "socket instance has invalid fd".into(),
            line,
        }),
    }
}

// VM runtime entry points invoked from `Vm::run_inner` invoke handling.
pub(crate) fn dispatch_socket_class(
    vm: &mut Vm,
    method_name: &str,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    match method_name {
        "connect" => {
            let (host, port) = match args {
                [VmValue::Str(h), VmValue::Int(p)] => (h.clone(), *p),
                _ => {
                    return Err(VmError::TypeError {
                        message: "Socket.connect expects (String, Int)".into(),
                        line,
                    });
                }
            };
            let reader = crate::native_socket::socket_connect(&host, port, line)?;
            let id = vm.next_socket_id();
            vm.insert_socket(id, reader);
            let methods = vm
                .class_methods("Socket")
                .ok_or_else(|| VmError::TypeError {
                    message: "Socket class not loaded; call load_stdlib() first".into(),
                    line,
                })?;
            let mut fields_map = HashMap::new();
            fields_map.insert("fd".to_string(), VmValue::Int(id));
            let fields_ref = vm.alloc_fields(fields_map);
            Ok(VmValue::Instance {
                class_name: "Socket".to_string(),
                ancestor_chain: std::rc::Rc::new(
                    vm.class_ancestors("Socket")
                        .unwrap_or_else(|| vec!["Socket".to_string()]),
                ),
                fields: fields_ref,
                methods,
            })
        }
        _ => Err(VmError::TypeError {
            message: format!("Socket has no class method '{}'", method_name),
            line,
        }),
    }
}

pub(crate) fn dispatch_socket_instance(
    vm: &mut Vm,
    fields_ref: crate::gc::GcRef,
    method_name: &str,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    let fields = vm.heap_fields_clone(fields_ref);
    let fd = crate::native_socket::extract_fd(&fields, line)?;
    let closed_err = || VmError::Raised(VmValue::Str(format!("socket fd {} is closed", fd)));
    match method_name {
        "write" => {
            let data = match args {
                [VmValue::Str(s)] => s.clone(),
                _ => {
                    return Err(VmError::TypeError {
                        message: "socket.write expects a String".into(),
                        line,
                    });
                }
            };
            let reader = vm.socket_mut(fd).ok_or_else(closed_err)?;
            crate::native_socket::socket_write(reader, &data, line)?;
            Ok(VmValue::Nil)
        }
        "read_line" => {
            if !args.is_empty() {
                return Err(VmError::TypeError {
                    message: "socket.read_line takes no arguments".into(),
                    line,
                });
            }
            let reader = vm.socket_mut(fd).ok_or_else(closed_err)?;
            crate::native_socket::socket_read_line(reader, line).map(VmValue::Str)
        }
        "read_bytes" => {
            let n = match args {
                [VmValue::Int(n)] => *n,
                _ => {
                    return Err(VmError::TypeError {
                        message: "socket.read_bytes expects an Int".into(),
                        line,
                    });
                }
            };
            let reader = vm.socket_mut(fd).ok_or_else(closed_err)?;
            crate::native_socket::socket_read_bytes(reader, n, line).map(VmValue::Str)
        }
        "read_all" => {
            if !args.is_empty() {
                return Err(VmError::TypeError {
                    message: "socket.read_all takes no arguments".into(),
                    line,
                });
            }
            let reader = vm.socket_mut(fd).ok_or_else(closed_err)?;
            crate::native_socket::socket_read_all(reader, line).map(VmValue::Str)
        }
        "close" => {
            vm.remove_socket(fd);
            Ok(VmValue::Nil)
        }
        _ => Err(VmError::TypeError {
            message: format!("Socket has no method '{}'", method_name),
            line,
        }),
    }
}
