use std::path::Path;

use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::{Ident, JSXOpeningElement, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXElementName, Lit, Str};
use swc_core::ecma::visit::VisitMut;
use swc_core::ecma::atoms::js_word;

#[derive(Default)]
pub struct AddClassnameVisitor<'a> {
    filename: &'a str,
}

impl<'a> AddClassnameVisitor<'a> {
    pub fn new(file_path: &'a str) -> Self {
        let path = Path::new(file_path);
        let filename: &str = path.file_stem().and_then(|stem| stem.to_str()).unwrap();

        AddClassnameVisitor {
            filename,
        }
    }

    fn class_name(&self, component_name: &str) -> String {

        format!(
            "{}-{}",
            self.camel_to_hyphen_case(self.filename),
            self.camel_to_hyphen_case(component_name)
        )
    }

    fn camel_to_hyphen_case(&self, camel_case: &str) -> String {
        let mut result: String = String::new();
        let mut prev_char_was_lowercase: bool = false;

        for (i, c) in camel_case.chars().enumerate() {
            if i > 0 && c.is_uppercase() {
                if prev_char_was_lowercase {
                    result.push('-');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else {
                result.push(c.to_lowercase().next().unwrap());
            }
            prev_char_was_lowercase = c.is_lowercase();
        }
        result
    }
}

impl<'a> VisitMut for AddClassnameVisitor<'a> {
    fn visit_mut_jsx_opening_element(&mut self, n: &mut JSXOpeningElement) {
        let component_name = match &n.name {
            JSXElementName::Ident(ident) => ident.sym.to_string(),
            JSXElementName::JSXMemberExpr(expr) => {
                // Adjust the pattern match to handle JSXExpr
                match (&*expr).prop {
                    // Expr::Ident(ident) => ident.sym.to_string(),
                    _ => "".to_string(),
                }
            }
            _ => return,
        };

        let class_name: String = self.class_name(&component_name);

        let has_class_name = n.attrs.iter_mut().any(|attr| match attr {
            JSXAttrOrSpread::JSXAttr(JSXAttr { name, value, .. }) => {
                if let JSXAttrName::Ident(ident) = name {
                    if ident.sym == js_word!("className") {
                        if let Some(JSXAttrValue::Lit(Lit::Str(existing_value))) = value.take() {
                            // Append to the existing className
                            let new_value = Lit::Str(Str {
                                span: DUMMY_SP,
                                value: format!("{} {}", existing_value.value, class_name).into(),
                                raw: None,
                            });
                            *value = Some(JSXAttrValue::Lit(new_value));
                        }
                        return true
                    }
                }
                false
            }
            _ => false,
        });

        if !has_class_name {
            n.attrs.push(
                JSXAttrOrSpread::JSXAttr(JSXAttr {
                    span: DUMMY_SP,
                    name: JSXAttrName::Ident(Ident::new(js_word!("className"), DUMMY_SP)),
                    value: Some(JSXAttrValue::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: class_name.into(),
                        raw: None,
                    }))),
                })
            );
        }
    }
}
