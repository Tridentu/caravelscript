


pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub enum Node {
    Int(i32),
    StringLiteral(String),
    NumExpr {
        op: Operation,
        lhs: Box<Node>,
        rhs: Box<Node>
    }
}

