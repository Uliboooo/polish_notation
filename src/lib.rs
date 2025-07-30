use std::ops::{Add, Div, Mul, Sub};

pub struct Expr<T: Add + Sub + Mul + Div + Sized> {
    op: Ope,
    values: (Box<Value<T>>, Box<Value<T>>),
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Sized> Expr<T> {
    pub fn eval(self) -> Box<T> {
        let v1 = match *self.values.0 {
            Value::Expr(expr) => *expr.eval(),
            Value::Value(v) => v,
        };
        let v2 = match *self.values.1 {
            Value::Expr(expr) => *expr.eval(),
            Value::Value(v) => v,
        };

        Box::new(match self.op {
            Ope::Add => v1.add(v2),
            Ope::Sub => v1.sub(v2),
            Ope::Mul => v1.mul(v2),
            Ope::Div => v1.div(v2),
        })
    }
}

pub enum Value<T: Add + Sub + Mul + Div + Sized> {
    Expr(Box<Expr<T>>),
    Value(T),
}

pub enum Ope {
    Add,
    Sub,
    Mul,
    Div,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// + 10 20
    #[test]
    fn eval_add() {
        let expr = Expr {
            op: Ope::Add,
            values: (Box::new(Value::Value(10)), Box::new(Value::Value(20))),
        };
        assert_eq!(*expr.eval(), 30);
    }

    /// - 20 10
    #[test]
    fn eval_sub() {
        let expr = Expr {
            op: Ope::Sub,
            values: (Box::new(Value::Value(20)), Box::new(Value::Value(10))),
        };
        assert_eq!(*expr.eval(), 10);
    }

    /// * 10 20
    #[test]
    fn eval_mul() {
        let expr = Expr {
            op: Ope::Mul,
            values: (Box::new(Value::Value(10)), Box::new(Value::Value(20))),
        };
        assert_eq!(*expr.eval(), 200);
    }

    /// / 20 10
    #[test]
    fn eval_div() {
        let expr = Expr {
            op: Ope::Div,
            values: (Box::new(Value::Value(20)), Box::new(Value::Value(10))),
        };
        assert_eq!(*expr.eval(), 2);
    }

    /// * + 1 2 + 3 4
    #[test]
    fn eval_nested() {
        let expr = Expr {
            op: Ope::Mul,
            values: (
                Box::new(Value::Expr(Box::new(Expr {
                    op: Ope::Add,
                    values: (Box::new(Value::Value(1)), Box::new(Value::Value(2))),
                }))),
                Box::new(Value::Expr(Box::new(Expr {
                    op: Ope::Add,
                    values: (Box::new(Value::Value(3)), Box::new(Value::Value(4))),
                }))),
            ),
        };
        assert_eq!(*expr.eval(), 21);
    }
}
