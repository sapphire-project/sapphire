use sapphire::ast::Expr;
use sapphire::doc::extract_file_doc;
use sapphire::lexer::Lexer;
use sapphire::parser::Parser;

fn parse_stmts(src: &str) -> Vec<Expr> {
    let tokens = Lexer::new(src).scan_tokens();
    Parser::new(tokens).parse().expect("parse error")
}

#[test]
fn test_doc_generation() {
    let src = r#"
      type Alias = Int | String

      interface Runnable {
        def run() -> Nil
      }

      class Animal {
        attr name: String = "unknown"
        attr age: Int

        def walk(speed: Float = 1.0) -> Nil {
          # walk logic
        }

        self {
          def create(name: String) -> Animal {
            # class method
          }
        }
      }

      def top_level_fn[T](val: T) -> T {
        val
      }
    "#;

    let exprs = parse_stmts(src);
    let doc = extract_file_doc(&exprs);

    assert_eq!(doc.type_aliases.len(), 1);
    assert_eq!(doc.type_aliases[0].name, "Alias");
    assert_eq!(doc.type_aliases[0].type_expr, "Int | String");

    assert_eq!(doc.interfaces.len(), 1);
    assert_eq!(doc.interfaces[0].name, "Runnable");
    assert_eq!(doc.interfaces[0].methods.len(), 1);
    assert_eq!(doc.interfaces[0].methods[0].name, "run");
    assert_eq!(doc.interfaces[0].methods[0].return_type.as_deref(), Some("Nil"));

    assert_eq!(doc.classes.len(), 1);
    assert_eq!(doc.classes[0].name, "Animal");
    assert_eq!(doc.classes[0].fields.len(), 2);
    assert_eq!(doc.classes[0].fields[0].name, "name");
    assert_eq!(doc.classes[0].fields[0].type_ann.as_deref(), Some("String"));
    assert_eq!(doc.classes[0].fields[0].default.as_deref(), Some("\"unknown\""));
    assert_eq!(doc.classes[0].fields[1].name, "age");
    assert_eq!(doc.classes[0].fields[1].type_ann.as_deref(), Some("Int"));
    assert_eq!(doc.classes[0].fields[1].default, None);

    assert_eq!(doc.classes[0].methods.len(), 2);
    let walk_method = doc.classes[0].methods.iter().find(|m| m.name == "walk").unwrap();
    assert_eq!(walk_method.params.len(), 1);
    assert_eq!(walk_method.params[0].name, "speed");
    assert_eq!(walk_method.params[0].type_ann.as_deref(), Some("Float"));
    assert_eq!(walk_method.params[0].default.as_deref(), Some("1.0"));
    assert_eq!(walk_method.return_type.as_deref(), Some("Nil"));

    let create_method = doc.classes[0].methods.iter().find(|m| m.name == "create").unwrap();
    assert!(create_method.class_method);
    assert_eq!(create_method.params.len(), 1);
    assert_eq!(create_method.params[0].name, "name");
    assert_eq!(create_method.params[0].type_ann.as_deref(), Some("String"));
    assert_eq!(create_method.return_type.as_deref(), Some("Animal"));

    assert_eq!(doc.functions.len(), 1);
    assert_eq!(doc.functions[0].name, "top_level_fn");
    assert_eq!(doc.functions[0].type_params, vec!["T"]);
    assert_eq!(doc.functions[0].params.len(), 1);
    assert_eq!(doc.functions[0].params[0].name, "val");
    assert_eq!(doc.functions[0].params[0].type_ann.as_deref(), Some("T"));
    assert_eq!(doc.functions[0].return_type.as_deref(), Some("T"));
}

#[test]
fn test_doc_generation_excludes_test_classes() {
    let src = r#"
      class Widget {
        class NestedTest < Test {
          def test_nested() {}
        }
      }

      class WidgetTest < Test {
        def test_widget() {}
      }

      class OtherTestNamedClass {}
    "#;

    let exprs = parse_stmts(src);
    let doc = extract_file_doc(&exprs);

    assert_eq!(doc.classes.len(), 2);
    assert_eq!(doc.classes[0].name, "Widget");
    assert!(doc.classes[0].nested.is_empty());
    assert_eq!(doc.classes[1].name, "OtherTestNamedClass");
}
