// Stdlib tests organized by class
// This mirrors the stdlib/ directory structure (int.spr, float.spr, string.spr, etc.)

mod support;

use sapphire::vm::VmValue;
pub use support::{
    eval_bool, eval_int, eval_stdlib as eval, eval_stdlib_err as eval_err, eval_str,
};

#[path = "stdlib/int.rs"]
mod int;

#[path = "stdlib/float.rs"]
mod float;

#[path = "stdlib/string.rs"]
mod string;

#[path = "stdlib/list.rs"]
mod list;

#[path = "stdlib/map.rs"]
mod map;

#[path = "stdlib/range.rs"]
mod range;

#[path = "stdlib/num.rs"]
mod num;

#[path = "stdlib/object.rs"]
mod object;

#[path = "stdlib/math.rs"]
mod math;

#[path = "stdlib/datetime.rs"]
mod datetime;

#[path = "stdlib/env.rs"]
mod env;

#[path = "stdlib/file.rs"]
mod file;

#[path = "stdlib/dir.rs"]
mod dir;

#[path = "stdlib/socket.rs"]
mod socket;

#[path = "stdlib/io.rs"]
mod io;
