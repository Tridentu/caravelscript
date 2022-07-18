

pub enum CaravelDirective {
    Language,
    Compiler,
    Interaction,
    ProjectStructure,
    License,
    Output
}


pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}


pub enum CaravelScriptNode {
    Int(i32),
    String(String),
    Directive {
        directive: CaravelDirective,
        expr: Box<CaravelScriptNode>
    },
    ArithmeticOp {
        op: Operation,
        lhs: Box<CaravelScriptNode>,
        rhs: Box<CaravelScriptNode>
    },
    Terms(Vec<CaravelScriptNode>)

}
