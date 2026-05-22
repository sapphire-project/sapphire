//! Native datetime materialization used from the VM invoke path.
//! Entry points: class/instance datetime native dispatch handling in `Vm::run_inner`.

use crate::vm::{Vm, VmError, VmValue};

pub(crate) fn finalize_dt(
    vm: &mut Vm,
    dt: crate::datetime::DtValue,
    line: u32,
) -> Result<VmValue, VmError> {
    match dt {
        crate::datetime::DtValue::Value(v) => Ok(v),
        crate::datetime::DtValue::NewInstance { class_name, fields } => {
            let methods = vm
                .class_methods(&class_name)
                .ok_or_else(|| VmError::TypeError {
                    message: format!(
                        "datetime class '{}' not loaded; call vm.load_stdlib() first",
                        class_name
                    ),
                    line,
                })?;
            let ancestor_chain = vm
                .class_ancestors(&class_name)
                .unwrap_or_else(|| vec![class_name.clone()]);
            let gc_fields = vm.alloc_fields(fields);
            Ok(VmValue::Instance {
                class_name,
                ancestor_chain: std::rc::Rc::new(ancestor_chain),
                fields: gc_fields,
                methods,
            })
        }
    }
}
