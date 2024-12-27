// EBNF:
// CompUnit      ::= FuncDef;

// Decl          ::= ConstDecl | VarDecl;
// ConstDecl     ::= "const" BType ConstDef {"," ConstDef} ";";
// BType         ::= "int";
// ConstDef      ::= IDENT "=" ConstInitVal;
// ConstInitVal  ::= ConstExp;
// VarDecl       ::= BType VarDef {"," VarDef} ";";
// VarDef        ::= IDENT | IDENT "=" InitVal;
// InitVal       ::= Exp;

// FuncDef       ::= FuncType IDENT "(" ")" Block;
// FuncType      ::= "int";

// Block         ::= "{" {BlockItem} "}";
// BlockItem     ::= Decl | Stmt;
// Stmt          ::= OpenStmt | ClosedStmt;
// OpenStmt      ::= "if" "(" Exp ")" Stmt
//                 | "if" "(" Exp ")" ClosedStmt "else" OpenStmt;
// ClosedStmt    ::= SimpleStmt
//                 | "if" "(" Exp ")" ClosedStmt "else" ClosedStmt;
// SimpleStmt    ::= LVal "=" Exp ";"
//                 | [Exp] ";"
//                 | Block
//                 | "return" [Exp] ";";

// Exp           ::= LOrExp;
// LVal          ::= IDENT;
// PrimaryExp    ::= "(" Exp ")" | LVal | Number;
// Number        ::= INT_CONST;
// UnaryExp      ::= PrimaryExp | UnaryOp UnaryExp;
// UnaryOp       ::= "+" | "-" | "!";
// MulExp        ::= UnaryExp | MulExp ("*" | "/" | "%") UnaryExp;
// AddExp        ::= MulExp | AddExp ("+" | "-") MulExp;
// RelExp        ::= AddExp | RelExp ("<" | ">" | "<=" | ">=") AddExp;
// EqExp         ::= RelExp | EqExp ("==" | "!=") RelExp;
// LAndExp       ::= EqExp | LAndExp "&&" EqExp;
// LOrExp        ::= LAndExp | LOrExp "||" LAndExp;
// ConstExp      ::= Exp;

#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}

#[derive(Debug)]
pub enum Decl {
    ConstDecl(ConstDecl),
    VarDecl(VarDecl),
}

#[derive(Debug)]
pub struct ConstDecl {
    pub const_def_list: Vec<ConstDef>,
}

#[derive(Debug, Clone, Copy)]
pub enum BType {
    Int,
}

#[derive(Debug)]
pub struct ConstDef {
    pub b_type: BType,
    pub ident: String,
    pub const_init_val: Box<ConstInitVal>,
}

#[derive(Debug)]
pub struct ConstInitVal {
    pub const_exp: Box<ConstExp>,
}

#[derive(Debug)]
pub struct VarDecl {
    pub var_def_list: Vec<VarDef>,
}

#[derive(Debug)]
pub struct VarDef {
    pub b_type: BType,
    pub ident: String,
    pub init_val: Box<Option<InitVal>>,
}

#[derive(Debug)]
pub struct InitVal {
    pub exp: Box<Exp>,
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
    pub block_item_list: Vec<BlockItem>,
}

#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

#[derive(Debug)]
pub enum Stmt {
    OpenStmt(OpenStmt),
    ClosedStmt(ClosedStmt),
}

#[derive(Debug)]
pub enum OpenStmt {
    If(Box<Exp>, Box<Stmt>),
    IfElse(Box<Exp>, Box<ClosedStmt>, Box<OpenStmt>),
}

#[derive(Debug)]
pub enum ClosedStmt {
    SimpleStmt(SimpleStmt),
    IfElse(Box<Exp>, Box<ClosedStmt>, Box<ClosedStmt>),
}

#[derive(Debug)]
pub enum SimpleStmt {
    Assign(LVal, Box<Exp>),
    Exp(Box<Option<Exp>>),
    Block(Box<Block>),
    Return(Box<Option<Exp>>),
}

#[derive(Debug)]
pub struct Exp {
    pub l_or_exp: LOrExp,
}

#[derive(Debug)]
pub struct LVal {
    pub ident: String,
}

#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    LVal(LVal),
    Number(i32),
}

#[derive(Debug)]
pub enum UnaryExp {
    PrimaryExp(PrimaryExp),
    UnaryExp(UnaryOp, Box<UnaryExp>),
}

#[derive(Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
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
pub enum RelExp {
    AddExp(AddExp),
    Lt(Box<RelExp>, Box<AddExp>),
    Gt(Box<RelExp>, Box<AddExp>),
    Le(Box<RelExp>, Box<AddExp>),
    Ge(Box<RelExp>, Box<AddExp>),
}

#[derive(Debug)]
pub enum EqExp {
    RelExp(RelExp),
    Eq(Box<EqExp>, Box<RelExp>),
    Ne(Box<EqExp>, Box<RelExp>),
}

#[derive(Debug)]
pub enum LAndExp {
    EqExp(EqExp),
    And(Box<LAndExp>, Box<EqExp>),
}

#[derive(Debug)]
pub enum LOrExp {
    LAndExp(LAndExp),
    Or(Box<LOrExp>, Box<LAndExp>),
}

#[derive(Debug)]
pub struct ConstExp {
    pub exp: Box<Exp>,
}
