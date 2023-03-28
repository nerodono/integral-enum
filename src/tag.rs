use proc_macro2::Span;
use syn::{token::Plus, BinOp, Expr, ExprBinary, ExprLit, Lit, LitInt};

pub struct Tag {
    pub current: Expr,
}

fn int(repr: &str) -> Expr {
    Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Int(LitInt::new(repr, Span::mixed_site())),
    })
}

fn add_one(to: Expr) -> Expr {
    Expr::Binary(ExprBinary {
        attrs: Vec::new(),
        left: to.into(),
        op: BinOp::Add(Plus([Span::mixed_site()])),
        right: int("1").into(),
    })
}

impl Default for Tag {
    fn default() -> Self {
        Self { current: int("0") }
    }
}

impl Tag {
    pub fn process(&mut self, expr: Option<&Expr>) -> Expr {
        match expr {
            Some(new_current) => {
                self.current = add_one(new_current.clone());
                new_current.clone()
            }

            None => {
                let prev = self.current.clone();
                self.current = add_one(self.current.clone());
                prev
            }
        }
    }
}
