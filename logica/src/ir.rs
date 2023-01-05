#[derive(Clone, Debug)]
pub enum BooleanExpr {
    True,
    False,
    Variable(usize),
    Not(Box<BooleanExpr>),
    And(Box<BooleanExpr>, Box<BooleanExpr>),
    Or(Box<BooleanExpr>, Box<BooleanExpr>),
}
