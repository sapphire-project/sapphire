use crate::vm::{Vm, VmError, VmValue};
use std::collections::HashMap;

pub fn build_regex(pattern: &str, ignore_case: bool, line: u32) -> Result<regex::Regex, VmError> {
    let mut builder = regex::RegexBuilder::new(pattern);
    builder.case_insensitive(ignore_case);
    builder.build().map_err(|e| VmError::TypeError {
        message: format!("Invalid regex pattern: {}", e),
        line,
    })
}

pub fn extract_id(fields: &HashMap<String, VmValue>, line: u32) -> Result<i64, VmError> {
    match fields.get("id") {
        Some(VmValue::Int(n)) => Ok(*n),
        _ => Err(VmError::TypeError {
            message: "regex instance has invalid id".into(),
            line,
        }),
    }
}

pub fn regex_match_bool(re: &regex::Regex, text: &str) -> bool {
    re.is_match(text)
}

pub fn regex_scan(re: &regex::Regex, text: &str) -> Vec<String> {
    re.find_iter(text).map(|m| m.as_str().to_string()).collect()
}

pub fn regex_replace(re: &regex::Regex, text: &str, replacement: &str) -> String {
    re.replace(text, replacement).to_string()
}

pub fn regex_replace_all(re: &regex::Regex, text: &str, replacement: &str) -> String {
    re.replace_all(text, replacement).to_string()
}

// VM runtime entry points invoked from `Vm::run_inner` invoke handling.
pub(crate) fn dispatch_regex_instance(
    vm: &mut Vm,
    fields_ref: crate::gc::GcRef,
    method_name: &str,
    args: &[VmValue],
    line: u32,
) -> Result<VmValue, VmError> {
    let fields = vm.heap_fields_clone(fields_ref);
    let id = crate::native_regex::extract_id(&fields, line)?;
    let re = vm.regex(id).ok_or_else(|| VmError::TypeError {
        message: format!("regex id {} not found", id),
        line,
    })?;
    match method_name {
        "match?" => {
            let text = match args {
                [VmValue::Str(s)] => s.clone(),
                _ => {
                    return Err(VmError::TypeError {
                        message: "regex.match? expects a String".into(),
                        line,
                    });
                }
            };
            Ok(VmValue::Bool(crate::native_regex::regex_match_bool(
                re, &text,
            )))
        }
        "match" => {
            let text = match args {
                [VmValue::Str(s)] => s.clone(),
                _ => {
                    return Err(VmError::TypeError {
                        message: "regex.match expects a String".into(),
                        line,
                    });
                }
            };
            match re.captures(&text) {
                None => Ok(VmValue::Nil),
                Some(caps) => {
                    let full = caps.get(0).unwrap().as_str().to_string();
                    let start = caps.get(0).unwrap().start() as i64;
                    let end = caps.get(0).unwrap().end() as i64;
                    let capture_list: Vec<VmValue> = caps
                        .iter()
                        .skip(1)
                        .map(|m| match m {
                            Some(m) => VmValue::Str(m.as_str().to_string()),
                            None => VmValue::Nil,
                        })
                        .collect();
                    let methods = vm
                        .class_methods("Match")
                        .ok_or_else(|| VmError::TypeError {
                            message: "Regex.Match class not loaded".to_string(),
                            line,
                        })?;
                    let mut match_fields = HashMap::new();
                    match_fields.insert("full".to_string(), VmValue::Str(full));
                    match_fields.insert("captures".to_string(), vm.alloc_list(capture_list));
                    match_fields.insert("start".to_string(), VmValue::Int(start));
                    match_fields.insert("end_pos".to_string(), VmValue::Int(end));
                    let gc_fields = vm.alloc_fields(match_fields);
                    Ok(VmValue::Instance {
                        class_name: "Match".to_string(),
                        ancestor_chain: std::rc::Rc::new(
                            vm.class_ancestors("Match")
                                .unwrap_or_else(|| vec!["Match".to_string()]),
                        ),
                        fields: gc_fields,
                        methods,
                    })
                }
            }
        }
        "scan" => {
            let text = match args {
                [VmValue::Str(s)] => s.clone(),
                _ => {
                    return Err(VmError::TypeError {
                        message: "regex.scan expects a String".into(),
                        line,
                    });
                }
            };
            let matches = crate::native_regex::regex_scan(re, &text);
            let match_vals: Vec<VmValue> = matches.into_iter().map(VmValue::Str).collect();
            Ok(vm.alloc_list(match_vals))
        }
        "replace" => {
            let (text, replacement) = match args {
                [VmValue::Str(t), VmValue::Str(r)] => (t.clone(), r.clone()),
                _ => {
                    return Err(VmError::TypeError {
                        message: "regex.replace expects (String, String)".into(),
                        line,
                    });
                }
            };
            Ok(VmValue::Str(crate::native_regex::regex_replace(
                re,
                &text,
                &replacement,
            )))
        }
        "replace_all" => {
            let (text, replacement) = match args {
                [VmValue::Str(t), VmValue::Str(r)] => (t.clone(), r.clone()),
                _ => {
                    return Err(VmError::TypeError {
                        message: "regex.replace_all expects (String, String)".into(),
                        line,
                    });
                }
            };
            Ok(VmValue::Str(crate::native_regex::regex_replace_all(
                re,
                &text,
                &replacement,
            )))
        }
        _ => Err(VmError::TypeError {
            message: format!("Regex has no method '{}'", method_name),
            line,
        }),
    }
}
