// Constant value which defines how many spaces the Node::display()
// function generates per indentation.
const DISPLAY_INDENTATION: usize = 4;

/// Represents the mathematical operations used in nodes suffixed with 'Op'
#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div
}

/// Every syntax tree object must implement the Node trait.
pub trait Node {
    /// Evaluate the node, producing a numerical output.
    fn evaluate(&self) -> f32;

    /// Display function should produce a string in the following format:
    /// 
    /// ```
    /// ObjectName {
    /// |-> attribute1: ChildObject {
    /// |-> |-> ...
    /// |-> }
    /// }
    /// ```
    /// 
    /// Where each `|-> ` is equal to `depth`+1.
    /// 
    /// Unless `depth` == 0, the first line should not have any indentation,
    /// as it is inlined with the parent display string.
    /// 
    fn display(&self, depth: usize) -> String;
}

/// Represents a binary operation, meaning it's a mathematical
/// operation with both a left and right side.
/// 
/// For example `1 + 1` is a binary operation.
/// It has a left and right hand side, with an operation in the middle.
pub struct BinOp {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub op: Op
}

impl Node for BinOp {
    fn evaluate(&self) -> f32 {
        // Simple map to rust native operations
        match self.op {
            Op::Add => self.left.evaluate() + self.right.evaluate(),
            Op::Sub => self.left.evaluate() - self.right.evaluate(),
            Op::Div => self.left.evaluate() / self.right.evaluate(),
            Op::Mult => self.left.evaluate() * self.right.evaluate(),
        }
    }

    fn display(&self, depth: usize) -> String {
        format!(
            "BinOp {{\n{1}left: {2}\n{1}right: {3}\n{1}op: {4:#?}\n{0}}}",
            " ".repeat(depth*DISPLAY_INDENTATION),
            " ".repeat((depth+1)*DISPLAY_INDENTATION),
            self.left.display(depth + 1), self.right.display(depth + 1), self.op
        )
    }
}

/// Represents a unary operation, meaning it's a mathematical
/// operation with just a right side.
/// 
/// The only meaningful operation is `-x` though `+x` is still
/// valid syntax, despite it not doing anything.
pub struct UnaryOp {
    pub right: Box<dyn Node>,
    pub op: Op
}

impl Node for UnaryOp {
    fn evaluate(&self) -> f32 {
        match self.op {
            Op::Add | Op::Mult | Op::Div 
            => self.right.evaluate(),
            Op::Sub => -self.right.evaluate()
        }
    }

    fn display(&self, depth: usize) -> String {
        format!(
            "UnaryOp {{\n{1}right: {2}\n{1}op: {3:#?}\n{0}}}",
            " ".repeat(depth*DISPLAY_INDENTATION),
            " ".repeat((depth+1)*DISPLAY_INDENTATION),
            self.right.display(depth + 1), self.op
        )
    }
}

/// Integer constants
/// 
/// e.g. `3` or `100`
pub struct IntLiteral {
    pub value: String
}

impl Node for IntLiteral {
    fn evaluate(&self) -> f32 {
        // TODO: See comment on FloatLiteral::evaluate()
        self.value.parse::<f32>().unwrap()
    }

    fn display(&self, depth: usize) -> String {
        format!(
            "IntLiteral {{\n{1}value: {2}\n{0}}}",
            " ".repeat(depth*DISPLAY_INDENTATION),
            " ".repeat((depth+1)*DISPLAY_INDENTATION),
            self.value
        )
    }
}

/// Decimal constants
/// 
/// e.g. `3.14` or `1.234`
pub struct FloatLiteral {
    pub value: String
}

impl Node for FloatLiteral {
    fn evaluate(&self) -> f32 {
        // TODO: Although the tokensier should produce values which sucesfully
        // parse everytime, it would still be good to do a check here rather
        // than panicking if it fails.
        self.value.parse::<f32>().unwrap()
    }

    fn display(&self, depth: usize) -> String {
        format!(
            "FloatLiteral {{\n{1}value: {2}\n{0}}}",
            " ".repeat(depth*DISPLAY_INDENTATION),
            " ".repeat((depth+1)*DISPLAY_INDENTATION),
            self.value
        )
    }
}