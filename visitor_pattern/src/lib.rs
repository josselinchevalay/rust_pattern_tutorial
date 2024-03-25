trait Node {
    fn accept(&self, visitor: &mut dyn Visitor); 
}

enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Val(i32) 
}

impl Node for Expression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        match self {
            Expression::Add(lhs, rhs) => visitor.visit_add(lhs, rhs),
            Expression::Sub(lhs, rhs) => visitor.visit_sub(lhs, rhs),
            Expression::Val(val) => visitor.visit_val(*val) 
        }
    }
}

trait Visitor {
    fn visit_add(&mut self, lhs: &Expression, rhs: &Expression);
    fn visit_sub(&mut self, lhs: &Expression, rhs: &Expression);
    fn visit_val(&mut self, val: i32); 
}

struct Evaluator {
    result: i32
}

impl Visitor for Evaluator {
    fn visit_add(&mut self, lhs: &Expression, rhs: &Expression) {
        lhs.accept(self);
        let lhs_res = self.result;
        rhs.accept(self);
        let rhs_res = self.result;
        self.result = lhs_res + rhs_res;
    }

    fn visit_sub(&mut self, lhs: &Expression, rhs: &Expression) {
        // ...
    }

    fn visit_val(&mut self, val: i32) {
        self.result = val; 
    }
}

struct Evaluator {
    result: i32
}

impl Visitor for Evaluator {
    fn visit_add(&mut self, lhs: &Expression, rhs: &Expression) {
        lhs.accept(self);
        let lhs_res = self.result;
        rhs.accept(self);
        let rhs_res = self.result;
        self.result = lhs_res + rhs_res;
    }

    fn visit_sub(&mut self, lhs: &Expression, rhs: &Expression) {
        // ...
    }

    fn visit_val(&mut self, val: i32) {
        self.result = val; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut eval = Evaluator { result: 0 };
        let expr = Expression::Add(
            Box::new(Expression::Val(2)), 
            Box::new(Expression::Sub(
                Box::new(Expression::Val(8)),
                Box::new(Expression::Val(3))
            ))
        );
        expr.accept(&mut eval);
        assert_eq!(eval.result, 7);
    }
}
