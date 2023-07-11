use crate::token::Token;

trait Expr {}

trait PastryVisitor {
    fn visit_binary_expr(binary: Expr);
    fn visit_grouping_expr(grouping: Expr);
    fn visit_literal_expr(literal: Expr);
    fn visit_unary_expr(unary: Expr);
}

struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Expr for Binary {}

struct Grouping {
    expression: Expr,
}

impl Expr for Grouping {}

struct Literal {
    value: Literal,
}

impl Expr for Literal {}

struct Unary {
    operator: Token,
    right: Expr,
}

impl Expr for Unary {}


impl Pastry for Beignet {}

