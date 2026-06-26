use crate::ast::{Expr, FieldDef, MethodDef, ParamDef, TypeExpr};
use crate::token::TokenKind;
use crate::value::Value;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct FileDoc {
    pub functions: Vec<FunctionDoc>,
    pub classes: Vec<ClassDoc>,
    pub interfaces: Vec<InterfaceDoc>,
    pub type_aliases: Vec<TypeAliasDoc>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FunctionDoc {
    pub name: String,
    pub type_params: Vec<String>,
    pub params: Vec<ParamDoc>,
    pub return_type: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ParamDoc {
    pub name: String,
    pub type_ann: Option<String>,
    pub default: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ClassDoc {
    pub name: String,
    pub type_params: Vec<String>,
    pub superclass: Option<String>,
    pub is_abstract: bool,
    pub is_module: bool,
    pub includes: Vec<String>,
    pub fields: Vec<FieldDoc>,
    pub methods: Vec<MethodDoc>,
    pub nested: Vec<ClassDoc>,
    pub constants: Vec<ConstantDoc>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FieldDoc {
    pub name: String,
    pub type_ann: Option<String>,
    pub default: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MethodDoc {
    pub name: String,
    pub type_params: Vec<String>,
    pub params: Vec<ParamDoc>,
    pub return_type: Option<String>,
    pub private: bool,
    pub class_method: bool,
    pub is_abstract: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct InterfaceDoc {
    pub name: String,
    pub type_params: Vec<String>,
    pub methods: Vec<MethodDoc>,
}

#[derive(Serialize, Debug, Clone)]
pub struct TypeAliasDoc {
    pub name: String,
    pub type_expr: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ConstantDoc {
    pub name: String,
    pub value: String,
}

pub fn extract_file_doc(exprs: &[Expr]) -> FileDoc {
    let mut functions = Vec::new();
    let mut classes = Vec::new();
    let mut interfaces = Vec::new();
    let mut type_aliases = Vec::new();

    for expr in exprs {
        match expr {
            Expr::Function {
                name,
                type_params,
                params,
                return_type,
                ..
            } => {
                functions.push(FunctionDoc {
                    name: name.clone(),
                    type_params: type_params.clone(),
                    params: params.iter().map(extract_param).collect(),
                    return_type: return_type.as_ref().map(format_type_expr),
                });
            }
            Expr::Class {
                name,
                type_params,
                superclass,
                is_abstract,
                is_module,
                includes,
                fields,
                methods,
                nested,
                constants,
            } => {
                if !is_test_class(superclass.as_deref()) {
                    classes.push(extract_class(
                        name.clone(),
                        type_params.clone(),
                        superclass.as_deref(),
                        *is_abstract,
                        *is_module,
                        includes.clone(),
                        fields,
                        methods,
                        nested,
                        constants,
                    ));
                }
            }
            Expr::Interface {
                name,
                type_params,
                methods,
            } => {
                interfaces.push(InterfaceDoc {
                    name: name.clone(),
                    type_params: type_params.clone(),
                    methods: methods.iter().map(extract_method).collect(),
                });
            }
            Expr::TypeAlias { name, type_expr } => {
                type_aliases.push(TypeAliasDoc {
                    name: name.clone(),
                    type_expr: format_type_expr(type_expr),
                });
            }
            _ => {}
        }
    }

    FileDoc {
        functions,
        classes,
        interfaces,
        type_aliases,
    }
}

#[allow(clippy::too_many_arguments)]
fn extract_class(
    name: String,
    type_params: Vec<String>,
    superclass: Option<&Expr>,
    is_abstract: bool,
    is_module: bool,
    includes: Vec<String>,
    fields: &[FieldDef],
    methods: &[MethodDef],
    nested: &[Expr],
    constants: &[(String, Box<Expr>)],
) -> ClassDoc {
    let superclass_str = superclass.map(format_expr);
    let mut nested_classes = Vec::new();
    for n in nested {
        if let Expr::Class {
            name: n_name,
            type_params: n_type_params,
            superclass: n_superclass,
            is_abstract: n_is_abstract,
            is_module: n_is_module,
            includes: n_includes,
            fields: n_fields,
            methods: n_methods,
            nested: n_nested,
            constants: n_constants,
        } = n
            && !is_test_class(n_superclass.as_deref())
        {
            nested_classes.push(extract_class(
                n_name.clone(),
                n_type_params.clone(),
                n_superclass.as_deref(),
                *n_is_abstract,
                *n_is_module,
                n_includes.clone(),
                n_fields,
                n_methods,
                n_nested,
                n_constants,
            ));
        }
    }

    ClassDoc {
        name,
        type_params,
        superclass: superclass_str,
        is_abstract,
        is_module,
        includes,
        fields: fields.iter().map(extract_field).collect(),
        methods: methods.iter().map(extract_method).collect(),
        nested: nested_classes,
        constants: constants
            .iter()
            .map(|(cname, cexpr)| ConstantDoc {
                name: cname.clone(),
                value: format_expr(cexpr),
            })
            .collect(),
    }
}

fn is_test_class(superclass: Option<&Expr>) -> bool {
    superclass.is_some_and(|expr| matches!(expr, Expr::Variable(name) if name == "Test"))
}

fn extract_param(param: &ParamDef) -> ParamDoc {
    ParamDoc {
        name: param.name.clone(),
        type_ann: param.type_ann.as_ref().map(format_type_expr),
        default: param.default.as_ref().map(format_expr),
    }
}

fn extract_field(field: &FieldDef) -> FieldDoc {
    FieldDoc {
        name: field.name.clone(),
        type_ann: field.type_ann.as_ref().map(format_type_expr),
        default: field.default.as_ref().map(format_expr),
    }
}

fn extract_method(method: &MethodDef) -> MethodDoc {
    MethodDoc {
        name: method.name.clone(),
        type_params: method.type_params.clone(),
        params: method.params.iter().map(extract_param).collect(),
        return_type: method.return_type.as_ref().map(format_type_expr),
        private: method.private,
        class_method: method.class_method,
        is_abstract: method.is_abstract,
    }
}

fn format_type_expr(te: &TypeExpr) -> String {
    match te {
        TypeExpr::Named(n) => n.clone(),
        TypeExpr::Apply(n, args) => format!(
            "{}[{}]",
            n,
            args.iter()
                .map(format_type_expr)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        TypeExpr::Literal(Value::Int(n)) => n.to_string(),
        TypeExpr::Literal(Value::Float(n)) => n.to_string(),
        TypeExpr::Literal(Value::Str(s)) => format!("{:?}", s),
        TypeExpr::Literal(Value::Bool(b)) => b.to_string(),
        TypeExpr::Literal(Value::Nil) => "Nil".to_string(),
        TypeExpr::Union(arms) => arms
            .iter()
            .map(format_type_expr)
            .collect::<Vec<_>>()
            .join(" | "),
        TypeExpr::Any => "Any".to_string(),
    }
}

fn token_kind_to_str(kind: &TokenKind) -> &str {
    match kind {
        TokenKind::Plus => "+",
        TokenKind::Minus => "-",
        TokenKind::Star => "*",
        TokenKind::Slash => "/",
        TokenKind::Percent => "%",
        TokenKind::Bang => "!",
        TokenKind::Tilde => "~",
        TokenKind::Amp => "&",
        TokenKind::Pipe => "|",
        TokenKind::Caret => "^",
        TokenKind::Eq => "=",
        TokenKind::EqEq => "==",
        TokenKind::BangEq => "!=",
        TokenKind::Less => "<",
        TokenKind::LessEq => "<=",
        TokenKind::Greater => ">",
        TokenKind::GreaterEq => ">=",
        _ => "",
    }
}

fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Literal(val) => match val {
            Value::Str(s) => format!("{:?}", s),
            other => other.to_string(),
        },
        Expr::Variable(name) => name.clone(),
        Expr::ListLit(items) => format!(
            "[{}]",
            items.iter().map(format_expr).collect::<Vec<_>>().join(", ")
        ),
        Expr::MapLit(pairs) => {
            let p: Vec<_> = pairs
                .iter()
                .map(|(k, v)| format!("{}: {}", k, format_expr(v)))
                .collect();
            format!("{{{}}}", p.join(", "))
        }
        Expr::Call { callee, args, .. } => {
            let formatted_args: Vec<_> = args
                .iter()
                .map(|arg| {
                    if let Some(ref name) = arg.name {
                        format!("{}: {}", name, format_expr(&arg.value))
                    } else {
                        format_expr(&arg.value)
                    }
                })
                .collect();
            format!("{}({})", format_expr(callee), formatted_args.join(", "))
        }
        Expr::Get { object, name, .. } => format!("{}.{}", format_expr(object), name),
        Expr::SelfExpr => "self".to_string(),
        Expr::IVar { name } => name.clone(),
        Expr::Range { from, to } => format!("{}..{}", format_expr(from), format_expr(to)),
        Expr::Unary { op, right } => format!("{}{}", token_kind_to_str(&op.kind), format_expr(right)),
        _ => "...".to_string(),
    }
}
