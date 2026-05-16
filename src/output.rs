use std::cell::RefCell;
use std::io::Write;

thread_local! {
    static SINK: RefCell<Option<String>> = const { RefCell::new(None) };
}

/// Activate buffered mode. All output goes to an in-memory string instead of stdout.
pub fn activate() {
    SINK.with(|s| *s.borrow_mut() = Some(String::new()));
}

/// Deactivate buffered mode and return whatever was collected.
pub fn take() -> String {
    SINK.with(|s| s.borrow_mut().take().unwrap_or_default())
}

/// Write `s` followed by a newline to the active sink, or to stdout.
pub fn emit_line(s: &str) {
    SINK.with(|sink| match &mut *sink.borrow_mut() {
        Some(buf) => {
            buf.push_str(s);
            buf.push('\n');
        }
        None => println!("{s}"),
    });
}

/// Write `s` without a trailing newline to the active sink, or to stdout.
pub fn emit_raw(s: &str) {
    SINK.with(|sink| match &mut *sink.borrow_mut() {
        Some(buf) => buf.push_str(s),
        None => {
            print!("{s}");
            std::io::stdout().flush().ok();
        }
    });
}
