use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}

pub fn eval(expr: Expr) -> Value {
    match expr {
        ArithExpr(arith_expr) => IntValue(eval_arith_expr(arith_expr)),
        BoolExpr(bool_expr) => BoolValue(eval_bool_expr(bool_expr)),
    }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr {
        BinArithExpr { left, right, op } => {
            let left = eval_arith_expr(*left);
            let right = eval_arith_expr(*right);
            match op {
                AddOp => left + right,
                SubOp => left - right,
                MulOp => left * right,
                IntDivOp => left / right,
            }
        },
        IntLit(int_lit) => int_lit,
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr {
        ArithCmpExpr { left, right, op } => {
            let left = eval_arith_expr(*left);
            let right = eval_arith_expr(*right);
            match op {
                LtOp => left < right,
                LteOp => left <= right,
                GtOp => left > right,
                GteOp => left >= right,
                ArithEqOp => left == right,
                ArithNeqOp => left != right,
            }
        },
        BinBoolExpr { left, right, op } => {
            let left = eval_bool_expr(*left);
            let right = eval_bool_expr(*right);
            match op {
                AndOp => left && right,
                OrOp => left || right,
                BoolEqOp => left == right,
                BoolNeqOp => left != right,
            }
        },
        NotExpr(expr) => !eval_bool_expr(*expr),
        BoolLit(bool_lit) => bool_lit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_add() {
        let expr = ArithExpr(
            BinArithExpr
            {
                left: Box::new(ArithExpr::IntLit(1)),
                right: Box::new(ArithExpr::IntLit(1)),
                op: AddOp,
            }
        );
        let answer = IntValue(2);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_expr_sub() {
        let expr = ArithExpr(
            BinArithExpr
            {
                left: Box::new(ArithExpr::IntLit(1)),
                right: Box::new(ArithExpr::IntLit(1)),
                op: SubOp,
            }
        );
        let answer = IntValue(0);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_expr_mul() {
        let expr = ArithExpr(
            BinArithExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: MulOp,
            }
        );
        let answer = IntValue(4);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_expr_int_div() {
        let expr = ArithExpr(
            BinArithExpr
            {
                left: Box::new(ArithExpr::IntLit(4)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: IntDivOp,
            }
        );

        let answer = IntValue(2);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_expr_lt() {
        let expr = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(4)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: LtOp,
            }
        );

        let expr2 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(4)),
                op: LtOp,
            }
        );

        let answer = BoolValue(false);
        let answer2 = BoolValue(true);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
    }

    #[test]
    fn test_expr_lte() {
        let expr = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(4)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: LteOp,
            }
        );

        let expr2 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(4)),
                op: LteOp,
            }
        );

        let expr3 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: LteOp,
            }
        );

        let answer = BoolValue(false);
        let answer2 = BoolValue(true);
        let answer3 = BoolValue(true);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
        assert_eq!(eval(expr3), answer3);
    }

    #[test]
    fn test_expr_gt() {
        let expr = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(4)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: GtOp,
            }
        );

        let expr2 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(4)),
                op: GtOp,
            }
        );

        let answer = BoolValue(true);
        let answer2 = BoolValue(false);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
    }

    #[test]
    fn test_expr_gte() {
        let expr = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(4)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: GteOp,
            }
        );

        let expr2 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(4)),
                op: GteOp,
            }
        );

        let expr3 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: GteOp,
            }
        );

        let answer = BoolValue(true);
        let answer2 = BoolValue(false);
        let answer3 = BoolValue(true);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
        assert_eq!(eval(expr3), answer3);
    }

    #[test]
    fn test_expr_eq() {
        let expr = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: ArithEqOp,
            }
        );

        let expr2 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(4)),
                op: ArithEqOp,
            }
        );

        let answer = BoolValue(true);
        let answer2 = BoolValue(false);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
    }

    #[test]
    fn test_expr_neq() {
        let expr = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(2)),
                op: ArithNeqOp,
            }
        );

        let expr2 = BoolExpr(
            ArithCmpExpr
            {
                left: Box::new(ArithExpr::IntLit(2)),
                right: Box::new(ArithExpr::IntLit(4)),
                op: ArithNeqOp,
            }
        );

        let answer = BoolValue(false);
        let answer2 = BoolValue(true);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
    }

    #[test]
    fn test_expr_and() {
        let expr = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(true)),
                op: AndOp,
            }
        );

        let expr2 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(false)),
                op: AndOp,
            }
        );

        let expr3 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(false)),
                right: Box::new(BoolExpr::BoolLit(true)),
                op: AndOp,
            }
        );

        let expr4 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(false)),
                right: Box::new(BoolExpr::BoolLit(false)),
                op: AndOp,
            }
        );

        let answer = BoolValue(true);
        let answer2 = BoolValue(false);
        let answer3 = BoolValue(false);
        let answer4 = BoolValue(false);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
        assert_eq!(eval(expr3), answer3);
        assert_eq!(eval(expr4), answer4);
    }

    #[test]
    fn test_expr_or() {
        let expr = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(true)),
                op: OrOp,
            }
        );

        let expr2 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(false)),
                op: OrOp,
            }
        );

        let expr3 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(false)),
                right: Box::new(BoolExpr::BoolLit(true)),
                op: OrOp,
            }
        );

        let expr4 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(false)),
                right: Box::new(BoolExpr::BoolLit(false)),
                op: OrOp,
            }
        );

        let answer = BoolValue(true);
        let answer2 = BoolValue(true);
        let answer3 = BoolValue(true);
        let answer4 = BoolValue(false);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
        assert_eq!(eval(expr3), answer3);
        assert_eq!(eval(expr4), answer4);
    }

    #[test]
    fn test_expr_bool_eq() {
        let expr = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(true)),
                op: BoolEqOp,
            }
        );

        let expr2 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(false)),
                op: BoolEqOp,
            }
        );

        let answer = BoolValue(true);
        let answer2 = BoolValue(false);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
    }

    #[test]
    fn test_expr_bool_neq() {
        let expr = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(true)),
                op: BoolNeqOp,
            }
        );

        let expr2 = BoolExpr(
            BinBoolExpr
            {
                left: Box::new(BoolExpr::BoolLit(true)),
                right: Box::new(BoolExpr::BoolLit(false)),
                op: BoolNeqOp,
            }
        );

        let answer = BoolValue(false);
        let answer2 = BoolValue(true);

        assert_eq!(eval(expr), answer);
        assert_eq!(eval(expr2), answer2);
    }

    #[test]
    fn test_expr_bool_not() {
        let expr = BoolExpr(
            NotExpr(Box::new(BoolExpr::BoolLit(true)))
        );

        let answer = BoolValue(false);

        assert_eq!(eval(expr), answer);
    }

    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}

fn main() {}

// Path: laboratory_3\tests\main.rs
// Unit Test: cargo llvm-cov --html
// Makefile: cargo llvm-cov --html && start ./target/llvm-cov/html/index.html 
// Makefile: cargo llvm-cov --html && open ./target/llvm-cov/html/index.html