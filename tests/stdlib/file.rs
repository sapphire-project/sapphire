use super::{VmValue, eval};
use std::path::{Path, PathBuf};

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "sapphire_file_test_{}_{}",
        std::process::id(),
        name
    ))
}

fn spr_path(path: &Path) -> String {
    format!("{:?}", path.to_string_lossy().as_ref())
}

#[test]
fn read_write_metadata_and_delete() {
    let path = temp_path("metadata.txt");
    let _ = std::fs::remove_file(&path);
    let path = spr_path(&path);

    assert_eq!(
        eval(&format!(r#"File.write({path}, "hello")"#)),
        VmValue::Nil
    );
    assert_eq!(
        eval(&format!("File.read({path})")),
        VmValue::Str("hello".into())
    );
    assert_eq!(eval(&format!("File.size({path})")), VmValue::Int(5));
    assert_eq!(
        eval(&format!("File.mtime({path}) > 0")),
        VmValue::Bool(true)
    );
    assert_eq!(eval(&format!("File.file?({path})")), VmValue::Bool(true));
    assert_eq!(
        eval(&format!("File.directory?({path})")),
        VmValue::Bool(false)
    );
    assert_eq!(eval(&format!("File.delete({path})")), VmValue::Nil);
    assert_eq!(eval(&format!("File.exist?({path})")), VmValue::Bool(false));
}

#[test]
fn rename_moves_file() {
    let from = temp_path("rename_from.txt");
    let to = temp_path("rename_to.txt");
    let _ = std::fs::remove_file(&from);
    let _ = std::fs::remove_file(&to);
    let from = spr_path(&from);
    let to = spr_path(&to);

    eval(&format!(r#"File.write({from}, "moved")"#));
    assert_eq!(eval(&format!("File.rename({from}, {to})")), VmValue::Nil);
    assert_eq!(eval(&format!("File.exist?({from})")), VmValue::Bool(false));
    assert_eq!(
        eval(&format!("File.read({to})")),
        VmValue::Str("moved".into())
    );
    assert_eq!(eval(&format!("File.delete({to})")), VmValue::Nil);
}

#[test]
fn path_helpers() {
    assert_eq!(
        eval(r#"File.join("/tmp", "sapphire", "file.txt")"#),
        VmValue::Str("/tmp/sapphire/file.txt".into())
    );
    assert_eq!(
        eval(r#"File.basename("/tmp/sapphire/file.txt")"#),
        VmValue::Str("file.txt".into())
    );
    assert_eq!(
        eval(r#"File.dirname("/tmp/sapphire/file.txt")"#),
        VmValue::Str("/tmp/sapphire".into())
    );
    assert_eq!(
        eval(r#"File.dirname("file.txt")"#),
        VmValue::Str(".".into())
    );
    assert_eq!(
        eval(r#"File.extname("/tmp/sapphire/file.txt")"#),
        VmValue::Str(".txt".into())
    );
    assert_eq!(
        eval(r#"File.extname("/tmp/sapphire/README")"#),
        VmValue::Str("".into())
    );
}
