use super::{VmValue, eval};
use std::path::{Path, PathBuf};

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("sapphire_dir_test_{}_{}", std::process::id(), name))
}

fn spr_path(path: &Path) -> String {
    format!("{:?}", path.to_string_lossy().as_ref())
}

#[test]
fn pwd_returns_current_directory() {
    assert_eq!(eval("!Dir.pwd.empty?"), VmValue::Bool(true));
}

#[test]
fn mkdir_entries_children_and_delete() {
    let base = temp_path("entries");
    let _ = std::fs::remove_dir_all(&base);
    let base_s = spr_path(&base);
    let file_s = spr_path(&base.join("a.txt"));

    assert_eq!(eval(&format!("Dir.mkdir({base_s})")), VmValue::Nil);
    assert_eq!(eval(&format!("Dir.exist?({base_s})")), VmValue::Bool(true));
    assert_eq!(eval(&format!(r#"File.write({file_s}, "a")"#)), VmValue::Nil);
    assert_eq!(
        eval(&format!(r#"Dir.children({base_s}).include?("a.txt")"#)),
        VmValue::Bool(true)
    );
    assert_eq!(
        eval(&format!(r#"Dir.entries({base_s}).include?(".")"#)),
        VmValue::Bool(true)
    );
    assert_eq!(
        eval(&format!(r#"Dir.entries({base_s}).include?("..")"#)),
        VmValue::Bool(true)
    );
    assert_eq!(eval(&format!("File.delete({file_s})")), VmValue::Nil);
    assert_eq!(eval(&format!("Dir.delete({base_s})")), VmValue::Nil);
    assert_eq!(eval(&format!("Dir.exist?({base_s})")), VmValue::Bool(false));
}

#[test]
fn mkdir_p_creates_nested_directories() {
    let base = temp_path("nested");
    let nested = base.join("a").join("b");
    let _ = std::fs::remove_dir_all(&base);
    let base_s = spr_path(&base);
    let nested_s = spr_path(&nested);

    assert_eq!(eval(&format!("Dir.mkdir_p({nested_s})")), VmValue::Nil);
    assert_eq!(
        eval(&format!("Dir.exist?({nested_s})")),
        VmValue::Bool(true)
    );
    std::fs::remove_dir_all(&base).unwrap();
    assert_eq!(eval(&format!("Dir.exist?({base_s})")), VmValue::Bool(false));
}
