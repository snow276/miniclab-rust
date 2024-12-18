// EBNF:
// CompUnit    ::= FuncDef;

// FuncDef     ::= FuncType IDENT "(" ")" Block;
// FuncType    ::= "int";

// Block       ::= "{" Stmt "}";
// Stmt        ::= "return" Exp ";";

// Exp         ::= UnaryExp;
// PrimaryExp  ::= "(" Exp ")" | Number;
// Number      ::= INT_CONST;
// UnaryExp    ::= PrimaryExp | UnaryOp UnaryExp;
// UnaryOp     ::= "+" | "-" | "!";

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
    pub unary_exp: UnaryExp,
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
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}
