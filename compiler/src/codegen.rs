use crate::parser::Expr;

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => format!("{}", n),
            Expr::Identifier(name) => name.clone(),
            Expr::BinaryOp { op, left, right } => {
                let left_code = CodeGenerator::generate(left);
                let right_code = CodeGenerator::generate(right);
                format!("({} {} {})", left_code, op, right_code)
            }
        }
    }
}