// EBNF:
// CompUnit    ::= FuncDef;

// FuncDef     ::= FuncType IDENT "(" ")" Block;
// FuncType    ::= "int";

// Block       ::= "{" Stmt "}";
// Stmt        ::= "return" Exp ";";

// Exp         ::= AddExp;
// PrimaryExp  ::= "(" Exp ")" | Number;
// Number      ::= INT_CONST;
// UnaryExp    ::= PrimaryExp | UnaryOp UnaryExp;
// UnaryOp     ::= "+" | "-" | "!";
// MulExp      ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
// AddExp      ::= MulExp | AddExp ("+" | "-") MulExp;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    pub ident: String,
    pub block: Block,
}

#[derive(Debug)]
pub enum FuncType {
    Int,
}

#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt,
}

#[derive(Debug)]
pub struct Stmt {
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Exp {
    pub add_exp: AddExp,
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(PrimaryExp),
    UnaryExp(UnaryOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    Number(i32),
}

#[derive(Debug)]
pub enum MulExp {
    UnaryExp(UnaryExp),
    Mul(Box<MulExp>, Box<UnaryExp>),
    Div(Box<MulExp>, Box<UnaryExp>),
    Mod(Box<MulExp>, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum AddExp {
    MulExp(MulExp),
    Add(Box<AddExp>, Box<MulExp>),
    Sub(Box<AddExp>, Box<MulExp>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}
