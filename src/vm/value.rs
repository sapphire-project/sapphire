use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use super::{NativeFn, VmMethod, VmValue};

/// Inclusive arity bounds for a [`SapphireMethod::Native`] (`min`..=`max` arguments).
#[derive(Clone, Copy)]
pub struct NativeArity {
    pub min: usize,
    pub max: usize,
}

impl From<usize> for NativeArity {
    fn from(n: usize) -> Self {
        Self { min: n, max: n }
    }
}

impl NativeArity {
    /// Sentinel `max` meaning “no upper bound” (arity is `min` or more).
    pub const VARIADIC_MAX: usize = usize::MAX;

    pub fn at_least(min: usize) -> Self {
        Self {
            min,
            max: Self::VARIADIC_MAX,
        }
    }
}

/// A method that lives in a `ClassObject` method table.
#[derive(Clone)]
pub enum SapphireMethod {
    Bytecode(VmMethod),
    Native {
        min_arity: usize,
        max_arity: usize,
        func: NativeFn,
    },
}

/// The heap-allocated cell shared between a closure and the variable it captures.
/// While the captured variable is still live on the stack the upvalue is "open"
/// (holds a stack index).  When the enclosing frame returns the upvalue is
/// "closed": the value is copied out of the stack into the cell itself.
#[derive(Debug, Clone)]
pub enum UpvalueState {
    Open(usize), // index into Vm::stack
    Closed(VmValue),
}

#[derive(Debug, Clone)]
pub struct Upvalue(pub Rc<RefCell<UpvalueState>>);

impl Upvalue {
    pub(super) fn new_open(stack_idx: usize) -> Self {
        Upvalue(Rc::new(RefCell::new(UpvalueState::Open(stack_idx))))
    }
}

impl PartialEq for Upvalue {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Display for VmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmValue::Int(n) => write!(f, "{}", n),
            VmValue::Float(n) => write!(f, "{}", n),
            VmValue::Str(s) => write!(f, "{}", s),
            VmValue::Bool(b) => write!(f, "{}", b),
            VmValue::Nil => write!(f, "nil"),
            VmValue::Function(func) => write!(f, "<fn {}>", func.name),
            VmValue::Closure { function, .. } => write!(f, "<fn {}>", function.name),
            VmValue::List(_) => write!(f, "<list>"),
            VmValue::Map(_) => write!(f, "<map>"),
            VmValue::Set(_) => write!(f, "<set>"),
            VmValue::Range { from, to } => write!(f, "{}..{}", from, to),
            VmValue::Class { name, .. } => write!(f, "<class {}>", name),
            VmValue::Instance { class_name, .. } => write!(f, "#<{}>", class_name),
            VmValue::ClassObj(_, name) => write!(f, "{}", name),
        }
    }
}
