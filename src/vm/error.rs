use std::fmt;

use crate::vm::VmValue;

#[derive(Debug, PartialEq)]
pub enum VmError {
    StackUnderflow,
    TypeError {
        message: String,
        line: u32,
    },
    /// `raise val` — propagates until caught by a `Begin` handler.
    Raised(VmValue),
    /// `break val` inside a block — unwinds to the enclosing call-with-block.
    Break(VmValue),
    /// `next val` inside a block — skips to the next `yield`.
    #[allow(dead_code)]
    Next(VmValue),
    /// `return val` inside a block called by a native method — propagates to
    /// the dispatch site so it can perform a non-local return from the
    /// enclosing Sapphire frame.
    Return(Option<VmValue>),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::StackUnderflow => {
                write!(
                    f,
                    "internal error: stack underflow (this is a Sapphire bug)"
                )
            }
            VmError::TypeError { message, line } => {
                write!(f, "[line {}] error: {}", line, message)
            }
            VmError::Raised(v) => write!(f, "uncaught raise: {}", v),
            VmError::Break(v) => write!(f, "break outside block: {}", v),
            VmError::Next(v) => write!(f, "next outside block: {}", v),
            VmError::Return(v) => write!(f, "return outside method: {:?}", v),
        }
    }
}

/// Rescue handler registered by `BeginRescue`; popped by `PopRescue`.
#[derive(Clone, Copy)]
pub struct RescueInfo {
    pub(crate) handler_ip: usize,
    pub(crate) rescue_var_slot: usize, // usize::MAX means no variable
    pub(crate) stack_height: usize,    // stack depth at BeginRescue time (for cleanup)
}
