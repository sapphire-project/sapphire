use std::collections::HashMap;

use crate::gc::{GcHeap, GcRef, Trace};
use crate::vm::{SapphireMethod, UpvalueState, VmValue};

/// Objects managed by the GC heap — all types that can form reference cycles.
pub enum HeapObject {
    List(Vec<VmValue>),
    Map(HashMap<String, VmValue>),
    Set(Vec<VmValue>),
    /// Instance field storage.
    Fields(HashMap<String, VmValue>),
    /// A heap-allocated class object in the Ruby-style object model.
    /// `class_ref` points to the class's own class (e.g. every ClassObject's
    /// class_ref points to the `Class` ClassObject, and `Class.class_ref`
    /// points to itself).  `None` only transiently during two-phase bootstrap.
    ClassObject {
        name: String,
        superclass: Option<GcRef>,
        class_ref: Option<GcRef>,
        methods: HashMap<String, SapphireMethod>,
        class_methods: HashMap<String, SapphireMethod>,
    },
}

impl Trace for HeapObject {
    fn trace(&self, out: &mut Vec<GcRef>) {
        match self {
            HeapObject::List(v) => v.iter().for_each(|val| collect_refs(val, out)),
            HeapObject::Map(m) => m.values().for_each(|val| collect_refs(val, out)),
            HeapObject::Set(v) => v.iter().for_each(|val| collect_refs(val, out)),
            HeapObject::Fields(f) => f.values().for_each(|val| collect_refs(val, out)),
            HeapObject::ClassObject {
                superclass,
                class_ref,
                methods,
                class_methods,
                ..
            } => {
                if let Some(r) = superclass {
                    out.push(*r);
                }
                if let Some(r) = class_ref {
                    out.push(*r);
                }
                for m in methods.values().chain(class_methods.values()) {
                    if let SapphireMethod::Bytecode(vm_method) = m {
                        for uv in &vm_method.upvalues {
                            if let UpvalueState::Closed(v) = &*uv.0.borrow() {
                                collect_refs(v, out);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl GcHeap<HeapObject> {
    pub fn get_list(&self, r: GcRef) -> &Vec<VmValue> {
        match self.get(r) {
            HeapObject::List(v) => v,
            _ => panic!("GcRef is not a List"),
        }
    }
    pub fn get_list_mut(&mut self, r: GcRef) -> &mut Vec<VmValue> {
        match self.get_mut(r) {
            HeapObject::List(v) => v,
            _ => panic!("GcRef is not a List"),
        }
    }
    pub fn get_map(&self, r: GcRef) -> &HashMap<String, VmValue> {
        match self.get(r) {
            HeapObject::Map(m) => m,
            _ => panic!("GcRef is not a Map"),
        }
    }
    pub fn get_map_mut(&mut self, r: GcRef) -> &mut HashMap<String, VmValue> {
        match self.get_mut(r) {
            HeapObject::Map(m) => m,
            _ => panic!("GcRef is not a Map"),
        }
    }
    pub fn get_set(&self, r: GcRef) -> &Vec<VmValue> {
        match self.get(r) {
            HeapObject::Set(v) => v,
            _ => panic!("GcRef is not a Set"),
        }
    }
    pub fn get_set_mut(&mut self, r: GcRef) -> &mut Vec<VmValue> {
        match self.get_mut(r) {
            HeapObject::Set(v) => v,
            _ => panic!("GcRef is not a Set"),
        }
    }
    pub fn get_fields(&self, r: GcRef) -> &HashMap<String, VmValue> {
        match self.get(r) {
            HeapObject::Fields(f) => f,
            _ => panic!("GcRef is not Fields"),
        }
    }
    pub fn get_fields_mut(&mut self, r: GcRef) -> &mut HashMap<String, VmValue> {
        match self.get_mut(r) {
            HeapObject::Fields(f) => f,
            _ => panic!("GcRef is not Fields"),
        }
    }
}

/// Push all GcRefs contained directly in `val` into `out`.
pub(super) fn collect_refs(val: &VmValue, out: &mut Vec<GcRef>) {
    match val {
        VmValue::List(r) | VmValue::Map(r) | VmValue::Set(r) | VmValue::ClassObj(r, _) => {
            out.push(*r)
        }
        VmValue::Instance { fields, .. } => out.push(*fields),
        _ => {}
    }
}

/// Recursively format `val` using heap data for List/Map/Instance.
pub fn format_value_with_heap(heap: &GcHeap<HeapObject>, val: &VmValue) -> String {
    match val {
        VmValue::List(r) => {
            let parts: Vec<String> = heap
                .get_list(*r)
                .iter()
                .map(|el| format_value_with_heap(heap, el))
                .collect();
            format!("[{}]", parts.join(", "))
        }
        VmValue::Map(r) => {
            let mut parts: Vec<String> = heap
                .get_map(*r)
                .iter()
                .map(|(k, v)| format!("{}: {}", k, format_value_with_heap(heap, v)))
                .collect();
            parts.sort();
            format!("{{{}}}", parts.join(", "))
        }
        VmValue::Set(r) => {
            let parts: Vec<String> = heap
                .get_set(*r)
                .iter()
                .map(|el| format_value_with_heap(heap, el))
                .collect();
            format!("Set{{{}}}", parts.join(", "))
        }
        VmValue::Instance {
            class_name, fields, ..
        } => {
            let mut pairs: Vec<String> = heap
                .get_fields(*fields)
                .iter()
                .map(|(k, v)| format!("{}={}", k, format_value_with_heap(heap, v)))
                .collect();
            pairs.sort();
            format!("#<{} {}>", class_name, pairs.join(", "))
        }
        other => format!("{}", other),
    }
}
