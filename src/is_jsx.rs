use swc_core::ecma::{
  ast::*,
  visit::{VisitMut, VisitMutWith},
};

pub struct IsJSXVisitor {
  is_jsx: bool,
}

impl IsJSXVisitor {
  pub fn test(node: &mut impl VisitMutWith<Self>) -> bool {
      let mut visitor = IsJSXVisitor { is_jsx: false };
      node.visit_mut_children_with(&mut visitor);
      visitor.is_jsx
  }
}

impl VisitMut for IsJSXVisitor {
  fn visit_mut_jsx_element(&mut self, el: &mut JSXElement) {
      el.visit_mut_children_with(self);
      self.is_jsx = true;
  }

  fn visit_mut_jsx_fragment(&mut self, el: &mut JSXFragment) {
      el.visit_mut_children_with(self);
      self.is_jsx = true;
  }
}
